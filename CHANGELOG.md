# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1] - 2026-05-14

### Added
- Foundation and scaffolding for the **AI PR Reviewer** single-binary Rust tool crate.
- Enterprise-ready `Cargo.toml` with explicit metadata mapping, release optimization profiles, and strict dependency features (`clap`, `serde`, `serde_json`, `thiserror`).
- Root Git repository hygiene guidelines cleanly bounded within `.gitignore`.
- Core Git interaction module (`src/git.rs`) capable of parsing local unified diffs and calculating exact target line mappings.
- Decoupled trait interface (`GitProvider`) isolating source-code buffer ingestion from future evaluation engines.
- Homebrew formula script (`Formula/ai-pr-reviewer.rb`) supporting native pre-compiled distribution.

[0.0.1]: https://github.com/dhia-bechattaoui/ai-pr-reviewer/releases/tag/v0.0.1
