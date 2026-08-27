use crate::{AdaptiveContinuationModel, StateRoles};

use super::language::{
    LanguageError, LanguageInput, LanguageObservationProfile, LanguageResponse, SurfaceSymbol,
};
use super::language_v2::{
    CohfieldLanguageModelV2, InternalEquivalenceProfile, LanguageStateV2,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ConsequenceEquivalenceAssessment {
    pub epoch: u64,
    pub left: SurfaceSymbol,
    pub right: SurfaceSymbol,
    pub profile: InternalEquivalenceProfile,
    pub measured_distance: f64,
    pub equivalent: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageRelationalConfigurationV3 {
    pub sequential: [[f64; 4]; 4],
    pub active_consequence_equivalence: [[bool; 4]; 4],
    pub active_profile: Option<InternalEquivalenceProfile>,
    pub assessment_history: Vec<ConsequenceEquivalenceAssessment>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageStateV3 {
    pub x: [f64; 4],
    pub theta: [f64; 4],
    pub relational: LanguageRelationalConfigurationV3,
}

impl LanguageStateV3 {
    pub fn initial() -> Self {
        Self {
            x: [0.0; 4],
            theta: [1.0; 4],
            relational: LanguageRelationalConfigurationV3 {
                sequential: [[0.0; 4]; 4],
                active_consequence_equivalence: [[false; 4]; 4],
                active_profile: None,
                assessment_history: Vec::new(),
            },
        }
    }

    pub fn from_v2_without_assessments(state: &LanguageStateV2) -> Self {
        Self {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV3 {
                sequential: state.relational.sequential,
                active_consequence_equivalence: [[false; 4]; 4],
                active_profile: None,
                assessment_history: Vec::new(),
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
pub enum LanguageExperienceV3 {
    Sequential {
        predecessor: Option<SurfaceSymbol>,
        current: SurfaceSymbol,
    },
    AssessConsequenceEquivalence(InternalEquivalenceProfile),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohfieldLanguageModelV3 {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub equivalence_coupling: f64,
}

impl Default for CohfieldLanguageModelV3 {
    fn default() -> Self {
        let v2 = CohfieldLanguageModelV2::default();
        Self {
            beta: v2.beta,
            input_gain: v2.input_gain,
            relational_gain: v2.relational_gain,
            psi_decay: v2.psi_decay,
            psi_gain: v2.psi_gain,
            equivalence_coupling: v2.equivalence_coupling,
        }
    }
}

impl CohfieldLanguageModelV3 {
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

    fn valid_state(&self, state: &LanguageStateV3) -> bool {
        state.x.iter().all(|value| value.is_finite())
            && state.theta.iter().all(|value| value.is_finite())
            && state
                .relational
                .sequential
                .iter()
                .flat_map(|row| row.iter())
                .all(|value| value.is_finite())
            && state
                .relational
                .assessment_history
                .iter()
                .all(|record| record.measured_distance.is_finite())
            && state.theta == [1.0; 4]
    }

    fn valid_profile(profile: InternalEquivalenceProfile) -> bool {
        profile.epsilon.is_finite() && profile.epsilon >= 0.0
    }

    fn step(
        &self,
        state: &LanguageStateV3,
        input: &LanguageInput,
    ) -> Result<LanguageStateV3, LanguageError> {
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
                let equivalence = if state.relational.active_consequence_equivalence[source][target]
                {
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

    fn consequence_signature_without_active_equivalence(
        &self,
        state: &LanguageStateV3,
        symbol: SurfaceSymbol,
        profile: InternalEquivalenceProfile,
    ) -> Result<Vec<f64>, LanguageError> {
        if !Self::valid_profile(profile) {
            return Err(LanguageError::InvalidParameter);
        }

        let mut witness = LanguageStateV3::equalized_from(state);
        witness.relational.active_consequence_equivalence = [[false; 4]; 4];
        witness.relational.active_profile = None;

        witness = self.step(&witness, &LanguageInput::symbol(symbol))?;
        let mut out = Vec::with_capacity((1 + profile.continuation_steps) * 2);
        for projected in profile.projection {
            out.push(witness.x[projected.index()]);
        }

        for _ in 0..profile.continuation_steps {
            witness = self.step(&witness, &LanguageInput::zero())?;
            for projected in profile.projection {
                out.push(witness.x[projected.index()]);
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

    fn assess_consequence_equivalence(
        &self,
        state: &LanguageStateV3,
        profile: InternalEquivalenceProfile,
    ) -> Result<LanguageStateV3, LanguageError> {
        if !self.valid_state(state) {
            return Err(LanguageError::InvalidState);
        }
        if !Self::valid_profile(profile) {
            return Err(LanguageError::InvalidParameter);
        }

        let signatures: Vec<Vec<f64>> = SurfaceSymbol::ALL
            .iter()
            .map(|&symbol| {
                self.consequence_signature_without_active_equivalence(state, symbol, profile)
            })
            .collect::<Result<_, _>>()?;

        let next_epoch = state
            .relational
            .assessment_history
            .last()
            .map(|record| record.epoch + 1)
            .unwrap_or(1);

        let mut next = state.clone();
        next.relational.active_consequence_equivalence = [[false; 4]; 4];
        next.relational.active_profile = Some(profile);

        for left in 0..SurfaceSymbol::ALL.len() {
            for right in (left + 1)..SurfaceSymbol::ALL.len() {
                let distance = Self::euclidean(&signatures[left], &signatures[right])
                    .ok_or(LanguageError::InvalidState)?;
                let equivalent = distance <= profile.epsilon;
                if equivalent {
                    next.relational.active_consequence_equivalence[left][right] = true;
                    next.relational.active_consequence_equivalence[right][left] = true;
                }
                next.relational
                    .assessment_history
                    .push(ConsequenceEquivalenceAssessment {
                        epoch: next_epoch,
                        left: SurfaceSymbol::ALL[left],
                        right: SurfaceSymbol::ALL[right],
                        profile,
                        measured_distance: distance,
                        equivalent,
                    });
            }
        }

        Ok(next)
    }

    pub fn expose(
        &self,
        initial: &LanguageStateV3,
        pattern: &[SurfaceSymbol],
        repeats: usize,
    ) -> Result<LanguageStateV3, LanguageError> {
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
                    &LanguageExperienceV3::Sequential {
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

impl AdaptiveContinuationModel for CohfieldLanguageModelV3 {
    type State = LanguageStateV3;
    type Fast = [f64; 4];
    type LocalCondition = [f64; 4];
    type RelationalConfiguration = LanguageRelationalConfigurationV3;
    type Input = LanguageInput;
    type Experience = LanguageExperienceV3;
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
            LanguageExperienceV3::Sequential {
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
            LanguageExperienceV3::AssessConsequenceEquivalence(profile) => {
                self.assess_consequence_equivalence(state, profile)
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
            let mut local = LanguageStateV3::equalized_from(state);
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
