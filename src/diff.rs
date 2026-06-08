//! Catalog diffing — track ecosystem growth over time.

use crate::catalog::Catalog;
use serde::{Deserialize, Serialize};

/// Difference between two catalog snapshots.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatalogDiff {
    /// Repo names present in `new` but not in `old`.
    pub added_repos: Vec<String>,
    /// Repo names present in `old` but not in `new`.
    pub removed_repos: Vec<String>,
    /// Capability names present in `new` but not in `old`.
    pub added_capabilities: Vec<String>,
    /// Capability names present in `old` but not in `new`.
    pub removed_capabilities: Vec<String>,
}

impl CatalogDiff {
    /// Returns true if there are no differences.
    pub fn is_empty(&self) -> bool {
        self.added_repos.is_empty()
            && self.removed_repos.is_empty()
            && self.added_capabilities.is_empty()
            && self.removed_capabilities.is_empty()
    }

    /// Total number of changes.
    pub fn total_changes(&self) -> usize {
        self.added_repos.len()
            + self.removed_repos.len()
            + self.added_capabilities.len()
            + self.removed_capabilities.len()
    }
}

/// Compute the diff between two catalog snapshots.
pub fn diff_catalogs(old: &Catalog, new: &Catalog) -> CatalogDiff {
    let old_names: std::collections::HashSet<String> = old.repos().map(|r| r.name.clone()).collect();
    let new_names: std::collections::HashSet<String> = new.repos().map(|r| r.name.clone()).collect();

    let added_repos = new_names.difference(&old_names).cloned().collect();
    let removed_repos = old_names.difference(&new_names).cloned().collect();

    let old_caps = collect_capability_names(old);
    let new_caps = collect_capability_names(new);

    let added_capabilities = new_caps.difference(&old_caps).cloned().collect();
    let removed_capabilities = old_caps.difference(&new_caps).cloned().collect();

    CatalogDiff {
        added_repos,
        removed_repos,
        added_capabilities,
        removed_capabilities,
    }
}

fn collect_capability_names(cat: &Catalog) -> std::collections::HashSet<String> {
    cat.repos()
        .flat_map(|r| r.capabilities.iter().map(|c| c.name.clone()))
        .collect()
}
