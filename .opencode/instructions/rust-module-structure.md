# Rust Module Structure

Apply these rules when creating or reorganizing Rust production and test code.

## Choose Flat Or Directory Modules

- Keep one small, cohesive concept in one file, such as `src/domain/engine.rs`.
- When it needs another concept-specific production file or a separate unit-test file, convert it to a directory module with `mod.rs` as the entry point.
- Name child files by responsibility; let the module tree express ownership instead of filename prefixes.

Preferred multi-file layout:

```text
src/domain/
├── mod.rs
└── engine/
    ├── mod.rs
    ├── api.rs
    ├── runtime.rs
    ├── state.rs
    └── types.rs
```

Avoid flat prefix fan-out:

```text
src/domain/engine_api.rs
src/domain/engine_runtime.rs
src/domain/engine_state.rs
src/domain/engine_types.rs
```

## Keep A Clear Boundary

The parent module declares the feature once:

```rust
mod engine;

pub use engine::{Engine, EngineError};
```

The feature entry point owns its internals and stable API:

```rust
mod api;
mod runtime;
mod state;
mod types;

pub use api::Engine;
pub use types::EngineError;
```

- Keep implementation modules and items private by default.
- Use `pub(super)` for parent-only access and `pub(crate)` for crate-internal APIs.
- Use `pub` only for an intentional public API.
- Re-export stable types from `mod.rs`; callers must not depend on paths such as `engine::api` or `engine::types`.
- Avoid wildcard re-exports unless the entire child module intentionally forms the public API.

## Put Tests In The Right Tree

- Keep small unit tests inline behind `#[cfg(test)] mod tests { ... }`.
- For a separate unit-test file, replace `auth.rs` with `auth/mod.rs`, add `auth/tests.rs`, and declare it from `mod.rs`:

```rust
#[cfg(test)]
mod tests;
```

```text
src/domain/auth/
├── mod.rs
└── tests.rs
```

- In `tests.rs`, use `use super::*;` when private implementation details are under test.
- Use the neutral name `tests.rs`, never `auth_tests.rs` or a custom `#[path = ...]` workaround.
- Never keep both `auth.rs` and `auth/` for the same module.
- Put public-API integration tests in the crate-level `tests/` directory, named by behavior or domain, such as `tests/auth.rs`.
- Keep doctests beside the public item they document.

Avoid:

```text
src/domain/auth/auth_tests.rs
src/domain/auth_tests.rs
src/domain/auth.rs + src/domain/auth/tests.rs
```

## Scope

- Integration tests, generated sources, platform-specific implementations, and unrelated neighboring modules do not by themselves trigger directory conversion.
- Do not split a cohesive one-file module merely to satisfy this rule.
- Normalize a violating multi-file feature when adding to or reorganizing that feature.
- Do not migrate unrelated legacy modules during an otherwise isolated task.
