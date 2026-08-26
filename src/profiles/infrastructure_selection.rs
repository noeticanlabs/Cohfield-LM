//! Downstream retention/selection utilities for the infrastructure profile.
//!
//! This module is not part of the domain-neutral CF-ACP core. It exists to
//! reconstruct the v0.08-v0.10 retention mathematics without promoting an
//! endogenous score into CohBit valuation, admissibility, or authority.

#[derive(Clone, Debug, PartialEq)]
pub struct AffineForgettingProfile {
    /// Lowest score observed/declared by the profile.
    pub score_min: f64,
    /// Highest score observed/declared by the profile.
    pub score_max: f64,
    /// Forgetting rate assigned to the lowest score.
    pub forgetting_at_min: f64,
    /// Forgetting rate assigned to the highest score.
    pub forgetting_at_max: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RetentionError {
    NonFinite,
    InvalidScoreRange,
    InvalidForgettingRate,
    ScoreOutsideProfile,
}

impl AffineForgettingProfile {
    pub fn validate(&self) -> Result<(), RetentionError> {
        if !self.score_min.is_finite()
            || !self.score_max.is_finite()
            || !self.forgetting_at_min.is_finite()
            || !self.forgetting_at_max.is_finite()
        {
            return Err(RetentionError::NonFinite);
        }
        if self.score_max <= self.score_min {
            return Err(RetentionError::InvalidScoreRange);
        }
        if !(0.0..=1.0).contains(&self.forgetting_at_min)
            || !(0.0..=1.0).contains(&self.forgetting_at_max)
        {
            return Err(RetentionError::InvalidForgettingRate);
        }
        Ok(())
    }

    /// Linearly interpolate forgetting rate across the declared score interval.
    ///
    /// No claim is made that this profile is universal. In v0.10 the higher
    /// endogenous recovery-margin score was mapped to a lower forgetting rate.
    pub fn forgetting_rate(&self, score: f64) -> Result<f64, RetentionError> {
        self.validate()?;
        if !score.is_finite() {
            return Err(RetentionError::NonFinite);
        }
        if score < self.score_min || score > self.score_max {
            return Err(RetentionError::ScoreOutsideProfile);
        }

        let t = (score - self.score_min) / (self.score_max - self.score_min);
        Ok(self.forgetting_at_min
            + t * (self.forgetting_at_max - self.forgetting_at_min))
    }

    pub fn retention_ratio(&self, score: f64, steps: usize) -> Result<f64, RetentionError> {
        let rate = self.forgetting_rate(score)?;
        retention_ratio(rate, steps)
    }
}

/// Uniform multiplicative retention after `steps` forgetting steps.
pub fn retention_ratio(forgetting_rate: f64, steps: usize) -> Result<f64, RetentionError> {
    if !forgetting_rate.is_finite() {
        return Err(RetentionError::NonFinite);
    }
    if !(0.0..=1.0).contains(&forgetting_rate) {
        return Err(RetentionError::InvalidForgettingRate);
    }
    Ok((1.0 - forgetting_rate).powi(steps as i32))
}

pub fn frobenius_norm(matrix: &[[f64; 3]; 3]) -> Result<f64, RetentionError> {
    let mut sum = 0.0;
    for row in matrix {
        for value in row {
            if !value.is_finite() {
                return Err(RetentionError::NonFinite);
            }
            sum += value * value;
        }
    }
    Ok(sum.sqrt())
}

/// Apply uniform multiplicative forgetting to relational configuration.
pub fn decay_relational_configuration(
    psi: &[[f64; 3]; 3],
    forgetting_rate: f64,
    steps: usize,
) -> Result<[[f64; 3]; 3], RetentionError> {
    let ratio = retention_ratio(forgetting_rate, steps)?;
    let mut out = [[0.0; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            *value = psi[i][j] * ratio;
        }
    }
    Ok(out)
}
