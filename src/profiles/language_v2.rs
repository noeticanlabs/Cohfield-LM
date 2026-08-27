use crate::{AdaptiveContinuationModel, StateRoles};

use super::language::{
    LanguageError, LanguageInput, LanguageObservationProfile, LanguageResponse, LanguageState,
    SurfaceSymbol,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageRelationalConfigurationV2 {
    pub sequential: [[f64; 4]; 4],
    pub consequence_equivalence: [[bool; 4]; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageStateV2 {
    pub x: [f64; 4],
    pub theta: [f64; 4],
    pub relational: LanguageRelationalConfigurationV2,
}

impl LanguageStateV2 {
    pub fn initial() -> Self {
        Self {
            x: [0.0; 4],
            theta: [1.0; 4],
            relational: LanguageRelationalConfigurationV2 {
                sequential: [[0.0; 4]; 4],
                consequence_equivalence: [[false; 4]; 4],
            },
        }
    }

    pub fn from_v1(state: &LanguageState) -> Self {
        Self {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV2 {
                sequential: state.psi,
                consequence_equivalence: [[false; 4]; 4],
            },
        }
    }

    pub fn equalized_from(state: &Self) -> Self {
        let mut next = state.clone();
        next.x = [0.0; 4];
        next.theta = [1.0; 4];
        next
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InternalEquivalenceProfile {
    pub continuation_steps: usize,
    pub projection: [SurfaceSymbol; 2],
    pub epsilon: f64,
}

impl InternalEquivalenceProfile {
    pub fn cf_lm_009() -> Self {
        Self {
            continuation_steps: 4,
            projection: [SurfaceSymbol::A, SurfaceSymbol::B],
            epsilon: 1.0e-12,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LanguageExperienceV2 {
    Sequential {
        predecessor: Option<SurfaceSymbol>,
        current: SurfaceSymbol,
    },
    InternalizeConsequenceEquivalence(InternalEquivalenceProfile),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohfieldLanguageModelV2 {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub equivalence_coupling: f64,
}

impl Default for CohfieldLanguageModelV2 {
    fn default() -> Self {
        Self {
            beta: 0.50,
            input_gain: 0.50,
            relational_gain: 0.20,
            psi_decay: 0.02,
            psi_gain: 0.08,
            equivalence_coupling: 1.0,
        }
    }
}

impl CohfieldLanguageModelV2 {
    fn valid_parameters(&self) -> bool {
        self.beta.is_finite()
            && self.input_gain.is_finite()
            && self.relational_gain.is_finite()
            && self.psi_decay.is_finite()
            && self.psi_gain.is_finite()
            && self.equivalence_coupling.is_finite()
            && (0.0..=1.0).contains(&self.psi_decay)
            && self.psi_gain >= 0.0
            && self.equivalence_coupling >= 0.0
    }

    fn valid_state(&self, state: &LanguageStateV2) -> bool {
        state.x.iter().all(|value| value.is_finite())
            && state.theta.iter().all(|value| value.is_finite())
            && state
                .relational
                .sequential
                .iter()
                .flat_map(|row| row.iter())
                .all(|value| value.is_finite())
            && state.theta == [1.0; 4]
    }

    fn step(
        &self,
        state: &LanguageStateV2,
        input: &LanguageInput,
    ) -> Result<LanguageStateV2, LanguageError> {
        if !self.valid_parameters() {
            return Err(LanguageError::InvalidParameter);
        }
        if !self.valid_state(state) || input.activity.iter().any(|value| !value.is_finite()) {
            return Err(LanguageError::InvalidState);
        }

        let mut next = state.clone();
        let mut relational = [0.0; 4];
        for (target, value) in relational.iter_mut().enumerate() {
            for (source, source_activity) in state.x.iter().enumerate() {
                let sequential = state.relational.sequential[source][target];
                let equivalence = if state.relational.consequence_equivalence[source][target] {
                    self.equivalence_coupling
                } else {
                    0.0
                };
                *value += (sequential + equivalence) * source_activity;
            }
        }

        for (index, value) in next.x.iter_mut().enumerate() {
            *value = self.beta * state.x[index]
                + self.input_gain * input.activity[index]
                + self.relational_gain * relational[index];
        }

        Ok(next)
    }

    fn consequence_signature(
        &self,
        state: &LanguageStateV2,
        symbol: SurfaceSymbol,
        profile: InternalEquivalenceProfile,
    ) -> Result<Vec<f64>, LanguageError> {
        if !profile.epsilon.is_finite() || profile.epsilon < 0.0 {
            return Err(LanguageError::InvalidParameter);
        }

        let mut local = LanguageStateV2::equalized_from(state);
        local = self.step(&local, &LanguageInput::symbol(symbol))?;

        let mut out = Vec::with_capacity((1 + profile.continuation_steps) * 2);
        for projected in profile.projection {
            out.push(local.x[projected.index()]);
        }

        for _ in 0..profile.continuation_steps {
            local = self.step(&local, &LanguageInput::zero())?;
            for projected in profile.projection {
                out.push(local.x[projected.index()]);
            }
        }

        Ok(out)
    }

    fn euclidean(left: &[f64], right: &[f64]) -> Option<f64> {
        if left.is_empty() || left.len() != right.len() {
            return None;
        }
        Some(
            left.iter()
                .zip(right.iter())
                .map(|(a, b)| (a - b) * (a - b))
                .sum::<f64>()
                .sqrt(),
        )
    }

    fn internalize_consequence_equivalence(
        &self,
        state: &LanguageStateV2,
        profile: InternalEquivalenceProfile,
    ) -> Result<LanguageStateV2, LanguageError> {
        if !self.valid_state(state) {
            return Err(LanguageError::InvalidState);
        }

        let signatures: Vec<Vec<f64>> = SurfaceSymbol::ALL
            .iter()
            .map(|&symbol| self.consequence_signature(state, symbol, profile))
            .collect::<Result<_, _>>()?;

        let mut next = state.clone();
        for left in 0..SurfaceSymbol::ALL.len() {
            for right in (left + 1)..SurfaceSymbol::ALL.len() {
                let distance = Self::euclidean(&signatures[left], &signatures[right])
                    .ok_or(LanguageError::InvalidState)?;
                if distance <= profile.epsilon {
                    next.relational.consequence_equivalence[left][right] = true;
                    next.relational.consequence_equivalence[right][left] = true;
                }
            }
        }

        Ok(next)
    }

    pub fn expose(
        &self,
        initial: &LanguageStateV2,
        pattern: &[SurfaceSymbol],
        repeats: usize,
    ) -> Result<LanguageStateV2, LanguageError> {
        if pattern.is_empty() || repeats == 0 {
            return Err(LanguageError::EmptyExposure);
        }
        if !self.valid_state(initial) {
            return Err(LanguageError::InvalidState);
        }

        let mut state = initial.clone();
        let mut predecessor = None;
        for _ in 0..repeats {
            for &symbol in pattern {
                state = self.step(&state, &LanguageInput::symbol(symbol))?;
                state = self.adapt(
                    &state,
                    &LanguageExperienceV2::Sequential {
                        predecessor,
                        current: symbol,
                    },
                )?;
                predecessor = Some(symbol);
            }
        }
        Ok(state)
    }
}

impl AdaptiveContinuationModel for CohfieldLanguageModelV2 {
    type State = LanguageStateV2;
    type Fast = [f64; 4];
    type LocalCondition = [f64; 4];
    type RelationalConfiguration = LanguageRelationalConfigurationV2;
    type Input = LanguageInput;
    type Experience = LanguageExperienceV2;
    type ObservationProfile = LanguageObservationProfile;
    type Response = LanguageResponse;
    type Error = LanguageError;

    fn roles(
        &self,
        state: &Self::State,
    ) -> StateRoles<Self::Fast, Self::LocalCondition, Self::RelationalConfiguration> {
        StateRoles {
            fast: state.x,
            local_condition: state.theta,
            relational_configuration: state.relational.clone(),
        }
    }

    fn evolve(
        &self,
        state: &Self::State,
        input: &Self::Input,
        horizon: f64,
    ) -> Result<Self::State, Self::Error> {
        if !horizon.is_finite() || horizon < 0.0 || horizon.fract() != 0.0 {
            return Err(LanguageError::InvalidHorizon);
        }
        if horizon == 0.0 {
            return Ok(state.clone());
        }

        let steps = horizon as usize;
        let mut next = self.step(state, input)?;
        for _ in 1..steps {
            next = self.step(&next, &LanguageInput::zero())?;
        }
        Ok(next)
    }

    fn adapt(
        &self,
        state: &Self::State,
        experience: &Self::Experience,
    ) -> Result<Self::State, Self::Error> {
        if !self.valid_parameters() {
            return Err(LanguageError::InvalidParameter);
        }
        if !self.valid_state(state) {
            return Err(LanguageError::InvalidState);
        }

        match *experience {
            LanguageExperienceV2::Sequential {
                predecessor,
                current,
            } => {
                let mut next = state.clone();
                for row in &mut next.relational.sequential {
                    for value in row {
                        *value *= 1.0 - self.psi_decay;
                    }
                }
                if let Some(predecessor) = predecessor {
                    next.relational.sequential[predecessor.index()][current.index()] +=
                        self.psi_gain;
                }
                Ok(next)
            }
            LanguageExperienceV2::InternalizeConsequenceEquivalence(profile) => {
                self.internalize_consequence_equivalence(state, profile)
            }
        }
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        if !self.valid_parameters() {
            return Err(LanguageError::InvalidParameter);
        }
        if !self.valid_state(state) {
            return Err(LanguageError::InvalidState);
        }
        if profile.probes.is_empty() {
            return Err(LanguageError::EmptyProbeFamily);
        }

        let mut vectors =
            Vec::with_capacity(profile.probes.len() * (2 + profile.continuation_steps));
        for probe in &profile.probes {
            let mut local = LanguageStateV2::equalized_from(state);
            for &symbol in probe {
                local = self.step(&local, &LanguageInput::symbol(symbol))?;
                vectors.push(local.x);
            }
            for _ in 0..profile.continuation_steps {
                local = self.step(&local, &LanguageInput::zero())?;
                vectors.push(local.x);
            }
        }

        Ok(LanguageResponse { vectors })
    }
}
