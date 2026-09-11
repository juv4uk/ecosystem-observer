//! Typed parsing for `language-contract.my`/`ecosystem-status.my`, the
//! first concrete deliverable of Stage 1 acceptance criteria's §3
//! "Ecosystem contracts" (`ECO-DECISION-2026-08-19-TAURICODE-STAGE1-
//! OBSERVER`), built on the `sexpr` reader landed as its own prior slice.
//!
//! That section names one **mandatory acceptance gate**: Stage 1 must
//! independently reproduce, from the real files and without hardcoding
//! the specific case, a conclusion like "`language-contract.my` (my-lisp)
//! says one version while `ecosystem-status.my` still claims another".
//! The gate test uses a portable fixture grounded in real content shapes.
//!
//! **Updated 2026-09-11:** fixtures moved from the old 3.0/1.0 case to the
//! current ratified language-contract **6.0** (owner ratification 2026-09-08).
//! The drift gate still demonstrates detection of actual-vs-claimed mismatch.
//!
//! **Two alist conventions coexist in the ecosystem's own `.my` files,**
//! and this module only speaks the second one:
//! 1. `repo.my`'s space-separated tagged-list convention, e.g.
//!    `(role agent-workstation)` — already handled by `Expr::assoc`/
//!    `Expr::tagged_list` (a `List` whose head is the key symbol).
//! 2. The classic dotted-pair alist convention used by
//!    `language-contract.my`/`tasks.my`/`ecosystem-status.my`, e.g.
//!    `(major . 6)`, `(as-of . "2026-09-08")` — this is an `Expr::
//!    DottedList`, which `Expr::assoc` does **not** match (it calls
//!    `as_list()`, which only matches `Expr::List`). `alist_get` below is
//!    the dotted-pair counterpart.
//!
//! Out of scope here: `repo.my` typed parsing beyond existing helpers,
//! `isa-contract.my`, `compatibility.my`, full `wsm-target-contract`
//! parsing, wiring into `EcosystemSnapshot` (reader-only precedent).

use crate::sexpr::{parse, Expr, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub enum ContractError {
    Parse(ParseError),
    MissingField(&'static str),
    WrongShape(&'static str),
}

impl From<ParseError> for ContractError {
    fn from(e: ParseError) -> Self {
        ContractError::Parse(e)
    }
}

/// Dotted-pair alist lookup: finds `(key . value)` among `list`'s
/// elements and returns `value`. Distinct from `Expr::assoc`'s
/// space-list convention — see this module's doc comment.
fn alist_get<'a>(list: &'a [Expr], key: &str) -> Option<&'a Expr> {
    list.iter().find_map(|item| match item {
        Expr::DottedList(head, tail) if head.len() == 1 && head[0].as_symbol() == Some(key) => {
            Some(tail.as_ref())
        }
        _ => None,
    })
}

fn as_integer(e: &Expr) -> Option<i64> {
    match e {
        Expr::Integer(n) => Some(*n),
        _ => None,
    }
}

/// `(major, minor)` from `language-contract.my`'s own versioning axis —
/// Level 1 (core semantics) + Level 2 (language contract) only, per that
/// file's own doc comment; deliberately not Level 3 (ecosystem
/// conformance), which changes independently and far more often.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractVersion {
    pub major: i64,
    pub minor: i64,
}

impl std::fmt::Display for ContractVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Parses a `language-contract.my`-shaped file: a single top-level
/// dotted-pair alist with `(major . N)` and `(minor . N)` entries, e.g.
/// `((major . 6) (minor . 0) (note . "...") ...)`.
pub fn parse_language_contract_version(content: &str) -> Result<ContractVersion, ContractError> {
    let exprs = parse(content)?;
    let root = exprs.first().ok_or(ContractError::MissingField("<root>"))?;
    let items = root
        .as_list()
        .ok_or(ContractError::WrongShape("<root> is not a list"))?;
    let major = alist_get(items, "major")
        .and_then(as_integer)
        .ok_or(ContractError::MissingField("major"))?;
    let minor = alist_get(items, "minor")
        .and_then(as_integer)
        .ok_or(ContractError::MissingField("minor"))?;
    Ok(ContractVersion { major, minor })
}

/// Parses `ecosystem-status.my`'s claim about `repo_name`'s
/// `language-contract` version: `(repositories . ((<repo_name> .
/// ((language-contract . (MAJOR MINOR)) ...)) ...))`. This is the
/// **claimed** version — what the snapshot file says, not what
/// `language-contract.my` itself actually says.
pub fn parse_claimed_language_contract_version(
    ecosystem_status_content: &str,
    repo_name: &str,
) -> Result<ContractVersion, ContractError> {
    let exprs = parse(ecosystem_status_content)?;
    let root = exprs.first().ok_or(ContractError::MissingField("<root>"))?;
    let root_items = root
        .as_list()
        .ok_or(ContractError::WrongShape("<root> is not a list"))?;
    let repositories = alist_get(root_items, "repositories")
        .and_then(Expr::as_list)
        .ok_or(ContractError::MissingField("repositories"))?;
    let repo_entry = repositories
        .iter()
        .find_map(|item| match item {
            Expr::DottedList(head, tail)
                if head.len() == 1 && head[0].as_symbol() == Some(repo_name) =>
            {
                tail.as_list()
            }
            _ => None,
        })
        .ok_or(ContractError::MissingField("repositories/<repo_name>"))?;
    let claimed = alist_get(repo_entry, "language-contract")
        .and_then(Expr::as_list)
        .ok_or(ContractError::MissingField("language-contract"))?;
    let major = claimed
        .first()
        .and_then(as_integer)
        .ok_or(ContractError::MissingField("language-contract/major"))?;
    let minor = claimed
        .get(1)
        .and_then(as_integer)
        .ok_or(ContractError::MissingField("language-contract/minor"))?;
    Ok(ContractVersion { major, minor })
}

