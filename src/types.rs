//! Core types for the SuperInstance capability catalog.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single capability exposed by a repository.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Capability {
    /// Short machine-readable name, e.g. `"wildlife-detection"`.
    pub name: String,
    /// Broad category: math, fleet, runtime, infrastructure, research, conservation.
    pub category: String,
    /// What this capability provides (API, library, service, etc.).
    pub provides: String,
    /// Human-readable description.
    pub description: String,
    /// Source repository short name, e.g. `"si-math"`.
    pub repo: String,
}

impl Capability {
    /// Convenience constructor.
    pub fn new(name: &str, category: &str, provides: &str, description: &str, repo: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            provides: provides.to_string(),
            description: description.to_string(),
            repo: repo.to_string(),
        }
    }
}

/// Entry for a single repository in the catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoEntry {
    /// Repository short name, e.g. `"si-math"`.
    pub name: String,
    /// Primary language, e.g. `"rust"`.
    pub language: String,
    /// Capabilities this repo provides.
    pub capabilities: Vec<Capability>,
    /// Other repos or capabilities this repo depends on.
    pub dependencies: Vec<String>,
}

impl RepoEntry {
    /// Convenience constructor.
    pub fn new(name: &str, language: &str, capabilities: Vec<Capability>, dependencies: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            language: language.to_string(),
            capabilities,
            dependencies,
        }
    }

    /// Returns true if this repo has a capability with the given name.
    pub fn has_capability(&self, cap_name: &str) -> bool {
        self.capabilities.iter().any(|c| c.name == cap_name)
    }

    /// Returns the capability with the given name, if any.
    pub fn get_capability(&self, cap_name: &str) -> Option<&Capability> {
        self.capabilities.iter().find(|c| c.name == cap_name)
    }

    /// All categories this repo touches.
    pub fn categories(&self) -> Vec<&str> {
        let mut cats: Vec<&str> = self.capabilities.iter().map(|c| c.category.as_str()).collect();
        cats.sort();
        cats.dedup();
        cats
    }
}

/// Aggregate statistics about the catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatalogStats {
    pub total_repos: usize,
    pub total_capabilities: usize,
    pub by_category: HashMap<String, usize>,
    pub by_language: HashMap<String, usize>,
}

/// Cross-reference result for a capability.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CrossRef {
    /// The capability name that was queried.
    pub capability: String,
    /// Repos that provide this capability.
    pub providers: Vec<String>,
    /// Repos that depend on this capability.
    pub dependents: Vec<String>,
}

impl CrossRef {
    /// Returns true if this capability is a leaf (nothing depends on it).
    pub fn is_leaf(&self) -> bool {
        self.dependents.is_empty()
    }

    /// Returns true if this capability is a root (nothing provides it — unusual).
    pub fn is_root(&self) -> bool {
        self.providers.is_empty()
    }
}
