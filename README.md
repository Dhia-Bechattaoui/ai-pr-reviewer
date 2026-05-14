# AI PR Reviewer

> **AI-Debt Governor:** Automated CI gatekeeper and command-line tool designed to intercept low-quality AI-generated code diffs before they reach human reviewers.

Rapid AI code generation often leads to increased review fatigue for human maintainers. **AI PR Reviewer** parses unified diffs to extract precise context and modifications, providing deterministic JSON outputs tailored for seamless integration into static analysis pipelines, custom heuristics, and GitHub Actions.

## Features

- **Robust Unified Diff Parsing:** Extracts added, deleted, and context line blocks cleanly while accurately tracking target line numbers.
- **Decoupled Architecture:** Isolates Git buffer ingestion from downstream review/evaluation engines via flexible Rust trait boundaries.
- **Deterministic Structured JSON Output:** Streams modification details as standardized JSON objects for reliable consumption by automated scripts and CI/CD tools.
- **Flexible Source Inputs:** Supports reading directly from local Git staging areas, unstaged modifications, or explicit raw diff strings.
- **High Performance & Single-Binary Portability:** Written entirely in optimized Rust for maximum speed, strict memory safety, and minimal runtime footprint.

## Installation

### From Source (Cargo)

Ensure you have a recent version of the Rust toolchain installed, then clone and install the crate locally:

```bash
git clone https://github.com/dhia-bechattaoui/ai-pr-reviewer.git
cd ai-pr-reviewer
cargo install --path .
```

### Via Homebrew (macOS / Linux)

You can install the tool directly using the included Homebrew formula:

```bash
brew install --build-from-source Formula/ai-pr-reviewer.rb
```

## Usage

By default, running `ai-pr-reviewer` inspects unstaged modifications in your local Git repository:

```bash
ai-pr-reviewer
```

### CLI Options

- **`--staged` (`-s`)**: Intercept staged/cached Git buffers instead of unstaged modifications.
  ```bash
  ai-pr-reviewer --staged
  ```

- **`--raw-diff` (`-r`)**: Provide an explicit raw diff string to parse directly instead of executing the local `git` binary.
  ```bash
  ai-pr-reviewer --raw-diff "diff --git a/src/main.rs b/src/main.rs..."
  ```

### Output Example

The CLI outputs clean JSON arrays describing modified files, their containing hunks, and exact line mappings:

```json
[
  {
    "old_path": "src/main.rs",
    "new_path": "src/main.rs",
    "hunks": [
      {
        "old_start": 1,
        "old_lines": 3,
        "new_start": 1,
        "new_lines": 4,
        "line_chunks": [
          {
            "line_type": "Context",
            "content": "fn main() {",
            "new_line_number": 1
          },
          {
            "line_type": "Added",
            "content": "    println!(\"Reviewed diff\");",
            "new_line_number": 2
          }
        ]
      }
    ]
  }
]
```

## Architecture & Design Philosophy

The codebase adheres to core operational rules designed for enterprise pipelines:
1. **Decoupled Engines:** The `GitProvider` trait completely decouples git buffer acquisition from parsing logic, enabling testability and future backend plugins.
2. **Graceful Failures:** Enforces strict diagnostic messaging and deterministic status exit codes (`ExitCode::SUCCESS` / `ExitCode::FAILURE`).

## License

This project is licensed under either of the following licenses, at your option:
- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)
