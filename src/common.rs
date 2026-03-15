//! Common re-exports for simplified imports across modules
//!
//! This module provides centralized access to commonly used types
//! to reduce repetitive imports throughout the codebase.
//!
//! ## Usage
//! Instead of writing:
//! ```ignore
//! use nannou::prelude::*;
//! use nannou_egui::egui;
//! use serde::{Deserialize, Serialize};
//! ```
//!
//! Simply write:
//! ```ignore
//! use crate::common::*;
//! ```

// ============================================================================
// External Crate Re-exports
// ============================================================================

pub use nannou::prelude::*;
pub use nannou_egui::egui;

// ============================================================================
// Serialization
// ============================================================================

pub use anyhow::{Error, Result};
pub use serde::{Deserialize, Serialize};
pub use serde_json;

// ============================================================================
// Enums and Iteration
// ============================================================================

pub use strum::IntoEnumIterator;
pub use strum_macros::{Display, EnumIter};
