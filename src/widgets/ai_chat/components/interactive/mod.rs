//! Interactive components module for AI Chat widget.
//!
//! This module contains interactive UI components for the AI Chat widget:
//! - [`RevertBanner`]: Banner for showing reverted messages with diff stats
//!
//! # Example
//!
//! ```rust,ignore
//! use ratkit::widgets::ai_chat::components::interactive::RevertBanner;
//!
//! let banner = RevertBanner::new()
//!     .with_reverted_count(3)
//!     .with_diff(10, 5)
//!     .hovered(true);
//! ```

pub mod revert_banner;

pub use revert_banner::{DiffStats, RevertBanner, RevertBannerRenderer};
