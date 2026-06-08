//! # si-catalog
//!
//! Unified capability catalog for SuperInstance — search, cross-reference,
//! and discover fleet capabilities.
//!
//! ## Quick Start
//!
//! ```
//! use si_catalog::registry::default_catalog;
//!
//! let catalog = default_catalog();
//! println!("Repos: {}", catalog.stats().total_repos);
//! ```

pub mod types;
pub mod catalog;
pub mod registry;
pub mod diff;
pub mod export;

pub use types::{Capability, RepoEntry, CatalogStats, CrossRef};
pub use catalog::Catalog;
pub use diff::CatalogDiff;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::default_catalog;

    // ── Catalog basics ─────────────────────────────────────────
    #[test]
    fn new_catalog_is_empty() {
        let cat = Catalog::new();
        assert!(cat.is_empty());
        assert_eq!(cat.len(), 0);
    }

    #[test]
    fn insert_and_get() {
        let mut cat = Catalog::new();
        let entry = RepoEntry::new("test-repo", "rust", vec![], vec![]);
        cat.insert(entry);
        assert_eq!(cat.len(), 1);
        assert!(cat.get("test-repo").is_some());
        assert!(cat.get("TEST-REPO").is_some()); // case-insensitive
        assert!(cat.get("nonexistent").is_none());
    }

    #[test]
    fn insert_replaces_existing() {
        let mut cat = Catalog::new();
        cat.insert(RepoEntry::new("dup", "rust", vec![], vec![]));
        cat.insert(RepoEntry::new("dup", "python", vec![], vec![]));
        assert_eq!(cat.len(), 1);
        assert_eq!(cat.get("dup").unwrap().language, "python");
    }

    // ── Search ─────────────────────────────────────────────────
    #[test]
    fn search_finds_substring() {
        let cat = default_catalog();
        let results = cat.search("math");
        assert!(results.iter().any(|r| r.name == "si-math"));
        assert!(results.iter().any(|r| r.name == "si-math-gpu"));
    }

    #[test]
    fn search_case_insensitive() {
        let cat = default_catalog();
        let results = cat.search("FLEET");
        assert!(!results.is_empty());
    }

    #[test]
    fn search_no_results() {
        let cat = default_catalog();
        let results = cat.search("nonexistent-xyz");
        assert!(results.is_empty());
    }

    // ── By capability ──────────────────────────────────────────
    #[test]
    fn by_capability_finds_provider() {
        let cat = default_catalog();
        let results = cat.by_capability("linear-algebra");
        assert!(results.iter().any(|r| r.name == "si-math"));
    }

    #[test]
    fn by_capability_case_insensitive() {
        let cat = default_catalog();
        let results = cat.by_capability("Linear-Algebra");
        assert!(!results.is_empty());
    }

    #[test]
    fn by_capability_unknown() {
        let cat = default_catalog();
        assert!(cat.by_capability("nonexistent").is_empty());
    }

    // ── By category ────────────────────────────────────────────
    #[test]
    fn by_category_math() {
        let cat = default_catalog();
        let results = cat.by_category("math");
        assert!(results.len() >= 3); // si-math, si-math-gpu, si-signal
    }

    #[test]
    fn by_category_conservation() {
        let cat = default_catalog();
        let results = cat.by_category("conservation");
        assert!(results.len() >= 5);
    }

    #[test]
    fn by_category_unknown() {
        let cat = default_catalog();
        assert!(cat.by_category("nonexistent").is_empty());
    }

    // ── By language ────────────────────────────────────────────
    #[test]
    fn by_language_rust() {
        let cat = default_catalog();
        let results = cat.by_language("rust");
        assert!(results.len() >= 20);
    }

    #[test]
    fn by_language_python() {
        let cat = default_catalog();
        let results = cat.by_language("python");
        assert!(results.len() >= 3);
    }

    #[test]
    fn by_language_typescript() {
        let cat = default_catalog();
        let results = cat.by_language("typescript");
        assert!(results.len() >= 2);
    }

    #[test]
    fn by_language_unknown() {
        let cat = default_catalog();
        assert!(cat.by_language("cobol").is_empty());
    }

    // ── Cross-reference ────────────────────────────────────────
    #[test]
    fn cross_ref_linear_algebra() {
        let cat = default_catalog();
        let xr = cat.cross_reference("linear-algebra");
        assert!(xr.providers.contains(&"si-math".to_string()));
        // Dependencies are repo names, not capability names, so dependents may be empty
    }

    #[test]
    fn cross_ref_repo_dependency() {
        let cat = default_catalog();
        // si-math-gpu depends on "si-math" as a repo dependency
        let xr = cat.cross_reference("si-math");
        assert!(xr.dependents.contains(&"si-math-gpu".to_string()));
        assert!(!xr.is_leaf());
    }

    #[test]
    fn cross_ref_unknown_capability() {
        let cat = default_catalog();
        let xr = cat.cross_reference("nonexistent");
        assert!(xr.providers.is_empty());
        assert!(xr.is_root());
    }

    // ── Stats ──────────────────────────────────────────────────
    #[test]
    fn stats_match_catalog() {
        let cat = default_catalog();
        let stats = cat.stats();
        assert_eq!(stats.total_repos, cat.len());
        assert!(stats.total_capabilities > 0);
        assert!(stats.by_category.contains_key("math"));
        assert!(stats.by_category.contains_key("conservation"));
        assert!(stats.by_language.contains_key("rust"));
        assert!(stats.by_language.contains_key("python"));
    }

    #[test]
    fn stats_empty_catalog() {
        let cat = Catalog::new();
        let stats = cat.stats();
        assert_eq!(stats.total_repos, 0);
        assert_eq!(stats.total_capabilities, 0);
    }

    // ── Diff ───────────────────────────────────────────────────
    #[test]
    fn diff_identical_catalogs() {
        let cat = default_catalog();
        let diff = crate::diff::diff_catalogs(&cat, &cat);
        assert!(diff.is_empty());
        assert_eq!(diff.total_changes(), 0);
    }

    #[test]
    fn diff_added_repo() {
        let mut old = Catalog::new();
        let mut new = Catalog::new();
        old.insert(RepoEntry::new("a", "rust", vec![], vec![]));
        new.insert(RepoEntry::new("a", "rust", vec![], vec![]));
        new.insert(RepoEntry::new("b", "rust", vec![], vec![]));
        let diff = crate::diff::diff_catalogs(&old, &new);
        assert!(diff.added_repos.contains(&"b".to_string()));
        assert!(diff.removed_repos.is_empty());
    }

    #[test]
    fn diff_removed_repo() {
        let mut old = Catalog::new();
        let mut new = Catalog::new();
        old.insert(RepoEntry::new("a", "rust", vec![], vec![]));
        old.insert(RepoEntry::new("b", "rust", vec![], vec![]));
        new.insert(RepoEntry::new("a", "rust", vec![], vec![]));
        let diff = crate::diff::diff_catalogs(&old, &new);
        assert!(diff.removed_repos.contains(&"b".to_string()));
        assert!(diff.added_repos.is_empty());
    }

    #[test]
    fn diff_added_capability() {
        let mut old = Catalog::new();
        let mut new = Catalog::new();
        old.insert(RepoEntry::new("a", "rust", vec![], vec![]));
        new.insert(RepoEntry::new("a", "rust",
            vec![Capability::new("cap-1", "math", "lib", "test", "a")], vec![]));
        let diff = crate::diff::diff_catalogs(&old, &new);
        assert!(diff.added_capabilities.contains(&"cap-1".to_string()));
    }

    #[test]
    fn diff_removed_capability() {
        let cap = Capability::new("cap-1", "math", "lib", "test", "a");
        let mut old = Catalog::new();
        let mut new = Catalog::new();
        old.insert(RepoEntry::new("a", "rust", vec![cap], vec![]));
        new.insert(RepoEntry::new("a", "rust", vec![], vec![]));
        let diff = crate::diff::diff_catalogs(&old, &new);
        assert!(diff.removed_capabilities.contains(&"cap-1".to_string()));
    }

    // ── RepoEntry helpers ──────────────────────────────────────
    #[test]
    fn repo_entry_has_capability() {
        let repo = RepoEntry::new(
            "test",
            "rust",
            vec![Capability::new("foo", "math", "lib", "desc", "test")],
            vec![],
        );
        assert!(repo.has_capability("foo"));
        assert!(!repo.has_capability("bar"));
 }

    #[test]
    fn repo_entry_get_capability() {
        let repo = RepoEntry::new(
            "test",
            "rust",
            vec![Capability::new("foo", "math", "lib", "desc", "test")],
            vec![],
        );
        assert!(repo.get_capability("foo").is_some());
        assert!(repo.get_capability("bar").is_none());
    }

    #[test]
    fn repo_entry_categories() {
        let repo = RepoEntry::new(
            "test",
            "rust",
            vec![
                Capability::new("a", "math", "lib", "", "test"),
                Capability::new("b", "math", "lib", "", "test"),
                Capability::new("c", "fleet", "lib", "", "test"),
            ],
            vec![],
        );
        let cats = repo.categories();
        assert_eq!(cats, vec!["fleet", "math"]);
    }

    // ── Export integration ─────────────────────────────────────
    #[test]
    fn export_markdown_has_all_repos() {
        let cat = default_catalog();
        let md = crate::export::export_markdown(&cat);
        for repo in cat.repos() {
            assert!(md.contains(&repo.name), "markdown missing {}", repo.name);
        }
    }

    #[test]
    fn export_dot_has_all_edges() {
        let cat = default_catalog();
        let dot = crate::export::export_dot(&cat);
        assert!(dot.contains("\"si-math-gpu\" -> \"si-math\""));
    }

    #[test]
    fn export_supabase_sql_escapes() {
        let mut cat = Catalog::new();
        cat.insert(RepoEntry::new(
            "test's repo",
            "rust",
            vec![Capability::new("it's cap", "math", "lib", "desc's", "test's repo")],
            vec![],
        ));
        let sql = crate::export::export_supabase_seed(&cat);
        // Single quotes should be escaped
        assert!(!sql.contains("it's cap")); // unescaped
        assert!(sql.contains("it''s cap")); // escaped
    }

    // ── Capability construction ────────────────────────────────
    #[test]
    fn capability_new() {
        let cap = Capability::new("test", "math", "lib", "description", "repo");
        assert_eq!(cap.name, "test");
        assert_eq!(cap.category, "math");
        assert_eq!(cap.provides, "lib");
        assert_eq!(cap.description, "description");
        assert_eq!(cap.repo, "repo");
    }

    // ── Serialization roundtrip ────────────────────────────────
    #[test]
    fn repo_entry_json_roundtrip() {
        let entry = RepoEntry::new(
            "test",
            "rust",
            vec![Capability::new("cap", "math", "lib", "desc", "test")],
            vec!["dep".into()],
        );
        let json = serde_json::to_string(&entry).unwrap();
        let parsed: RepoEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, parsed);
    }
}

