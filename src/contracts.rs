//! Typed parsing for `language-contract.my`/`ecosystem-status.my`/`target-contract.wsm`,
//! Stage 1 §3 "Ecosystem contracts" plus light ABI-contract awareness.
//!
//! **Updated 2026-09-11:** language-contract fixtures at **6.0**; added
//! `parse_wsm_target_contract_version` for the neutral ABI repo
//! `wsm-target-contract` (`target-contract.wsm`, schema wsm-os-target-v1).
//!
//! Observer does not claim authority over ABI numbers — only reports
//! declared version when the file is readable.

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

fn as_symbol_or_string(e: &Expr) -> Option<&str> {
    e.as_symbol()
        .or_else(|| match e {
            Expr::String(s) => Some(s.as_str()),
            _ => None,
        })
}

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

/// Declared version of `wsm-target-contract` / `target-contract.wsm`.
/// Integer `version` field (currently 4), plus schema string when present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WsmTargetContractVersion {
    pub version: i64,
    pub schema: Option<String>,
    pub architecture: Option<String>,
}

impl std::fmt::Display for WsmTargetContractVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}", self.version)?;
        if let Some(ref s) = self.schema {
            write!(f, " ({s})")?;
        }
        Ok(())
    }
}

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

/// Parses `target-contract.wsm` from `wsm-target-contract`:
/// `((kind . wsm-os-target-contract) (schema . "wsm-os-target-v1") (version . 4) ...)`.
pub fn parse_wsm_target_contract_version(
    content: &str,
) -> Result<WsmTargetContractVersion, ContractError> {
    let exprs = parse(content)?;
    let root = exprs.first().ok_or(ContractError::MissingField("<root>"))?;
    let items = root
        .as_list()
        .ok_or(ContractError::WrongShape("<root> is not a list"))?;
    let version = alist_get(items, "version")
        .and_then(as_integer)
        .ok_or(ContractError::MissingField("version"))?;
    let schema = alist_get(items, "schema").and_then(as_symbol_or_string).map(str::to_string);
    let architecture = alist_get(items, "architecture")
        .and_then(as_symbol_or_string)
        .map(str::to_string);
    Ok(WsmTargetContractVersion {
        version,
        schema,
        architecture,
    })
}

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

    #[test]
    fn gate_detects_real_drift_case() {
        let language_contract_my = r#"
            ((major . 6) (minor . 0)
             (note . "RATIFIED by owner 2026-09-08.")
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

        assert!(drift.is_some());
        let drift = drift.unwrap();
        assert_eq!(drift.actual, ContractVersion { major: 6, minor: 0 });
        assert_eq!(drift.claimed, ContractVersion { major: 5, minor: 0 });
    }

    /// Real shape from wsm-target-contract/target-contract.wsm (version 4).
    #[test]
    fn parses_wsm_target_contract_version_4() {
        let content = r#"
            ((kind . wsm-os-target-contract)
             (schema . "wsm-os-target-v1")
             (version . 4)
             (architecture . x86_64)
             (endianness . little)
             (word . ((bits . 64) (tag-bits . 3)))
             (tags . ((cons . 0) (nil . 1) (true . 2) (boxed . 7)))
             (boxed . ((kinds-defined-so-far . (string game-handle)))))
        "#;
        let v = parse_wsm_target_contract_version(content).unwrap();
        assert_eq!(v.version, 4);
        assert_eq!(v.schema.as_deref(), Some("wsm-os-target-v1"));
        assert_eq!(v.architecture.as_deref(), Some("x86_64"));
    }

    #[test]
    fn wsm_target_missing_version_is_error() {
        let content = r#"((kind . wsm-os-target-contract) (schema . "wsm-os-target-v1"))"#;
        let err = parse_wsm_target_contract_version(content).unwrap_err();
        assert_eq!(err, ContractError::MissingField("version"));
    }
}
