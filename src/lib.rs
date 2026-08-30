//! Domain-neutral reference runtime for CF-ACP-000.
//!
//! The core modules contain no infrastructure- or language-specific equations.
//! Domain profiles live under `profiles` and supply their own state, evolution,
//! adaptation, observation, and optional counterfactual semantics.

pub mod corpus_bridge_v001;
pub mod counterfactual;
pub mod geometry;
pub mod model;
pub mod profiles;
pub mod teacher_bridge;
pub mod teacher_bridge_v002;
pub mod teacher_bridge_v003;
pub mod teacher_bridge_v004;
pub mod teacher_bridge_v005;

pub use counterfactual::{
    binary_survival, mean_recovery_margin, recovery_margin, CounterfactualProfile,
};
pub use geometry::{pullback_metric, DifferentialResponse};
pub use model::{AdaptiveContinuationModel, StateRoles};
