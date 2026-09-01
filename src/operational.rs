//! Bounded read-only probes for Guard and swarm-node operational visibility.
//!
//! This module deliberately observes sources and live processes only. It does
//! not turn a reference topic into an architectural verdict, and it does not
//! turn a running process into delivery or mesh-convergence evidence.

use crate::sexpr::{self, Expr};
use crate::snapshot::{
    AgentProcess, GuardReferenceSnapshot, LegacyPathObservation, ObservationStatus,
    SwarmNodeInstance, SwarmNodeSnapshot,
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalSources {
    pub guard_reference_path: PathBuf,
    pub legacy_guard_paths: Vec<PathBuf>,
}

pub(crate) fn observe_guard_reference(sources: &OperationalSources) -> GuardReferenceSnapshot {
    let legacy_paths = sources
        .legacy_guard_paths
        .iter()
        .map(|path| LegacyPathObservation {
            path: path.display().to_string(),
            exists: path.exists(),
        })
        .collect();
    let source_path = sources.guard_reference_path.display().to_string();

    let text = match std::fs::read_to_string(&sources.guard_reference_path) {
        Ok(text) => text,
        Err(error) => {
            return GuardReferenceSnapshot {
                source_path,
                status: ObservationStatus::Unavailable,
                topics: Vec::new(),
                canonical_entry_point_present: None,
                legacy_paths,
                error: Some(error.to_string()),
            };
        }
    };

    match sexpr::parse(&text) {
        Ok(expressions) => {
            let mut topics = Vec::new();
            for expression in &expressions {
                collect_topics(expression, &mut topics);
            }
            topics.sort();
            topics.dedup();
            let canonical_entry_point_present = expressions
                .iter()
                .any(|expression| contains_def(expression, "guard-ask"));
            GuardReferenceSnapshot {
                source_path,
                status: ObservationStatus::Complete,
                topics,
                canonical_entry_point_present: Some(canonical_entry_point_present),
                legacy_paths,
                error: None,
            }
        }
        Err(error) => GuardReferenceSnapshot {
            source_path,
            status: ObservationStatus::Partial,
            topics: Vec::new(),
            canonical_entry_point_present: None,
            legacy_paths,
            error: Some(format!("Guard reference parse failed: {error:?}")),
        },
    }
}

pub(crate) fn observe_swarm_nodes(processes: &[AgentProcess]) -> SwarmNodeSnapshot {
    let mut live_instances: Vec<SwarmNodeInstance> = processes
        .iter()
        .filter_map(|process| {
            let observed = process.os_observed.as_ref()?;
            if !command_contains_swarm_node(&observed.command) {
                return None;
            }
            Some(SwarmNodeInstance {
                pid: process.pid,
                command: observed.command.clone(),
                cwd: observed.cwd.clone(),
                started_at_observed: observed.started_at_observed.clone(),
            })
        })
        .collect();
    live_instances.sort_by_key(|instance| instance.pid);

    SwarmNodeSnapshot {
        status: ObservationStatus::Complete,
        live_instances,
        note: "Process liveness only; delivery, peer acceptance, and mesh convergence are separate evidence"
            .to_string(),
    }
}

fn command_contains_swarm_node(command: &str) -> bool {
    command.split_whitespace().any(|part| {
        std::path::Path::new(part)
            .file_name()
            .and_then(|name| name.to_str())
            == Some("swarm-node")
    })
}

fn collect_topics(expression: &Expr, topics: &mut Vec<String>) {
    match expression {
        Expr::List(items) => {
            if let [Expr::Symbol(tag), Expr::Symbol(topic), ..] = items.as_slice() {
                if tag == "topic" {
                    topics.push(topic.clone());
                }
            }
            for item in items {
                collect_topics(item, topics);
            }
        }
        Expr::DottedList(items, tail) => {
            for item in items {
                collect_topics(item, topics);
            }
            collect_topics(tail, topics);
        }
        Expr::Symbol(_) | Expr::String(_) | Expr::Integer(_) => {}
    }
}

fn contains_def(expression: &Expr, name: &str) -> bool {
    match expression {
        Expr::List(items) => {
            matches!(items.as_slice(), [Expr::Symbol(def), Expr::Symbol(found), ..] if def == "def" && found == name)
                || items.iter().any(|item| contains_def(item, name))
        }
        Expr::DottedList(items, tail) => {
            items.iter().any(|item| contains_def(item, name)) || contains_def(tail, name)
        }
        Expr::Symbol(_) | Expr::String(_) | Expr::Integer(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::{IdentityStatus, OsObservedFacts, SelfReportedIdentity};

    #[test]
    fn guard_snapshot_reads_topics_entry_point_and_legacy_presence() {
        let root =
            std::env::temp_dir().join(format!("ecosystem-observer-guard-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let reference = root.join("guard-reference.wsm");
        let legacy = root.join("guard.py");
        std::fs::write(
            &reference,
            "(references (entry (topic task-freshness)) (entry (topic swarm-node-spawn-ownership)))\n(def guard-ask (lambda (name) name))\n",
        )
        .unwrap();
        std::fs::write(&legacy, "# legacy presence only\n").unwrap();

        let snapshot = observe_guard_reference(&OperationalSources {
            guard_reference_path: reference,
            legacy_guard_paths: vec![legacy],
        });

        assert_eq!(snapshot.status, ObservationStatus::Complete);
        assert_eq!(
            snapshot.topics,
            vec!["swarm-node-spawn-ownership", "task-freshness"]
        );
        assert_eq!(snapshot.canonical_entry_point_present, Some(true));
        assert!(snapshot.legacy_paths[0].exists);
        std::fs::remove_dir_all(root).ok();
    }

    #[test]
    fn missing_guard_reference_is_unavailable_not_empty_success() {
        let snapshot = observe_guard_reference(&OperationalSources {
            guard_reference_path: PathBuf::from("/definitely/missing/guard-reference.wsm"),
            legacy_guard_paths: Vec::new(),
        });
        assert_eq!(snapshot.status, ObservationStatus::Unavailable);
        assert!(snapshot.canonical_entry_point_present.is_none());
        assert!(snapshot.error.is_some());
    }

    #[test]
    fn swarm_snapshot_reports_process_liveness_without_convergence_claim() {
        let processes = vec![AgentProcess {
            pid: 42,
            os_observed: Some(OsObservedFacts {
                command: "/tmp/swarm-node --node-id test".to_string(),
                cwd: Some("/tmp".to_string()),
                started_at_observed: None,
                repo_association: None,
            }),
            identity_status: IdentityStatus::NotFound,
            identity: SelfReportedIdentity::default(),
        }];
        let snapshot = observe_swarm_nodes(&processes);
        assert_eq!(snapshot.live_instances.len(), 1);
        assert!(snapshot.note.contains("convergence"));
    }

    /// Optional live-source gate. CI has no sibling my-lisp checkout, so the
    /// path is supplied explicitly when an ecosystem integration witness is
    /// required instead of being hardcoded into the portable crate.
    #[test]
    fn configured_live_guard_reference_is_parseable() {
        let Some(path) = std::env::var_os("ECOSYSTEM_GUARD_REFERENCE") else {
            return;
        };
        let snapshot = observe_guard_reference(&OperationalSources {
            guard_reference_path: PathBuf::from(path),
            legacy_guard_paths: Vec::new(),
        });
        assert_eq!(snapshot.status, ObservationStatus::Complete);
        assert_eq!(snapshot.canonical_entry_point_present, Some(true));
        assert!(snapshot.topics.contains(&"task-freshness".to_string()));
        assert!(snapshot
            .topics
            .contains(&"swarm-node-spawn-ownership".to_string()));
    }
}
