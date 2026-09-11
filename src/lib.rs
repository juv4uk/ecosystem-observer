//! `ecosystem-observer` — observer core for the my-lisp ecosystem.
//!
//! Slice 1: repository discovery + read-only git scanning.
//! Slice 2: local runtime observation + agent identity contract.
//! `sexpr` / `contracts`: S-expression reader and typed contract parsing
//! (language-contract **6.0**, `wsm-target-contract` v4).
//! `surface_ext` (2026-09-11): equal `.my`/`.мій`, `.wsm`/`.всм`, `.lisp`/`.лісп`.
//!
//! Every operation is read-only.

mod contracts;
mod discover;
mod git_read;
mod identity_contract;
mod operational;
mod process_observe;
mod sexpr;
mod snapshot;
mod surface_ext;
mod time_util;

pub use contracts::{
    detect_language_contract_drift, parse_claimed_language_contract_version,
    parse_language_contract_version, parse_wsm_target_contract_version, ContractDrift,
    ContractError, ContractVersion, WsmTargetContractVersion,
};
pub use discover::{discover_ecosystem, scan_repository, DiscoverInput};
pub use operational::OperationalSources;
pub use process_observe::{read_start_token, OsProcess};
pub use sexpr::{parse as parse_sexpr, Expr as SexprExpr, ParseError as SexprParseError};
pub use snapshot::{
    AgentProcess, EcosystemSnapshot, GitState, GuardReferenceSnapshot, IdentityStatus,
    LegacyPathObservation, ObservationStatus, OsObservedFacts, ProbeFailure, RemoteInfo,
    RepositorySnapshot, ScanMetadata, ScanStatus, SelfReportedIdentity, SwarmNodeInstance,
    SwarmNodeSnapshot,
};
pub use surface_ext::{
    alternate_extension, is_ecosystem_surface_ext, resolve_existing_surface_path,
};
pub use time_util::{iso8601_from_unix_seconds, iso8601_now};
