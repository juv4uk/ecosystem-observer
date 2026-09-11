//! Equal surface spellings for ecosystem file extensions (owner 2026-09-10).
//!
//! ```text
//! .my   ↔ .мій
//! .wsm  ↔ .всм
//! .lisp ↔ .лісп
//! ```
//!
//! This is **file/surface spelling only**: same parser path, no new semantics,
//! no mass rename, Latin spellings stay first-class. Cyrillic is not a
//! transliteration target for path components in general — only these three
//! extension pairs are equated when resolving a readable contract/source file.

use std::path::{Path, PathBuf};

/// Latin extension (with leading dot) → Cyrillic twin, and the reverse.
pub fn alternate_extension(ext: &str) -> Option<&'static str> {
    match ext {
        "my" | ".my" => Some("мій"),
        "мій" | ".мій" => Some("my"),
        "wsm" | ".wsm" => Some("всм"),
        "всм" | ".всм" => Some("wsm"),
        "lisp" | ".lisp" => Some("лісп"),
        "лісп" | ".лісп" => Some("lisp"),
        _ => None,
    }
}

/// True if `path`'s extension is one of the three equal pairs (either side).
pub fn is_ecosystem_surface_ext(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| {
            matches!(
                e,
                "my" | "мій" | "wsm" | "всм" | "lisp" | "лісп"
            )
        })
}

/// If `preferred` exists, return it. Else try the Cyrillic/Latin twin
/// extension on the same stem. If neither exists, return `preferred`
/// (caller still sees Unavailable for a missing path — we do not invent).
pub fn resolve_existing_surface_path(preferred: &Path) -> PathBuf {
    if preferred.exists() {
        return preferred.to_path_buf();
    }
    if let Some(alt) = twin_path(preferred) {
        if alt.exists() {
            return alt;
        }
    }
    preferred.to_path_buf()
}

fn twin_path(path: &Path) -> Option<PathBuf> {
    let ext = path.extension()?.to_str()?;
    let twin_ext = alternate_extension(ext)?;
    Some(path.with_extension(twin_ext))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn alternate_pairs_are_symmetric() {
        assert_eq!(alternate_extension("my"), Some("мій"));
        assert_eq!(alternate_extension("мій"), Some("my"));
        assert_eq!(alternate_extension("wsm"), Some("всм"));
        assert_eq!(alternate_extension("всм"), Some("wsm"));
        assert_eq!(alternate_extension("lisp"), Some("лісп"));
        assert_eq!(alternate_extension("лісп"), Some("lisp"));
        assert_eq!(alternate_extension("json"), None);
    }

    #[test]
    fn surface_ext_recognizes_both_spellings() {
        assert!(is_ecosystem_surface_ext(Path::new("language-contract.my")));
        assert!(is_ecosystem_surface_ext(Path::new("language-contract.мій")));
        assert!(is_ecosystem_surface_ext(Path::new("target-contract.wsm")));
        assert!(is_ecosystem_surface_ext(Path::new("target-contract.всм")));
        assert!(is_ecosystem_surface_ext(Path::new("програма.лісп")));
        assert!(!is_ecosystem_surface_ext(Path::new("readme.md")));
    }

    #[test]
    fn resolve_prefers_existing_latin_then_cyrillic_twin() {
        let root = std::env::temp_dir().join(format!(
            "eco-obs-surface-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();

        let latin = root.join("guard-reference.wsm");
        let cyrillic = root.join("guard-reference.всм");

        // Neither exists → return preferred unchanged.
        assert_eq!(resolve_existing_surface_path(&latin), latin);

        // Only Cyrillic exists → resolve to it.
        fs::write(&cyrillic, "((kind . test))\n").unwrap();
        assert_eq!(resolve_existing_surface_path(&latin), cyrillic);

        // Latin exists → prefer Latin (no forced rename).
        fs::write(&latin, "((kind . test))\n").unwrap();
        assert_eq!(resolve_existing_surface_path(&latin), latin);

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn unicode_fixture_names_are_valid_paths() {
        // Acceptance: UTF-8 filenames are not rejected by the helper.
        let p = Path::new("приклад.мій");
        assert!(is_ecosystem_surface_ext(p));
        assert_eq!(alternate_extension("мій"), Some("my"));
        let p2 = Path::new("знання.всм");
        assert!(is_ecosystem_surface_ext(p2));
        let p3 = Path::new("програма.лісп");
        assert!(is_ecosystem_surface_ext(p3));
    }
}
