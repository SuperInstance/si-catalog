# si-catalog

> Unified capability catalog for **SuperInstance** — search, cross-reference, and discover fleet capabilities.

[![Crates.io](https://img.shields.io/crates/v/si-catalog.svg)](https://crates.io/crates/si-catalog)
[![Docs.rs](https://docs.rs/si-catalog/badge.svg)](https://docs.rs/si-catalog)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Types](#core-types)
  - [Capability](#capability)
  - [RepoEntry](#repoentry)
  - [CatalogStats](#catalogstats)
  - [CrossRef](#crossref)
- [The Catalog](#the-catalog)
  - [Creating a Catalog](#creating-a-catalog)
  - [Search](#search)
  - [Filtering](#filtering)
  - [Cross-Referencing](#cross-referencing)
  - [Statistics](#statistics)
- [Static Registry](#static-registry)
  - [All Known Repos](#all-known-repos)
  - [Categories](#categories)
- [Catalog Diffing](#catalog-diffing)
- [Export Formats](#export-formats)
  - [Markdown Table](#markdown-table)
  - [JSON Export](#json-export)
  - [Graphviz DOT Graph](#graphviz-dot-graph)
  - [Supabase Seed SQL](#supabase-seed-sql)
- [Supabase Integration Guide](#supabase-integration-guide)
  - [Schema Setup](#schema-setup)
  - [Seeding from si-catalog](#seeding-from-si-catalog)
  - [Querying the Catalog in SQL](#querying-the-catalog-in-sql)
- [DOT Graph Visualization](#dot-graph-visualization)
  - [Generating the Graph](#generating-the-graph)
  - [Styling by Language](#styling-by-language)
  - [Example Output](#example-output)
- [Architecture](#architecture)
- [API Reference](#api-reference)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

`si-catalog` provides a single source of truth for all capabilities across the SuperInstance fleet. It ships with a **hardcoded registry** of 35+ repositories spanning six categories:

| Category        | Description                                    |
|-----------------|------------------------------------------------|
| **math**        | Linear algebra, statistics, signal processing   |
| **fleet**       | Orchestration, scheduling, gateway, SDK         |
| **runtime**     | Async runtime, plugins, config, logging         |
| **infrastructure** | Storage, networking, auth, CI, monitoring   |
| **research**    | ML training, NLP, simulation, visualization     |
| **conservation**| Wildlife detection, GIS, marine, policy          |

Use it to:

- **Search** for repos by name, capability, category, or language
- **Cross-reference** who provides a capability and who depends on it
- **Diff** catalog snapshots to track ecosystem growth
- **Export** to Markdown, JSON, Graphviz DOT, or Supabase SQL

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
si-catalog = "0.1"
```

Or via `cargo add`:

```bash
cargo add si-catalog
```

---

## Quick Start

```rust
use si_catalog::registry::default_catalog;

fn main() {
    // Get the pre-populated catalog with all known repos
    let catalog = default_catalog();

    // Basic stats
    let stats = catalog.stats();
    println!("📦 {} repos, {} capabilities",
        stats.total_repos, stats.total_capabilities);

    // Search for repos
    for repo in catalog.search("math") {
        println!("Found: {} ({})", repo.name, repo.language);
    }

    // Find who provides wildlife detection
    let providers = catalog.by_capability("wildlife-detection");
    for repo in &providers {
        println!("{} provides wildlife-detection", repo.name);
    }

    // Cross-reference
    let xr = catalog.cross_reference("si-math");
    println!("Providers: {:?}", xr.providers);
    println!("Dependents: {:?}", xr.dependents);

    // Export to various formats
    let markdown = si_catalog::export::export_markdown(&catalog);
    let json = si_catalog::export::export_json(&catalog);
    let dot = si_catalog::export::export_dot(&catalog);
    let sql = si_catalog::export::export_supabase_seed(&catalog);
}
```

---

## Core Types

### Capability

A single capability exposed by a repository:

```rust
use si_catalog::Capability;

let cap = Capability::new(
    "wildlife-detection",    // name
    "conservation",          // category
    "service",               // provides
    "Camera trap image classification", // description
    "si-wildlife-detect",    // repo
);
```

Fields:
- `name` — Short machine-readable identifier
- `category` — One of: math, fleet, runtime, infrastructure, research, conservation
- `provides` — What kind of thing: library, service, tool
- `description` — Human-readable summary
- `repo` — Source repository short name

### RepoEntry

A repository and its capabilities:

```rust
use si_catalog::{RepoEntry, Capability};

let entry = RepoEntry::new(
    "si-wildlife-detect",
    "rust",
    vec![
        Capability::new("wildlife-detection", "conservation", "service",
            "Camera trap image classification and species detection", "si-wildlife-detect"),
        Capability::new("audio-monitoring", "conservation", "service",
            "Bioacoustic monitoring and species identification", "si-wildlife-detect"),
    ],
    vec!["si-research-ml".into(), "si-signal".into()],
);

// Helper methods
assert!(entry.has_capability("wildlife-detection"));
assert!(entry.get_capability("audio-monitoring").is_some());
assert_eq!(entry.categories(), vec!["conservation"]);
```

### CatalogStats

Aggregate statistics returned by `catalog.stats()`:

```rust
let stats = catalog.stats();
println!("Repos: {}", stats.total_repos);
println!("Capabilities: {}", stats.total_capabilities);
println!("By category: {:?}", stats.by_category);
println!("By language: {:?}", stats.by_language);
```

### CrossRef

Cross-reference result for a capability or repo:

```rust
let xr = catalog.cross_reference("si-math");
println!("Capability: {}", xr.capability);
println!("Providers: {:?}", xr.providers);   // repos that provide it
println!("Dependents: {:?}", xr.dependents); // repos that depend on it
println!("Is leaf: {}", xr.is_leaf());       // nothing depends on it
println!("Is root: {}", xr.is_root());       // nothing provides it
```

---

## The Catalog

### Creating a Catalog

```rust
use si_catalog::Catalog;
use si_catalog::RepoEntry;

// Empty catalog
let mut cat = Catalog::new();

// Insert entries
cat.insert(RepoEntry::new("my-repo", "rust", vec![], vec![]));

// Insert replaces by name
cat.insert(RepoEntry::new("my-repo", "python", vec![], vec![]));
assert_eq!(cat.get("my-repo").unwrap().language, "python");
```

### Search

Fuzzy substring search on repo names (case-insensitive):

```rust
let results = catalog.search("math");
// Returns: si-math, si-math-gpu
```

### Filtering

```rust
// By capability name
let repos = catalog.by_capability("wildlife-detection");

// By category
let math_repos = catalog.by_category("math");
let conservation_repos = catalog.by_category("conservation");

// By primary language
let rust_repos = catalog.by_language("rust");
let python_repos = catalog.by_language("python");
```

### Cross-Referencing

Find who provides and who depends on a capability or repo name:

```rust
let xr = catalog.cross_reference("si-math");
// providers: repos that have a capability with this name
// dependents: repos that list this in their dependencies
```

### Statistics

```rust
let stats = catalog.stats();
// total_repos, total_capabilities, by_category, by_language
```

---

## Static Registry

The `registry` module provides a pre-built catalog with all known SuperInstance repos.

```rust
use si_catalog::registry::default_catalog;

let catalog = default_catalog();
assert!(catalog.len() >= 30);
```

### All Known Repos

| # | Repo | Language | Category | Key Capabilities |
|---|------|----------|----------|------------------|
| 1 | si-math | rust | math | linear-algebra, statistics, optimization |
| 2 | si-math-gpu | rust | math | gpu-linear-algebra, gpu-fft |
| 3 | si-signal | rust | math | signal-processing, audio-codec |
| 4 | si-fleet | rust | fleet | fleet-manager, node-discovery, task-scheduler |
| 5 | si-fleet-gateway | rust | fleet | gateway, load-balancer |
| 6 | si-fleet-provisioner | rust | fleet | provisioning |
| 7 | si-catalog | rust | fleet | capability-search, catalog-diff, catalog-export |
| 8 | si-runtime | rust | runtime | task-runtime, worker-pool |
| 9 | si-runtime-plugins | rust | runtime | plugin-loader, plugin-registry |
| 10 | si-config | rust | runtime | config-management, secrets-vault |
| 11 | si-logging | rust | runtime | structured-logging, log-aggregation |
| 12 | si-infra | rust | infrastructure | infrastructure-as-code, terraform-provider |
| 13 | si-monitor | rust | infrastructure | metrics, alerting, health-checks |
| 14 | si-storage | rust | infrastructure | object-storage, cache |
| 15 | si-db | rust | infrastructure | database-pool, migrations |
| 16 | si-network | rust | infrastructure | mesh-networking, service-mesh |
| 17 | si-auth | rust | infrastructure | authentication, authorization |
| 18 | si-ci | rust | infrastructure | ci-pipeline, artifact-registry |
| 19 | si-research-core | rust | research | experiment-runner, data-pipeline, notebook-engine |
| 20 | si-research-ml | python | research | model-training, model-inference, auto-ml |
| 21 | si-research-viz | python | research | data-visualization, geospatial-viz |
| 22 | si-research-nlp | python | research | text-analysis, embeddings |
| 23 | si-research-sim | rust | research | monte-carlo, agent-based-model |
| 24 | si-conservation-core | rust | conservation | species-tracker, habitat-mapper, threat-assessment |
| 25 | si-conservation-field | rust | conservation | field-data-collection, gps-tracking |
| 26 | si-conservation-gis | rust | conservation | gis-processing, remote-sensing |
| 27 | si-wildlife-detect | rust | conservation | wildlife-detection, audio-monitoring |
| 28 | si-marine | rust | conservation | marine-survey, oceanographic-modeling |
| 29 | si-conservation-policy | rust | conservation | policy-engine, reporting |
| 30 | si-sdk | rust | fleet | client-sdk |
| 31 | si-cli | rust | fleet | cli |
| 32 | si-web | typescript | fleet, research | web-dashboard, data-explorer |
| 33 | si-docs | typescript | fleet | documentation |
| 34 | si-bench | rust | infrastructure | benchmarking |
| 35 | si-archiver | rust | infrastructure | data-archival |

### Categories

Six top-level categories organize all capabilities:

- **math** — Pure math and signal processing foundations
- **fleet** — Cluster management, orchestration, and developer tools
- **runtime** — Task execution, plugins, config, and observability
- **infrastructure** — Storage, networking, auth, CI/CD, monitoring
- **research** — ML, NLP, simulation, and visualization
- **conservation** — Wildlife, GIS, marine, and policy

---

## Catalog Diffing

Track ecosystem growth by comparing catalog snapshots:

```rust
use si_catalog::diff::diff_catalogs;
use si_catalog::registry::default_catalog;

let old = default_catalog();

// Later, after adding repos:
let new = default_catalog(); // or modified catalog

let diff = diff_catalogs(&old, &new);
println!("Added repos: {:?}", diff.added_repos);
println!("Removed repos: {:?}", diff.removed_repos);
println!("Added capabilities: {:?}", diff.added_capabilities);
println!("Removed capabilities: {:?}", diff.removed_capabilities);

if diff.is_empty() {
    println!("No changes detected");
}
println!("Total changes: {}", diff.total_changes());
```

`CatalogDiff` fields:
- `added_repos` — Repo names in `new` but not `old`
- `removed_repos` — Repo names in `old` but not `new`
- `added_capabilities` — Capability names in `new` but not `old`
- `removed_capabilities` — Capability names in `old` but not `new`

---

## Export Formats

### Markdown Table

Generate a formatted Markdown table of all repos:

```rust
use si_catalog::export::export_markdown;
use si_catalog::registry::default_catalog;

let markdown = export_markdown(&default_catalog());
println!("{}", markdown);
```

Output:

```markdown
| Repo | Language | Capabilities | Categories |
|------|----------|-------------|------------|
| si-auth | rust | authentication, authorization | infrastructure |
| si-bench | rust | benchmarking | infrastructure |
| si-catalog | rust | capability-search, catalog-diff, catalog-export | fleet |
...
```

### JSON Export

Serialize the entire catalog as JSON:

```rust
use si_catalog::export::export_json;

let json = export_json(&catalog);
// Returns a JSON array of RepoEntry objects
```

Example output:

```json
[
  {
    "name": "si-math",
    "language": "rust",
    "capabilities": [
      {
        "name": "linear-algebra",
        "category": "math",
        "provides": "library",
        "description": "Core linear algebra: vectors, matrices, decompositions",
        "repo": "si-math"
      }
    ],
    "dependencies": []
  }
]
```

### Graphviz DOT Graph

Generate a dependency graph in DOT format:

```rust
use si_catalog::export::export_dot;

let dot = export_dot(&catalog);
// Feed to `dot -Tpng` or any Graphviz renderer
```

Nodes are colored by language:
- **Rust** → lightblue
- **Python** → lightgreen
- **TypeScript** → plum
- **Other** → lightyellow

### Supabase Seed SQL

Generate `INSERT` statements for Supabase/PostgreSQL:

```rust
use si_catalog::export::export_supabase_seed;

let sql = export_supabase_seed(&catalog);
```

Generates:

```sql
-- Repos
INSERT INTO repos (name, language) VALUES ('si-math', 'rust')
  ON CONFLICT (name) DO UPDATE SET language = EXCLUDED.language;

-- Capabilities
INSERT INTO capabilities (name, category, provides, description, repo)
  VALUES ('linear-algebra', 'math', 'library', 'Core linear algebra...', 'si-math')
  ON CONFLICT (name) DO UPDATE SET ...

-- Dependencies
INSERT INTO dependencies (repo, depends_on) VALUES ('si-math-gpu', 'si-math');
```

Uses `ON CONFLICT` for idempotent seeding.

---

## Supabase Integration Guide

### Schema Setup

Create these tables in your Supabase project:

```sql
-- Repositories
CREATE TABLE repos (
    name TEXT PRIMARY KEY,
    language TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Capabilities
CREATE TABLE capabilities (
    name TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    provides TEXT NOT NULL,
    description TEXT,
    repo TEXT NOT NULL REFERENCES repos(name),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Dependencies between repos
CREATE TABLE dependencies (
    id BIGSERIAL PRIMARY KEY,
    repo TEXT NOT NULL REFERENCES repos(name),
    depends_on TEXT NOT NULL REFERENCES repos(name),
    UNIQUE(repo, depends_on)
);

-- Useful indexes
CREATE INDEX idx_capabilities_category ON capabilities(category);
CREATE INDEX idx_capabilities_repo ON capabilities(repo);
CREATE INDEX idx_dependencies_repo ON dependencies(repo);
CREATE INDEX idx_dependencies_depends_on ON dependencies(depends_on);
```

### Seeding from si-catalog

Use the `export_supabase_seed` function to generate a complete seed file:

```rust
use si_catalog::registry::default_catalog;
use si_catalog::export::export_supabase_seed;
use std::fs;

let sql = export_supabase_seed(&default_catalog());
fs::write("seed.sql", &sql).unwrap();
```

Then run against your Supabase database:

```bash
# Using Supabase CLI
supabase db reset

# Or directly with psql
psql "$DATABASE_URL" < seed.sql
```

### Querying the Catalog in SQL

Once seeded, use these queries:

```sql
-- All repos by language
SELECT language, COUNT(*) FROM repos GROUP BY language;

-- All capabilities in the conservation category
SELECT name, description, repo
FROM capabilities
WHERE category = 'conservation';

-- Dependency graph: what does si-wildlife-detect depend on?
WITH RECURSIVE deps AS (
    SELECT depends_on, 1 AS depth
    FROM dependencies WHERE repo = 'si-wildlife-detect'
    UNION ALL
    SELECT d.depends_on, deps.depth + 1
    FROM dependencies d
    JOIN deps ON d.repo = deps.depends_on
    WHERE deps.depth < 10
)
SELECT * FROM deps;

-- Most depended-upon repos
SELECT depends_on, COUNT(*) AS dependents
FROM dependencies
GROUP BY depends_on
ORDER BY dependents DESC;
```

---

## DOT Graph Visualization

### Generating the Graph

```rust
use si_catalog::export::export_dot;
use si_catalog::registry::default_catalog;
use std::process::Command;
use std::fs;

let dot = export_dot(&default_catalog());
fs::write("fleet.dot", &dot).unwrap();

// Render to PNG
Command::new("dot")
    .args(["-Tpng", "fleet.dot", "-o", "fleet.png"])
    .status()
    .unwrap();
```

Or from the command line:

```bash
cargo run --example export_dot | dot -Tsvg -o fleet.svg
```

### Styling by Language

The DOT export automatically colors nodes by language:

| Language   | Fill Color  |
|------------|-------------|
| Rust       | lightblue   |
| Python     | lightgreen  |
| TypeScript | plum        |
| Other      | lightyellow |

### Example Output

```
digraph SuperInstance {
    rankdir=LR;
    node [shape=box, style=filled, fillcolor=lightyellow];

    "si-auth" [fillcolor=lightblue];
    "si-catalog" [fillcolor=lightblue];
    "si-math" [fillcolor=lightblue];
    "si-research-ml" [fillcolor=lightgreen];
    "si-web" [fillcolor=plum];

    "si-catalog" -> "si-fleet";
    "si-math-gpu" -> "si-math";
    "si-wildlife-detect" -> "si-research-ml";
    "si-wildlife-detect" -> "si-signal";
}
```

---

## Architecture

```
si-catalog
├── src/
│   ├── lib.rs         # Re-exports + 40+ integration tests
│   ├── types.rs       # Capability, RepoEntry, CatalogStats, CrossRef
│   ├── catalog.rs     # Catalog struct with search/filter/cross-ref
│   ├── registry.rs    # 35 hardcoded repos + default_catalog()
│   ├── diff.rs        # diff_catalogs() → CatalogDiff
│   └── export.rs      # Markdown, JSON, DOT, Supabase SQL exporters
├── Cargo.toml
└── README.md
```

### Module Responsibilities

- **types** — Data structures with serde support
- **catalog** — In-memory catalog with indexed lookups and fuzzy search
- **registry** — Static data representing the current SuperInstance ecosystem
- **diff** — Compare two catalog snapshots
- **export** — Format conversion for external tools

### Design Decisions

1. **In-memory only** — No I/O, no async. Pure data structures.
2. **Serde everywhere** — All types derive `Serialize`/`Deserialize`.
3. **Case-insensitive lookups** — Names are indexed lowercase.
4. **Replace semantics** — `insert()` replaces by name; no duplicates.
5. **Substring search** — Simple fuzzy matching; extensible to Levenshtein later.

---

## API Reference

### `Catalog`

| Method | Description |
|--------|-------------|
| `new()` | Create empty catalog |
| `insert(entry)` | Insert or replace a repo |
| `get(name)` | Get repo by name (case-insensitive) |
| `repos()` | Iterator over all repos |
| `len()` / `is_empty()` | Repo count |
| `search(query)` | Fuzzy name search |
| `by_capability(name)` | Repos providing a capability |
| `by_category(cat)` | Repos in a category |
| `by_language(lang)` | Repos by language |
| `cross_reference(name)` | Providers + dependents |
| `stats()` | Aggregate statistics |

### `Capability`

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Machine-readable identifier |
| `category` | String | math/fleet/runtime/infrastructure/research/conservation |
| `provides` | String | library/service/tool |
| `description` | String | Human-readable summary |
| `repo` | String | Source repo name |

### `RepoEntry`

| Field/Method | Description |
|--------------|-------------|
| `name` | Repo short name |
| `language` | Primary language |
| `capabilities` | Vec of Capability |
| `dependencies` | Vec of dependency names |
| `has_capability(name)` | Check if repo has a capability |
| `get_capability(name)` | Get specific capability |
| `categories()` | Unique categories across all capabilities |

### `CatalogDiff`

| Field/Method | Description |
|--------------|-------------|
| `added_repos` | New repo names |
| `removed_repos` | Removed repo names |
| `added_capabilities` | New capability names |
| `removed_capabilities` | Removed capability names |
| `is_empty()` | No changes |
| `total_changes()` | Count of all changes |

### Export Functions

| Function | Output |
|----------|--------|
| `export_markdown(catalog)` | Markdown table string |
| `export_json(catalog)` | JSON array of repos |
| `export_dot(catalog)` | Graphviz DOT digraph |
| `export_supabase_seed(catalog)` | SQL INSERT statements |

---

## Contributing

1. Fork the repo
2. Add your changes (new repos go in `src/registry.rs`)
3. Run `cargo test` — all 40+ tests must pass
4. Ensure new capabilities use valid categories
5. Submit a PR

### Adding a New Repo

Edit `src/registry.rs` and add to the `all_entries()` vec:

```rust
RepoEntry::new(
    "si-your-new-repo",
    "rust",
    vec![
        Capability::new(
            "your-capability",
            "fleet",  // must be a valid category
            "library",
            "What it does",
            "si-your-new-repo",
        ),
    ],
    vec!["si-dependency".into()],
),
```

### Adding a New Category

1. Add the category name to the `categories_are_valid` test
2. Add repos using the new category
3. Update this README

---

## License

MIT — see [LICENSE](LICENSE) for details.
