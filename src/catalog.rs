//! Main catalog data structure with search and cross-referencing.

use crate::types::{CatalogStats, CrossRef, RepoEntry};
use std::collections::HashMap;

/// The unified capability catalog.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    repos: Vec<RepoEntry>,
    /// Index: lowercase repo name → index in `repos`.
    name_index: HashMap<String, usize>,
}

impl Catalog {
    /// Create an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a repo entry. If a repo with the same name already exists it is replaced.
    pub fn insert(&mut self, entry: RepoEntry) {
        let key = entry.name.to_lowercase();
        if let Some(&idx) = self.name_index.get(&key) {
            self.repos[idx] = entry;
        } else {
            let idx = self.repos.len();
            self.repos.push(entry);
            self.name_index.insert(key, idx);
        }
    }

    /// Get a repo by exact name (case-insensitive).
    pub fn get(&self, name: &str) -> Option<&RepoEntry> {
        self.name_index.get(&name.to_lowercase()).map(|&i| &self.repos[i])
    }

    /// Iterate over all repos.
    pub fn repos(&self) -> impl Iterator<Item = &RepoEntry> {
        self.repos.iter()
    }

    /// Number of repos.
    pub fn len(&self) -> usize {
        self.repos.len()
    }

    /// Is the catalog empty?
    pub fn is_empty(&self) -> bool {
        self.repos.is_empty()
    }

    /// Fuzzy search by repo name — case-insensitive substring match.
    pub fn search(&self, query: &str) -> Vec<&RepoEntry> {
        let q = query.to_lowercase();
        self.repos
            .iter()
            .filter(|r| r.name.to_lowercase().contains(&q))
            .collect()
    }

    /// Find repos that provide a capability matching the given name (case-insensitive).
    pub fn by_capability(&self, cap: &str) -> Vec<&RepoEntry> {
        let c = cap.to_lowercase();
        self.repos
            .iter()
            .filter(|r| r.capabilities.iter().any(|cap| cap.name.to_lowercase() == c))
            .collect()
    }

    /// Find repos that have at least one capability in the given category.
    pub fn by_category(&self, cat: &str) -> Vec<&RepoEntry> {
        let c = cat.to_lowercase();
        self.repos
            .iter()
            .filter(|r| {
                r.capabilities
                    .iter()
                    .any(|cap| cap.category.to_lowercase() == c)
            })
            .collect()
    }

    /// Find repos by primary language.
    pub fn by_language(&self, lang: &str) -> Vec<&RepoEntry> {
        let l = lang.to_lowercase();
        self.repos
            .iter()
            .filter(|r| r.language.to_lowercase() == l)
            .collect()
    }

    /// Cross-reference a capability: who provides it and who depends on it.
    pub fn cross_reference(&self, cap_name: &str) -> CrossRef {
        let providers: Vec<String> = self
            .by_capability(cap_name)
            .iter()
            .map(|r| r.name.clone())
            .collect();

        let dependents: Vec<String> = self
            .repos
            .iter()
            .filter(|r| r.dependencies.iter().any(|d| d.to_lowercase() == cap_name.to_lowercase()))
            .map(|r| r.name.clone())
            .collect();

        CrossRef {
            capability: cap_name.to_string(),
            providers,
            dependents,
        }
    }

    /// Compute aggregate statistics.
    pub fn stats(&self) -> CatalogStats {
        let mut by_category: HashMap<String, usize> = HashMap::new();
        let mut by_language: HashMap<String, usize> = HashMap::new();
        let mut total_capabilities = 0;

        for repo in &self.repos {
            *by_language.entry(repo.language.clone()).or_insert(0) += 1;
            for cap in &repo.capabilities {
                total_capabilities += 1;
                *by_category.entry(cap.category.clone()).or_insert(0) += 1;
            }
        }

        CatalogStats {
            total_repos: self.repos.len(),
            total_capabilities,
            by_category,
            by_language,
        }
    }
}
