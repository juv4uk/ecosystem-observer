use ecosystem_observer::{
    discover_ecosystem, DiscoverInput, EcosystemSnapshot, OperationalSources,
};
use std::path::{Path, PathBuf};

/// Default scan list when `ECOSYSTEM_REPOS` is unset.
/// Knowledge list aligned with docs/known-repos.md (2026-09-11).
/// Not an authority claim — only which local checkouts the desktop
/// probes by default. Override with ECOSYSTEM_REPOS or leave empty
/// and use full directory discovery via ECOSYSTEM_ROOT + empty override
/// is not available here; callers who want full scan must pass names
/// or we keep an explicit critical set so the UI does not silently
/// ignore first-class repos.
const DEFAULT_REPOSITORIES: [&str; 18] = [
    // observer + language core
    "ecosystem-observer",
    "my-lisp",
    "fpga-lisp",
    "cml",
    // ABI / self-hosted WSM
    "wsm-target-contract",
    "wsm-my-lisp",
    "wsm-os-lisp",
    "wsm-os",
    "wsm",
    // product surface
    "my-lisp-cyberpunk",
    "my-idea",
    "chess-lisp-zero",
    // foundation / research
    "pravda",
    "my-lisp-panini",
    "shiva-sutras",
    "mccarthy-eval",
    // legacy / optional still present on many machines
    "tauricode",
    "radio-log",
];

fn home_dir(home: Option<&str>) -> PathBuf {
    home.filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn resolve_root(raw: Option<&str>, home: Option<&str>) -> PathBuf {
    raw.filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| home_dir(home).join("GitHub"))
}

fn parse_repositories(raw: Option<&str>) -> Vec<String> {
    let parsed: Vec<String> = raw
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(String::from)
        .collect();
    if parsed.is_empty() {
        DEFAULT_REPOSITORIES
            .iter()
            .map(|name| name.to_string())
            .collect()
    } else {
        parsed
    }
}

fn operational_sources(root: &Path, home: &Path) -> OperationalSources {
    let coordination_root = std::env::var("ECOSYSTEM_COORDINATION_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home.join("ecosystem"));
    let guard_reference_path = std::env::var("ECOSYSTEM_GUARD_REFERENCE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("my-lisp/knowledge/guard-reference.wsm"));

    OperationalSources {
        guard_reference_path,
        legacy_guard_paths: vec![
            coordination_root.join("guard/guard.py"),
            coordination_root.join("guard/constitution.rules.my"),
        ],
    }
}

fn discover_input() -> DiscoverInput {
    let raw_home = std::env::var("HOME").ok();
    let home = home_dir(raw_home.as_deref());
    let root = resolve_root(
        std::env::var("ECOSYSTEM_ROOT").ok().as_deref(),
        raw_home.as_deref(),
    );
    let repositories = parse_repositories(std::env::var("ECOSYSTEM_REPOS").ok().as_deref());
    DiscoverInput {
        operational_sources: Some(operational_sources(&root, &home)),
        root,
        repositories: Some(repositories),
        identity_base_dir: None,
    }
}

#[tauri::command]
pub async fn get_ecosystem_snapshot() -> Result<EcosystemSnapshot, String> {
    tauri::async_runtime::spawn_blocking(|| discover_ecosystem(discover_input()))
        .await
        .map_err(|error| format!("observer scan task failed: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_defaults_below_home_without_machine_specific_path() {
        assert_eq!(
            resolve_root(None, Some("/tmp/person")),
            PathBuf::from("/tmp/person/GitHub")
        );
    }

    #[test]
    fn explicit_root_wins() {
        assert_eq!(
            resolve_root(Some("/srv/repos"), Some("/tmp/person")),
            PathBuf::from("/srv/repos")
        );
    }

    #[test]
    fn repository_override_is_trimmed_and_bounded_by_input() {
        assert_eq!(
            parse_repositories(Some("my-lisp, cml ,,wsm-os")),
            vec!["my-lisp", "cml", "wsm-os"]
        );
    }

    #[test]
    fn empty_repository_override_uses_declared_defaults() {
        let repositories = parse_repositories(Some(" , "));
        assert!(repositories.contains(&"ecosystem-observer".to_string()));
        assert!(repositories.contains(&"my-lisp".to_string()));
    }

    /// 2026-09-11: defaults must not silently drop first-class repos
    /// from the post-surge ecosystem (cyberpunk surface, ABI contract,
    /// self-hosted WSM line).
    #[test]
    fn defaults_include_post_surge_critical_repos() {
        let repositories = parse_repositories(None);
        for name in [
            "my-lisp-cyberpunk",
            "wsm-target-contract",
            "wsm-my-lisp",
            "wsm-os-lisp",
            "pravda",
            "cml",
            "fpga-lisp",
        ] {
            assert!(
                repositories.iter().any(|r| r == name),
                "DEFAULT_REPOSITORIES missing critical repo: {name}"
            );
        }
    }
}
