# AI PR Reviewer (AI-Debt Governor)

## 1. Project Overview
The **AI PR Reviewer** is a dedicated command-line tool and automated CI gatekeeper designed to tackle the massive surge in pull request review times caused by rapid AI code generation. By analyzing diffs specifically for "AI hallmarks"—such as hallucinated variables, unhandled edge cases, complex uncommented logic, and insecure API patterns—it prevents low-quality code from overwhelming human maintainers.

## 2. Core Architecture & Philosophy
- **Language:** Rust (for maximum speed, safety, and single-binary portability).
- **Design Pattern:** The tool operates as a multi-stage static analysis pipeline augmented by specialized LLM heuristic checks.
- **Resilience:** Implements strict timeouts, streaming responses, and structured JSON outputs for seamless integration into GitHub Actions.

## 3. Directory Structure
```text
ai-pr-reviewer/
├── .agents/
│   ├── rules/          # Contextual agent guidelines and coding standards
│   └── workflows/      # Multi-step automated execution pipelines
├── src/                # Rust source code
├── Cargo.toml          # Package manifest
└── plan.md             # Living architecture document
```

## 4. Implementation Phases

### Phase 1: Foundation & Git Interception
- [x] Scaffold the Rust CLI library executable using `clap`.
- [x] Configure `Cargo.toml` with enterprise metadata, optimization profiles, and exclusion rules.
- [x] Establish root Git repository hygiene boundaries (`.gitignore`).
- [x] Implement local Git diff parsing to extract added and modified line blocks cleanly.

### Phase 2: Engine Integration
- Abstract the review engine interface to support both lightweight static pattern matching and API-driven LLM evaluation backends.
- Define structured diagnostic reporting models using `serde`.

### Phase 3: CI/CD Packaging & Deployment
- Package the compiled application with optimized exclusion rules.
- Author Homebrew Formula (`Formula/ai-pr-reviewer.rb`) to allow zero-config native `brew install` taps.
- Publish a companion GitHub Action template for native repository workflow usage.
