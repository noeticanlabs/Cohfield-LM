//! Domain-neutral reference runtime for CF-ACP-000.
//!
//! This crate intentionally contains no infrastructure- or language-specific
//! equations. Domain profiles supply state, evolution, adaptation, observation,
//! and optional counterfactual semantics.

pub mod counterfactual;
pub mod geometry;
pub mod model;

pub use counterfactual::{
    binary_survival, mean_recovery_margin, recovery_margin, CounterfactualProfile,
};
pub use geometry::{pullback_metric, DifferentialResponse};
pub use model::{AdaptiveContinuationModel, StateRoles};
