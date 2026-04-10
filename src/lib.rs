//! # ratkit - Core runtime for ratkit TUI components
//!
//! ratkit provides the core runtime (Runner + Layout Manager) and optional
//! re-exports for ratkit TUI components. Enable only the features you need or
//! use the `all` feature for the full bundle.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! ratkit = "0.2.14"
//! ```
//!
//! For the core runtime only:
//!
//! ```toml
//! ratkit = "0.2.14"
//! ```
//!
//! For selected components:
//!
//! ```toml
//! ratkit = { version = "0.2.14", default-features = false, features = ["tree-view", "toast"] }
//! ```
//!
//! For the full bundle:
//!
//! ```toml
//! ratkit = { version = "0.2.14", features = ["all"] }
//! ```
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use ratkit::prelude::*;
//! use ratatui::Frame;
//!
//! struct MyApp;
//!
//! impl CoordinatorApp for MyApp {
//!     fn on_event(&mut self, _event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
//!         Ok(CoordinatorAction::Continue)
//!     }
//!
//!     fn on_draw(&mut self, _frame: &mut Frame) {}
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     run(MyApp, RunnerConfig::default())
//! }
//! ```
//!
//! With widget features enabled, import UI primitives from `ratkit::widgets`.
//!
//! # Feature Flags
//!
//! - `default`: Core runtime only (Runner + Layout Manager)
//! - `all`: All widgets and services
//! - `full`: Alias for `all`
//! - `widgets`: All UI widgets
//! - `services`: All service components
//! - Individual feature flags for each component

#![doc(html_root_url = "https://docs.rs/ratkit/0.2.14")]
#![warn(missing_docs, clippy::cargo)]
#![cfg_attr(doc, cfg(feature = "docsrs"))]

mod coordinator;
mod error;
mod events;
mod focus;
mod layout;
mod mouse_router;
mod redraw_signal;
mod registry;
mod runner_helper;
mod types;

/// Core runtime pieces for ratkit.
pub mod core;

/// Feature-gated primitive widget modules.
pub mod primitives;

/// Feature-gated widget modules (includes primitives for backward compatibility).
pub mod widgets;

/// Feature-gated service modules.
pub mod services;

pub use runner_helper::{run, run_with_diagnostics};

pub use core::{
    CoordinatorAction, CoordinatorApp, CoordinatorConfig, CoordinatorEvent, Element, ElementHandle,
    ElementId, ElementMetadata, FocusRequest, KeyboardEvent, LayoutCoordinator, LayoutError,
    LayoutResult, MouseEvent, MouseRouterConfig, RedrawSignal, ResizeEvent, Runner, RunnerAction,
    RunnerConfig, RunnerEvent, TickEvent, Visibility,
};

/// Runner-first imports for applications.
pub mod prelude {
    pub use crate::{
        run, run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorConfig,
        CoordinatorEvent, KeyboardEvent, LayoutResult, MouseEvent, MouseRouterConfig, ResizeEvent,
        Runner, RunnerAction, RunnerConfig, RunnerEvent, TickEvent,
    };
}
