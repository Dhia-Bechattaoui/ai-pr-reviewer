# Core Architectural Guidelines

1. **Decoupled Engines:** Always isolate Git file parsing logic from LLM/static evaluation backends using strict Rust traits.
2. **Deterministic Outputs:** Ensure all diagnostic reviews can be streamed as valid JSON objects for automated parser integration.
3. **Graceful Failures:** Fail open or return explicit operational diagnostics if upstream APIs experience rate limits or network timeouts.
