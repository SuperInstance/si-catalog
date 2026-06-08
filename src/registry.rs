//! Static registry data — hardcoded entries for all known SuperInstance repos.

use crate::catalog::Catalog;
use crate::types::{Capability, RepoEntry};

/// Build a [`Catalog`] pre-populated with all known SuperInstance repos.
pub fn default_catalog() -> Catalog {
    let mut cat = Catalog::new();
    for entry in all_entries() {
        cat.insert(entry);
    }
    cat
}

/// Returns the full list of known SuperInstance repo entries.
pub fn all_entries() -> Vec<RepoEntry> {
    vec![
        // ── Math ──────────────────────────────────────────────────────
        RepoEntry::new(
            "si-math",
            "rust",
            vec![
                Capability::new("linear-algebra", "math", "library",
                    "Core linear algebra: vectors, matrices, decompositions", "si-math"),
                Capability::new("statistics", "math", "library",
                    "Statistical distributions, hypothesis testing, regression", "si-math"),
                Capability::new("optimization", "math", "library",
                    "Gradient-based and gradient-free optimization solvers", "si-math"),
            ],
            vec![],
        ),
        RepoEntry::new(
            "si-math-gpu",
            "rust",
            vec![
                Capability::new("gpu-linear-algebra", "math", "library",
                    "CUDA/OpenCL-accelerated linear algebra kernels", "si-math-gpu"),
                Capability::new("gpu-fft", "math", "library",
                    "GPU-accelerated fast Fourier transforms", "si-math-gpu"),
            ],
            vec!["si-math".into()],
        ),
        RepoEntry::new(
            "si-signal",
            "rust",
            vec![
                Capability::new("signal-processing", "math", "library",
                    "Digital signal processing: filters, FFT, spectral analysis", "si-signal"),
                Capability::new("audio-codec", "math", "library",
                    "Audio encoding/decoding utilities", "si-signal"),
            ],
            vec!["si-math".into()],
        ),

        // ── Fleet ─────────────────────────────────────────────────────
        RepoEntry::new(
            "si-fleet",
            "rust",
            vec![
                Capability::new("fleet-manager", "fleet", "service",
                    "Central fleet orchestration and node management", "si-fleet"),
                Capability::new("node-discovery", "fleet", "service",
                    "Automatic node discovery and health monitoring", "si-fleet"),
                Capability::new("task-scheduler", "fleet", "service",
                    "Distributed task scheduling with priority queues", "si-fleet"),
            ],
            vec![],
        ),
        RepoEntry::new(
            "si-fleet-gateway",
            "rust",
            vec![
                Capability::new("gateway", "fleet", "service",
                    "HTTP/WebSocket gateway for fleet communication", "si-fleet-gateway"),
                Capability::new("load-balancer", "fleet", "service",
                    "Round-robin and least-connections load balancing", "si-fleet-gateway"),
            ],
            vec!["si-fleet".into()],
        ),
        RepoEntry::new(
            "si-fleet-provisioner",
            "rust",
            vec![
                Capability::new("provisioning", "fleet", "service",
                    "Automated node provisioning and configuration", "si-fleet-provisioner"),
            ],
            vec!["si-fleet".into()],
        ),
        RepoEntry::new(
            "si-catalog",
            "rust",
            vec![
                Capability::new("capability-search", "fleet", "library",
                    "Search and cross-reference fleet capabilities", "si-catalog"),
                Capability::new("catalog-diff", "fleet", "library",
                    "Diff catalog snapshots to track ecosystem growth", "si-catalog"),
                Capability::new("catalog-export", "fleet", "library",
                    "Export catalog to Markdown, JSON, DOT, Supabase SQL", "si-catalog"),
            ],
            vec!["si-fleet".into()],
        ),

        // ── Runtime ───────────────────────────────────────────────────
        RepoEntry::new(
            "si-runtime",
            "rust",
            vec![
                Capability::new("task-runtime", "runtime", "library",
                    "Async task execution runtime with cancellation support", "si-runtime"),
                Capability::new("worker-pool", "runtime", "library",
                    "Configurable worker pool with back-pressure", "si-runtime"),
            ],
            vec![],
        ),
        RepoEntry::new(
            "si-runtime-plugins",
            "rust",
            vec![
                Capability::new("plugin-loader", "runtime", "library",
                    "Dynamic plugin loading and lifecycle management", "si-runtime-plugins"),
                Capability::new("plugin-registry", "runtime", "library",
                    "Plugin metadata registry and versioning", "si-runtime-plugins"),
            ],
            vec!["si-runtime".into()],
        ),
        RepoEntry::new(
            "si-config",
            "rust",
            vec![
                Capability::new("config-management", "runtime", "library",
                    "Hierarchical configuration loading and validation", "si-config"),
                Capability::new("secrets-vault", "runtime", "library",
                    "Encrypted secrets storage and rotation", "si-config"),
            ],
            vec![],
        ),
        RepoEntry::new(
            "si-logging",
            "rust",
            vec![
                Capability::new("structured-logging", "runtime", "library",
                    "Structured JSON logging with span correlation", "si-logging"),
                Capability::new("log-aggregation", "runtime", "service",
                    "Centralized log collection and query", "si-logging"),
            ],
            vec![],
        ),

        // ── Infrastructure ────────────────────────────────────────────
        RepoEntry::new(
            "si-infra",
            "rust",
            vec![
                Capability::new("infrastructure-as-code", "infrastructure", "library",
                    "Declarative infrastructure provisioning", "si-infra"),
                Capability::new("terraform-provider", "infrastructure", "library",
                    "Custom Terraform provider for SuperInstance resources", "si-infra"),
            ],
            vec!["si-fleet".into(), "si-config".into()],
        ),
        RepoEntry::new(
            "si-monitor",
            "rust",
            vec![
                Capability::new("metrics", "infrastructure", "service",
                    "Prometheus-compatible metrics collection and export", "si-monitor"),
                Capability::new("alerting", "infrastructure", "service",
                    "Rule-based alerting with multi-channel notifications", "si-monitor"),
                Capability::new("health-checks", "infrastructure", "service",
                    "Automated health check execution and reporting", "si-monitor"),
            ],
            vec!["si-fleet".into()],
        ),
        RepoEntry::new(
            "si-storage",
            "rust",
            vec![
                Capability::new("object-storage", "infrastructure", "library",
                    "S3-compatible object storage abstraction", "si-storage"),
                Capability::new("cache", "infrastructure", "library",
                    "In-memory and distributed caching layer", "si-storage"),
            ],
            vec![],
        ),
        RepoEntry::new(
            "si-db",
            "rust",
            vec![
                Capability::new("database-pool", "infrastructure", "library",
                    "Connection pooling for PostgreSQL and SQLite", "si-db"),
                Capability::new("migrations", "infrastructure", "library",
                    "Database schema migration runner", "si-db"),
            ],
            vec![],
        ),
        RepoEntry::new(
            "si-network",
            "rust",
            vec![
                Capability::new("mesh-networking", "infrastructure", "service",
                    "Libp2p-based mesh networking for fleet nodes", "si-network"),
                Capability::new("service-mesh", "infrastructure", "service",
                    "mTLS service mesh with traffic management", "si-network"),
            ],
            vec!["si-fleet".into()],
        ),
        RepoEntry::new(
            "si-auth",
            "rust",
            vec![
                Capability::new("authentication", "infrastructure", "service",
                    "JWT and mTLS authentication service", "si-auth"),
                Capability::new("authorization", "infrastructure", "service",
                    "RBAC and ABAC policy engine", "si-auth"),
            ],
            vec!["si-config".into()],
        ),
        RepoEntry::new(
            "si-ci",
            "rust",
            vec![
                Capability::new("ci-pipeline", "infrastructure", "service",
                    "Continuous integration pipeline runner", "si-ci"),
                Capability::new("artifact-registry", "infrastructure", "service",
                    "Build artifact storage and versioning", "si-ci"),
            ],
            vec!["si-fleet".into(), "si-storage".into()],
        ),

        // ── Research ──────────────────────────────────────────────────
        RepoEntry::new(
            "si-research-core",
            "rust",
            vec![
                Capability::new("experiment-runner", "research", "library",
                    "Reproducible experiment execution with parameter sweeps", "si-research-core"),
                Capability::new("data-pipeline", "research", "library",
                    "ETL pipeline for research datasets", "si-research-core"),
                Capability::new("notebook-engine", "research", "library",
                    "Jupyter-compatible notebook execution engine", "si-research-core"),
            ],
            vec!["si-math".into(), "si-runtime".into()],
        ),
        RepoEntry::new(
            "si-research-ml",
            "python",
            vec![
                Capability::new("model-training", "research", "library",
                    "PyTorch-based model training utilities", "si-research-ml"),
                Capability::new("model-inference", "research", "service",
                    "Optimized model inference serving", "si-research-ml"),
                Capability::new("auto-ml", "research", "library",
                    "Automated hyperparameter search and model selection", "si-research-ml"),
            ],
            vec!["si-math".into(), "si-math-gpu".into()],
        ),
        RepoEntry::new(
            "si-research-viz",
            "python",
            vec![
                Capability::new("data-visualization", "research", "library",
                    "Publication-quality plots and interactive dashboards", "si-research-viz"),
                Capability::new("geospatial-viz", "research", "library",
                    "Geospatial data visualization with map overlays", "si-research-viz"),
            ],
            vec!["si-research-core".into()],
        ),
        RepoEntry::new(
            "si-research-nlp",
            "python",
            vec![
                Capability::new("text-analysis", "research", "library",
                    "NLP text classification, NER, and sentiment analysis", "si-research-nlp"),
                Capability::new("embeddings", "research", "library",
                    "Text embedding generation and similarity search", "si-research-nlp"),
            ],
            vec!["si-research-ml".into()],
        ),
        RepoEntry::new(
            "si-research-sim",
            "rust",
            vec![
                Capability::new("monte-carlo", "research", "library",
                    "Monte Carlo simulation framework", "si-research-sim"),
                Capability::new("agent-based-model", "research", "library",
                    "Agent-based modeling for ecological simulations", "si-research-sim"),
            ],
            vec!["si-math".into(), "si-runtime".into()],
        ),

        // ── Conservation ──────────────────────────────────────────────
        RepoEntry::new(
            "si-conservation-core",
            "rust",
            vec![
                Capability::new("species-tracker", "conservation", "service",
                    "Species occurrence tracking and population modeling", "si-conservation-core"),
                Capability::new("habitat-mapper", "conservation", "service",
                    "Habitat suitability mapping and change detection", "si-conservation-core"),
                Capability::new("threat-assessment", "conservation", "service",
                    "Automated threat level assessment for species and habitats", "si-conservation-core"),
            ],
            vec!["si-math".into(), "si-research-sim".into()],
        ),
        RepoEntry::new(
            "si-conservation-field",
            "rust",
            vec![
                Capability::new("field-data-collection", "conservation", "service",
                    "Offline-first mobile field data collection", "si-conservation-field"),
                Capability::new("gps-tracking", "conservation", "service",
                    "GPS collar and tag data ingestion and analysis", "si-conservation-field"),
            ],
            vec!["si-conservation-core".into()],
        ),
        RepoEntry::new(
            "si-conservation-gis",
            "rust",
            vec![
                Capability::new("gis-processing", "conservation", "library",
                    "Geographic information system data processing", "si-conservation-gis"),
                Capability::new("remote-sensing", "conservation", "library",
                    "Satellite imagery ingestion and analysis", "si-conservation-gis"),
            ],
            vec!["si-conservation-core".into(), "si-storage".into()],
        ),
        RepoEntry::new(
            "si-wildlife-detect",
            "rust",
            vec![
                Capability::new("wildlife-detection", "conservation", "service",
                    "Camera trap image classification and species detection", "si-wildlife-detect"),
                Capability::new("audio-monitoring", "conservation", "service",
                    "Bioacoustic monitoring and species identification", "si-wildlife-detect"),
            ],
            vec!["si-research-ml".into(), "si-signal".into()],
        ),
        RepoEntry::new(
            "si-marine",
            "rust",
            vec![
                Capability::new("marine-survey", "conservation", "service",
                    "Marine survey data management and analysis", "si-marine"),
                Capability::new("oceanographic-modeling", "conservation", "library",
                    "Ocean current and temperature modeling", "si-marine"),
            ],
            vec!["si-conservation-core".into(), "si-math".into()],
        ),
        RepoEntry::new(
            "si-conservation-policy",
            "rust",
            vec![
                Capability::new("policy-engine", "conservation", "library",
                    "Conservation policy rules engine and compliance checker", "si-conservation-policy"),
                Capability::new("reporting", "conservation", "service",
                    "Automated conservation status report generation", "si-conservation-policy"),
            ],
            vec!["si-conservation-core".into()],
        ),

        // ── Additional cross-cutting repos ────────────────────────────
        RepoEntry::new(
            "si-sdk",
            "rust",
            vec![
                Capability::new("client-sdk", "fleet", "library",
                    "Official Rust SDK for interacting with SuperInstance services", "si-sdk"),
            ],
            vec!["si-fleet-gateway".into(), "si-auth".into()],
        ),
        RepoEntry::new(
            "si-cli",
            "rust",
            vec![
                Capability::new("cli", "fleet", "tool",
                    "Command-line interface for SuperInstance management", "si-cli"),
            ],
            vec!["si-sdk".into(), "si-config".into()],
        ),
        RepoEntry::new(
            "si-web",
            "typescript",
            vec![
                Capability::new("web-dashboard", "fleet", "service",
                    "React-based fleet management dashboard", "si-web"),
                Capability::new("data-explorer", "research", "service",
                    "Interactive data exploration and visualization UI", "si-web"),
            ],
            vec!["si-fleet-gateway".into()],
        ),
        RepoEntry::new(
            "si-docs",
            "typescript",
            vec![
                Capability::new("documentation", "fleet", "service",
                    "Auto-generated API documentation and guides", "si-docs"),
            ],
            vec![],
        ),
        RepoEntry::new(
            "si-bench",
            "rust",
            vec![
                Capability::new("benchmarking", "infrastructure", "tool",
                    "Performance benchmarking and regression detection", "si-bench"),
            ],
            vec!["si-runtime".into()],
        ),
        RepoEntry::new(
            "si-archiver",
            "rust",
            vec![
                Capability::new("data-archival", "infrastructure", "service",
                    "Long-term data archival with compression and integrity checks", "si-archiver"),
            ],
            vec!["si-storage".into()],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_catalog_not_empty() {
        let cat = default_catalog();
        assert!(cat.len() >= 30, "expected at least 30 repos, got {}", cat.len());
    }

    #[test]
    fn all_entries_unique_names() {
        let entries = all_entries();
        let mut names = std::collections::HashSet::new();
        for e in &entries {
            assert!(names.insert(e.name.clone()), "duplicate repo name: {}", e.name);
        }
    }

    #[test]
    fn categories_are_valid() {
        let valid = ["math", "fleet", "runtime", "infrastructure", "research", "conservation"];
        for entry in all_entries() {
            for cap in &entry.capabilities {
                assert!(
                    valid.contains(&cap.category.as_str()),
                    "invalid category '{}' in {}",
                    cap.category,
                    cap.name
                );
            }
        }
    }
}
