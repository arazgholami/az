//! Example language plugin skeleton.
//!
//! Copy this file to a new name such as `todo.rs`, then register it in
//! `plugins/mod.rs`. A real plugin can expose any of these pieces:
//!
//! * `segments(line)` for syntax highlighting ranges.
//! * `completion_context(before, explicit)` to decide when completion opens.
//! * `completion_items(kind, ctx)` to return candidates.
//! * `symbols(line)` for quick open symbol support.
//!
//! The editor intentionally keeps plugins simple. A plugin is just Rust code
//! returning colored ranges and completion items. No external dependency is
//! required.

#[allow(dead_code)]
pub(crate) const DESCRIPTION: &str = "Tiny example plugin. Register a new module in plugins/mod.rs to enable it.";
