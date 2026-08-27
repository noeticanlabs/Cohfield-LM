use crate::{AdaptiveContinuationModel, StateRoles};

use super::language::{
    LanguageError, LanguageInput, LanguageObservationProfile, LanguageResponse, SurfaceSymbol,
};
use super::language_v2::{CohfieldLanguageModelV2, InternalEquivalenceProfile};
use super::language_v3::{
    CohfieldLanguageModelV3, ConsequenceEquivalenceAssessment, LanguageStateV3,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageRelationalConfigurationV4 {
    pub sequential: [[f64; 4]; 4],
    pub selected_profile: Option<InternalEquivalenceProfile>,
    pub assessment_history: Vec<ConsequenceEquivalenceAssessment>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageStateV4 {
    pub x: [f64; 4],
    pub theta: [f64; 4],
    pub relational: LanguageRelationalConfigurationV4,
}

impl LanguageStateV4 {
    pub fn initial() -> Self {
        Self {
            x: [0.0; 4],
            theta: [1.0; 4],
            relational: LanguageRelationalConfigurationV4 {
                sequential: [[0.0; 4]; 4],
                selected_profile: None,
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

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageErrorV4 {
    Base(LanguageError),
    ProfileNotAssessed,
    IncompleteAssessmentHistory,
    MigrationMismatch,
}

impl From<LanguageError> for LanguageErrorV4 {
    fn from(value: LanguageError) -> Self {
        Self::Base(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LanguageExperienceV4 {
    Sequential {
        predecessor: Option<SurfaceSymbol>,
        current: SurfaceSymbol,
    },
    AssessConsequenceEquivalence(InternalEquivalenceProfile),
    SelectConsequenceProfile(InternalEquivalenceProfile),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohfieldLanguageModelV4 {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub equivalence_coupling: f64,
}

impl Default for CohfieldLanguageModelV4 {
    fn default() -> Self {
        let v3 = CohfieldLanguageModelV3::default();
        Self {
            beta: v3.beta,
            input_gain: v3.input_gain,
            relational_gain: v3.relational_gain,
            psi_decay: v3.psi_decay,
            psi_gain: v3.psi_gain,
            equivalence_coupling: v3.equivalence_coupling,
        }
    }
}

impl CohfieldLanguageModelV4 {
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

    fn valid_profile(profile: InternalEquivalenceProfile) -> bool {
        profile.epsilon.is_finite() && profile.epsilon >= 0.0
    }

    fn valid_state(&self, state: &LanguageStateV4) -> bool {
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

    fn latest_profile_matrix_from_history(
        history: &[ConsequenceEquivalenceAssessment],
        profile: InternalEquivalenceProfile,
    ) -> Result<Option<[[bool; 4]; 4]>, LanguageErrorV4> {
        let Some(latest_epoch) = history
            .iter()
            .filter(|record| record.profile == profile)
            .map(|record| record.epoch)
            .max()
        else {
            return Ok(None);
        };

        let records: Vec<_> = history
            .iter()
            .filter(|record| record.epoch == latest_epoch && record.profile == profile)
            .collect();
        if records.len() != 6 {
            return Err(LanguageErrorV4::IncompleteAssessmentHistory);
        }

        let mut seen = [[false; 4]; 4];
        let mut matrix = [[false; 4]; 4];
        for record in records {
            let left = record.left.index();
            let right = record.right.index();
            if left == right || seen[left][right] || seen[right][left] {
                return Err(LanguageErrorV4::IncompleteAssessmentHistory);
            }
            seen[left][right] = true;
            seen[right][left] = true;
            if record.equivalent {
                matrix[left][right] = true;
                matrix[right][left] = true;
            }
        }

        let expected_pairs = SurfaceSymbol::ALL.len() * (SurfaceSymbol::ALL.len() - 1) / 2;
        let observed_pairs = (0..SurfaceSymbol::ALL.len())
            .map(|left| {
                ((left + 1)..SurfaceSymbol::ALL.len())
                    .filter(|&right| seen[left][right])
                    .count()
            })
            .sum::<usize>();
        if observed_pairs != expected_pairs {
            return Err(LanguageErrorV4::IncompleteAssessmentHistory);
        }

        Ok(Some(matrix))
    }

    pub fn equivalence_for_profile(
        &self,
        state: &LanguageStateV4,
        profile: InternalEquivalenceProfile,
    ) -> Result<Option<[[bool; 4]; 4]>, LanguageErrorV4> {
        if !Self::valid_profile(profile) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidParameter));
        }
        Self::latest_profile_matrix_from_history(&state.relational.assessment_history, profile)
    }

    pub fn selected_equivalence(
        &self,
        state: &LanguageStateV4,
    ) -> Result<[[bool; 4]; 4], LanguageErrorV4> {
        match state.relational.selected_profile {
            Some(profile) => self
                .equivalence_for_profile(state, profile)?
                .ok_or(LanguageErrorV4::ProfileNotAssessed),
            None => Ok([[false; 4]; 4]),
        }
    }

    pub fn migrate_from_v3(
        &self,
        state: &LanguageStateV3,
    ) -> Result<LanguageStateV4, LanguageErrorV4> {
        let migrated = LanguageStateV4 {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV4 {
                sequential: state.relational.sequential,
                selected_profile: state.relational.active_profile,
                assessment_history: state.relational.assessment_history.clone(),
            },
        };

        if let Some(profile) = state.relational.active_profile {
            let derived = self
                .equivalence_for_profile(&migrated, profile)?
                .ok_or(LanguageErrorV4::MigrationMismatch)?;
            if derived != state.relational.active_consequence_equivalence {
                return Err(LanguageErrorV4::MigrationMismatch);
            }
        }

        Ok(migrated)
    }

    fn step(
        &self,
        state: &LanguageStateV4,
        input: &LanguageInput,
    ) -> Result<LanguageStateV4, LanguageErrorV4> {
        if !self.valid_parameters() {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidParameter));
        }
        if !self.valid_state(state) || input.activity.iter().any(|value| !value.is_finite()) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidState));
        }

        let equivalence = self.selected_equivalence(state)?;
        let mut next = state.clone();
        let mut relational = [0.0; 4];
        for (target, value) in relational.iter_mut().enumerate() {
            for (source, source_activity) in state.x.iter().enumerate() {
                let sequential = state.relational.sequential[source][target];
                let equivalence_coupling = if equivalence[source][target] {
                    self.equivalence_coupling
                } else {
                    0.0
                };
                *value += (sequential + equivalence_coupling) * source_activity;
            }
        }

        for (index, value) in next.x.iter_mut().enumerate() {
            *value = self.beta * state.x[index]
                + self.input_gain * input.activity[index]
                + self.relational_gain * relational[index];
        }

        Ok(next)
    }

    fn consequence_signature_without_selected_equivalence(
        &self,
        state: &LanguageStateV4,
        symbol: SurfaceSymbol,
        profile: InternalEquivalenceProfile,
    ) -> Result<Vec<f64>, LanguageErrorV4> {
        if !Self::valid_profile(profile) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidParameter));
        }

        let mut witness = LanguageStateV4::equalized_from(state);
        witness.relational.selected_profile = None;
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
        state: &LanguageStateV4,
        profile: InternalEquivalenceProfile,
    ) -> Result<LanguageStateV4, LanguageErrorV4> {
        if !self.valid_state(state) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidState));
        }
        if !Self::valid_profile(profile) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidParameter));
        }

        let signatures: Vec<Vec<f64>> = SurfaceSymbol::ALL
            .iter()
            .map(|&symbol| {
                self.consequence_signature_without_selected_equivalence(state, symbol, profile)
            })
            .collect::<Result<_, _>>()?;

        let next_epoch = state
            .relational
            .assessment_history
            .last()
            .map(|record| record.epoch + 1)
            .unwrap_or(1);

        let mut next = state.clone();
        for left in 0..SurfaceSymbol::ALL.len() {
            for right in (left + 1)..SurfaceSymbol::ALL.len() {
                let distance = Self::euclidean(&signatures[left], &signatures[right])
                    .ok_or(LanguageErrorV4::Base(LanguageError::InvalidState))?;
                next.relational
                    .assessment_history
                    .push(ConsequenceEquivalenceAssessment {
                        epoch: next_epoch,
                        left: SurfaceSymbol::ALL[left],
                        right: SurfaceSymbol::ALL[right],
                        profile,
                        measured_distance: distance,
                        equivalent: distance <= profile.epsilon,
                    });
            }
        }
        Ok(next)
    }

    fn select_profile(
        &self,
        state: &LanguageStateV4,
        profile: InternalEquivalenceProfile,
    ) -> Result<LanguageStateV4, LanguageErrorV4> {
        if !self.valid_state(state) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidState));
        }
        self.equivalence_for_profile(state, profile)?
            .ok_or(LanguageErrorV4::ProfileNotAssessed)?;

        let mut next = state.clone();
        next.relational.selected_profile = Some(profile);
        Ok(next)
    }

    pub fn expose(
        &self,
        initial: &LanguageStateV4,
        pattern: &[SurfaceSymbol],
        repeats: usize,
    ) -> Result<LanguageStateV4, LanguageErrorV4> {
        if pattern.is_empty() || repeats == 0 {
            return Err(LanguageErrorV4::Base(LanguageError::EmptyExposure));
        }
        if !self.valid_state(initial) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidState));
        }

        let mut state = initial.clone();
        let mut predecessor = None;
        for _ in 0..repeats {
            for &symbol in pattern {
                state = self.step(&state, &LanguageInput::symbol(symbol))?;
                state = self.adapt(
                    &state,
                    &LanguageExperienceV4::Sequential {
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

impl AdaptiveContinuationModel for CohfieldLanguageModelV4 {
    type State = LanguageStateV4;
    type Fast = [f64; 4];
    type LocalCondition = [f64; 4];
    type RelationalConfiguration = LanguageRelationalConfigurationV4;
    type Input = LanguageInput;
    type Experience = LanguageExperienceV4;
    type ObservationProfile = LanguageObservationProfile;
    type Response = LanguageResponse;
    type Error = LanguageErrorV4;

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
            return Err(LanguageErrorV4::Base(LanguageError::InvalidHorizon));
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
            return Err(LanguageErrorV4::Base(LanguageError::InvalidParameter));
        }
        if !self.valid_state(state) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidState));
        }

        match *experience {
            LanguageExperienceV4::Sequential {
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
            LanguageExperienceV4::AssessConsequenceEquivalence(profile) => {
                self.assess_consequence_equivalence(state, profile)
            }
            LanguageExperienceV4::SelectConsequenceProfile(profile) => {
                self.select_profile(state, profile)
            }
        }
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        if !self.valid_parameters() {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidParameter));
        }
        if !self.valid_state(state) {
            return Err(LanguageErrorV4::Base(LanguageError::InvalidState));
        }
        if profile.probes.is_empty() {
            return Err(LanguageErrorV4::Base(LanguageError::EmptyProbeFamily));
        }

        let mut vectors =
            Vec::with_capacity(profile.probes.len() * (2 + profile.continuation_steps));
        for probe in &profile.probes {
            let mut local = LanguageStateV4::equalized_from(state);
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