/// A drift between `language-contract.my`'s real version and what
/// `ecosystem-status.my` claims for the same repository. `None` from
/// `detect_language_contract_drift` means the two agree — not that no
/// comparison was made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDrift {
    pub repo: String,
    pub actual: ContractVersion,
    pub claimed: ContractVersion,
}

pub fn detect_language_contract_drift(
    repo: &str,
    actual: ContractVersion,
    claimed: ContractVersion,
) -> Option<ContractDrift> {
    if actual == claimed {
        None
    } else {
        Some(ContractDrift {
            repo: repo.to_string(),
            actual,
            claimed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_language_contract_version_6_0() {
        // Shape taken from the live my-lisp language-contract.my (6.0, 2026-09-08).
        let v = parse_language_contract_version(
            r#"((major . 6) (minor . 0) (note . "RATIFIED by owner 2026-09-08.") (covers . (G1 G2 G3 G4 G5 G6 G7 G8 S1 S2 S3)))"#,
        )
        .unwrap();
        assert_eq!(v, ContractVersion { major: 6, minor: 0 });
    }

    #[test]
    fn parses_claimed_version_from_ecosystem_status() {
        let content = r#"
            ((kind . ecosystem-status)
             (as-of . "2026-09-11")
             (repositories .
              ((my-lisp .
                ((role . semantic-source-of-truth)
                 (language-contract . (6 0))
                 (blocking-others . nil)))
               (fpga-lisp .
                ((role . hardware-synthesizer)
                 (language-contract . (6 0)))))))
        "#;
        let v = parse_claimed_language_contract_version(content, "my-lisp").unwrap();
        assert_eq!(v, ContractVersion { major: 6, minor: 0 });
        let v2 = parse_claimed_language_contract_version(content, "fpga-lisp").unwrap();
        assert_eq!(v2, ContractVersion { major: 6, minor: 0 });
    }

    #[test]
    fn claimed_version_missing_repo_is_a_contract_error_not_a_panic() {
        let content = r#"((repositories . ((my-lisp . ((language-contract . (6 0)))))))"#;
        let err = parse_claimed_language_contract_version(content, "no-such-repo").unwrap_err();
        assert_eq!(err, ContractError::MissingField("repositories/<repo_name>"));
    }

    #[test]
    fn no_drift_when_versions_agree() {
        let v = ContractVersion { major: 6, minor: 0 };
        assert_eq!(detect_language_contract_drift("my-lisp", v, v), None);
    }

    #[test]
    fn drift_reported_when_versions_disagree() {
        let actual = ContractVersion { major: 6, minor: 0 };
        let claimed = ContractVersion { major: 5, minor: 0 };
        let drift = detect_language_contract_drift("my-lisp", actual, claimed).unwrap();
        assert_eq!(drift.repo, "my-lisp");
        assert_eq!(drift.actual, actual);
        assert_eq!(drift.claimed, claimed);
    }

    /// Acceptance gate: independently detect actual-vs-claimed mismatch.
    /// Fixture uses current 6.0 shape for actual, and a deliberately stale
    /// claim (5.0) so the gate still exercises drift detection without
    /// depending on a machine-local path or a currently-live ecosystem-status
    /// file that may already have been updated.
    #[test]
    fn gate_detects_real_drift_case() {
        let language_contract_my = r#"
            ((major . 6) (minor . 0)
             (note . "RATIFIED by owner 2026-09-08. Contract 6.0 makes Canon 0+7 program resolution genuinely immutable.")
             (covers . (G1 G2 G3 G4 G5 G6 G7 G8 S1 S2 S3)))
        "#;
        let ecosystem_status_my = r#"
            ((kind . ecosystem-status)
             (as-of . "2026-09-07")
             (repositories .
              ((my-lisp .
                ((role . semantic-source-of-truth)
                 (language-contract . (5 0))
                 (exactness-model . fully-implemented))))))
        "#;

        let actual = parse_language_contract_version(language_contract_my).unwrap();
        let claimed =
            parse_claimed_language_contract_version(ecosystem_status_my, "my-lisp").unwrap();
        let drift = detect_language_contract_drift("my-lisp", actual, claimed);

        assert!(
            drift.is_some(),
            "acceptance gate failed: expected language-contract.my (6.0) vs \
             ecosystem-status.my's claim (5.0) to be detected as drift"
        );
        let drift = drift.unwrap();
        assert_eq!(drift.actual, ContractVersion { major: 6, minor: 0 });
        assert_eq!(drift.claimed, ContractVersion { major: 5, minor: 0 });
    }
}
