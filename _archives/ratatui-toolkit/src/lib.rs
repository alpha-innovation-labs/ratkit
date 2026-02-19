//! # ratatui-toolkit (deprecated)
//!
//! This crate has been renamed to [`ratkit`](https://crates.io/crates/ratkit).
//! Please migrate by replacing:
//!
//! - `ratatui-toolkit` -> `ratkit` in `Cargo.toml`
//! - `ratatui_toolkit` -> `ratkit` in Rust imports
//!
//! This crate is now only a compatibility shim that re-exports `ratkit`.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[deprecated(
    since = "0.2.6",
    note = "crate `ratatui-toolkit` was renamed to `ratkit`; switch dependencies and imports to `ratkit`"
)]
pub use ratkit::*;

#[deprecated(
    since = "0.2.6",
    note = "module `ratatui_toolkit::prelude` moved to `ratkit::prelude`"
)]
pub mod prelude {
    pub use ratkit::prelude::*;
}

#[deprecated(
    since = "0.2.6",
    note = "crate `ratatui-toolkit` was renamed to `ratkit`; this shim will be removed in a future release"
)]
pub const DEPRECATION_NOTICE: &str =
    "ratatui-toolkit has been renamed to ratkit. Use `ratkit` instead.";
