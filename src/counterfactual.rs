/// Optional counterfactual extension to CF-ACP-000.
pub trait CounterfactualProfile {
    type State;
    type Perturbation;
    type Trajectory;
    type Error;

    /// Produce one hypothetical continuation from the same reference state.
    fn rollout(
        &self,
        state: &Self::State,
        perturbation: &Self::Perturbation,
    ) -> Result<Self::Trajectory, Self::Error>;

    /// Domain-defined non-negative recovery observable r(trajectory).
    fn recovery_measure(&self, trajectory: &Self::Trajectory) -> f64;
}

/// v0.10 signed boundary margin: m = 1 - r / r_max.
pub fn recovery_margin(recovery: f64, recovery_max: f64) -> Option<f64> {
    if !recovery.is_finite() || !recovery_max.is_finite() || recovery_max <= 0.0 {
        return None;
    }
    Some(1.0 - recovery / recovery_max)
}

/// Binary survival is a lossy threshold projection of the signed margin.
pub fn binary_survival(margin: f64) -> bool {
    margin >= 0.0
}

/// Q_rm = mean_j m_j. Returns None for an empty or non-finite sample.
pub fn mean_recovery_margin(margins: &[f64]) -> Option<f64> {
    if margins.is_empty() || margins.iter().any(|m| !m.is_finite()) {
        return None;
    }
    Some(margins.iter().sum::<f64>() / margins.len() as f64)
}
