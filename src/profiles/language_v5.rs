use crate::{AdaptiveContinuationModel, StateRoles};

use super::language::{
    LanguageError, LanguageInput, LanguageObservationProfile, LanguageResponse, SurfaceSymbol,
};
use super::language_v2::InternalEquivalenceProfile;
use super::language_v3::ConsequenceEquivalenceAssessment;
use super::language_v4::{
    CohfieldLanguageModelV4, LanguageErrorV4, LanguageExperienceV4,
    LanguageRelationalConfigurationV4, LanguageStateV4,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ContextRecognitionRecordV5 {
    pub epoch: u64,
    pub cue: Vec<SurfaceSymbol>,
    pub activity: [f64; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfileContextScoreV5 {
    pub profile: InternalEquivalenceProfile,
    pub score: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContextSelectionRecordV5 {
    pub epoch: u64,
    pub context_epoch: u64,
    pub candidate_scores: Vec<ProfileContextScoreV5>,
    pub selected_profile: InternalEquivalenceProfile,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageRelationalConfigurationV5 {
    pub sequential: [[f64; 4]; 4],
    pub selected_profile: Option<InternalEquivalenceProfile>,
    pub assessment_history: Vec<ConsequenceEquivalenceAssessment>,
    pub current_context_epoch: Option<u64>,
    pub context_history: Vec<ContextRecognitionRecordV5>,
    pub selection_history: Vec<ContextSelectionRecordV5>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageStateV5 {
    pub x: [f64; 4],
    pub theta: [f64; 4],
    pub relational: LanguageRelationalConfigurationV5,
}

impl LanguageStateV5 {
    pub fn initial() -> Self {
        Self {
            x: [0.0; 4],
            theta: [1.0; 4],
            relational: LanguageRelationalConfigurationV5 {
                sequential: [[0.0; 4]; 4],
                selected_profile: None,
                assessment_history: Vec::new(),
                current_context_epoch: None,
                context_history: Vec::new(),
                selection_history: Vec::new(),
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
pub enum LanguageErrorV5 {
    BaseV4(LanguageErrorV4),
    EmptyContext,
    NoRecognizedContext,
    NoAssessedProfiles,
    UnsupportedContext,
    AmbiguousContext,
}

impl From<LanguageErrorV4> for LanguageErrorV5 {
    fn from(value: LanguageErrorV4) -> Self {
        Self::BaseV4(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageExperienceV5 {
    Sequential {
        predecessor: Option<SurfaceSymbol>,
        current: SurfaceSymbol,
    },
    AssessConsequenceEquivalence(InternalEquivalenceProfile),
    RecognizeContext(Vec<SurfaceSymbol>),
    InferConsequenceProfileFromContext,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohfieldLanguageModelV5 {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub equivalence_coupling: f64,
    pub minimum_context_score: f64,
    pub minimum_context_margin: f64,
}

impl Default for CohfieldLanguageModelV5 {
    fn default() -> Self {
        let v4 = CohfieldLanguageModelV4::default();
        Self {
            beta: v4.beta,
            input_gain: v4.input_gain,
            relational_gain: v4.relational_gain,
            psi_decay: v4.psi_decay,
            psi_gain: v4.psi_gain,
            equivalence_coupling: v4.equivalence_coupling,
            minimum_context_score: 0.50,
            minimum_context_margin: 0.25,
        }
    }
}

impl CohfieldLanguageModelV5 {
    fn v4_model(&self) -> CohfieldLanguageModelV4 {
        CohfieldLanguageModelV4 {
            beta: self.beta,
            input_gain: self.input_gain,
            relational_gain: self.relational_gain,
            psi_decay: self.psi_decay,
            psi_gain: self.psi_gain,
            equivalence_coupling: self.equivalence_coupling,
        }
    }

    fn valid_parameters(&self) -> bool {
        self.beta.is_finite()
            && self.input_gain.is_finite()
            && self.relational_gain.is_finite()
            && self.psi_decay.is_finite()
            && self.psi_gain.is_finite()
            && self.equivalence_coupling.is_finite()
            && self.minimum_context_score.is_finite()
            && self.minimum_context_margin.is_finite()
            && (0.0..=1.0).contains(&self.psi_decay)
            && self.psi_gain >= 0.0
            && self.equivalence_coupling >= 0.0
            && self.minimum_context_score >= 0.0
            && self.minimum_context_margin >= 0.0
    }

    fn valid_state(&self, state: &LanguageStateV5) -> bool {
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
            && state
                .relational
                .context_history
                .iter()
                .flat_map(|record| record.activity.iter())
                .all(|value| value.is_finite())
            && state
                .relational
                .selection_history
                .iter()
                .flat_map(|record| record.candidate_scores.iter())
                .all(|entry| entry.score.is_finite())
            && state.theta == [1.0; 4]
            && state.relational.current_context_epoch.is_none_or(|epoch| {
                state
                    .relational
                    .context_history
                    .iter()
                    .any(|record| record.epoch == epoch)
            })
    }

    fn to_v4_state(&self, state: &LanguageStateV5) -> LanguageStateV4 {
        LanguageStateV4 {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV4 {
                sequential: state.relational.sequential,
                selected_profile: state.relational.selected_profile,
                assessment_history: state.relational.assessment_history.clone(),
            },
        }
    }

    fn apply_v4_state(&self, state: &LanguageStateV5, v4: LanguageStateV4) -> LanguageStateV5 {
        let mut next = state.clone();
        next.x = v4.x;
        next.theta = v4.theta;
        next.relational.sequential = v4.relational.sequential;
        next.relational.selected_profile = v4.relational.selected_profile;
        next.relational.assessment_history = v4.relational.assessment_history;
        next
    }

    pub fn migrate_from_v4(&self, state: &LanguageStateV4) -> Result<LanguageStateV5, LanguageErrorV5> {
        let next = LanguageStateV5 {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV5 {
                sequential: state.relational.sequential,
                selected_profile: state.relational.selected_profile,
                assessment_history: state.relational.assessment_history.clone(),
                current_context_epoch: None,
                context_history: Vec::new(),
                selection_history: Vec::new(),
            },
        };
        if !self.valid_state(&next) {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidState,
            )));
        }
        Ok(next)
    }

    pub fn equivalence_for_profile(
        &self,
        state: &LanguageStateV5,
        profile: InternalEquivalenceProfile,
    ) -> Result<Option<[[bool; 4]; 4]>, LanguageErrorV5> {
        self.v4_model()
            .equivalence_for_profile(&self.to_v4_state(state), profile)
            .map_err(Into::into)
    }

    pub fn selected_equivalence(
        &self,
        state: &LanguageStateV5,
    ) -> Result<[[bool; 4]; 4], LanguageErrorV5> {
        self.v4_model()
            .selected_equivalence(&self.to_v4_state(state))
            .map_err(Into::into)
    }

    fn step(
        &self,
        state: &LanguageStateV5,
        input: &LanguageInput,
    ) -> Result<LanguageStateV5, LanguageErrorV5> {
        if !self.valid_parameters() {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidParameter,
            )));
        }
        if !self.valid_state(state) {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidState,
            )));
        }
        let v4 = self
            .v4_model()
            .evolve(&self.to_v4_state(state), input, 1.0)?;
        Ok(self.apply_v4_state(state, v4))
    }

    fn recognize_context(
        &self,
        state: &LanguageStateV5,
        cue: &[SurfaceSymbol],
    ) -> Result<LanguageStateV5, LanguageErrorV5> {
        if cue.is_empty() {
            return Err(LanguageErrorV5::EmptyContext);
        }
        if !self.valid_state(state) {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidState,
            )));
        }

        let mut activity = [0.0; 4];
        let scale = 1.0 / cue.len() as f64;
        for &symbol in cue {
            activity[symbol.index()] += scale;
        }

        let next_epoch = state
            .relational
            .context_history
            .last()
            .map(|record| record.epoch + 1)
            .unwrap_or(1);

        let mut next = state.clone();
        next.relational.context_history.push(ContextRecognitionRecordV5 {
            epoch: next_epoch,
            cue: cue.to_vec(),
            activity,
        });
        next.relational.current_context_epoch = Some(next_epoch);
        Ok(next)
    }

    fn assessed_profiles(
        &self,
        state: &LanguageStateV5,
    ) -> Result<Vec<InternalEquivalenceProfile>, LanguageErrorV5> {
        let mut profiles = Vec::new();
        for record in &state.relational.assessment_history {
            if !profiles.contains(&record.profile) {
                self.equivalence_for_profile(state, record.profile)?
                    .ok_or(LanguageErrorV5::NoAssessedProfiles)?;
                profiles.push(record.profile);
            }
        }
        if profiles.is_empty() {
            return Err(LanguageErrorV5::NoAssessedProfiles);
        }
        Ok(profiles)
    }

    fn current_context(
        &self,
        state: &LanguageStateV5,
    ) -> Result<&ContextRecognitionRecordV5, LanguageErrorV5> {
        let epoch = state
            .relational
            .current_context_epoch
            .ok_or(LanguageErrorV5::NoRecognizedContext)?;
        state
            .relational
            .context_history
            .iter()
            .find(|record| record.epoch == epoch)
            .ok_or(LanguageErrorV5::NoRecognizedContext)
    }

    fn profile_context_score(
        context: &ContextRecognitionRecordV5,
        profile: InternalEquivalenceProfile,
    ) -> f64 {
        profile
            .projection
            .iter()
            .map(|symbol| context.activity[symbol.index()])
            .sum()
    }

    fn infer_profile_from_context(
        &self,
        state: &LanguageStateV5,
    ) -> Result<LanguageStateV5, LanguageErrorV5> {
        if !self.valid_state(state) {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidState,
            )));
        }
        let context = self.current_context(state)?;
        let profiles = self.assessed_profiles(state)?;
        let candidate_scores: Vec<ProfileContextScoreV5> = profiles
            .iter()
            .map(|&profile| ProfileContextScoreV5 {
                profile,
                score: Self::profile_context_score(context, profile),
            })
            .collect();

        let top_score = candidate_scores
            .iter()
            .map(|entry| entry.score)
            .fold(f64::NEG_INFINITY, f64::max);
        if top_score < self.minimum_context_score {
            return Err(LanguageErrorV5::UnsupportedContext);
        }

        let top_indices: Vec<usize> = candidate_scores
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| (entry.score == top_score).then_some(index))
            .collect();
        if top_indices.len() != 1 {
            return Err(LanguageErrorV5::AmbiguousContext);
        }

        let winner = top_indices[0];
        let runner_up = candidate_scores
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != winner)
            .map(|(_, entry)| entry.score)
            .fold(0.0, f64::max);
        if top_score - runner_up <= self.minimum_context_margin {
            return Err(LanguageErrorV5::AmbiguousContext);
        }

        let selected_profile = candidate_scores[winner].profile;
        let next_epoch = state
            .relational
            .selection_history
            .last()
            .map(|record| record.epoch + 1)
            .unwrap_or(1);

        let mut next = state.clone();
        next.relational.selected_profile = Some(selected_profile);
        next.relational.selection_history.push(ContextSelectionRecordV5 {
            epoch: next_epoch,
            context_epoch: context.epoch,
            candidate_scores,
            selected_profile,
        });
        Ok(next)
    }

    pub fn expose(
        &self,
        initial: &LanguageStateV5,
        pattern: &[SurfaceSymbol],
        repeats: usize,
    ) -> Result<LanguageStateV5, LanguageErrorV5> {
        if pattern.is_empty() || repeats == 0 {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::EmptyExposure,
            )));
        }
        if !self.valid_state(initial) {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidState,
            )));
        }

        let mut state = initial.clone();
        let mut predecessor = None;
        for _ in 0..repeats {
            for &symbol in pattern {
                state = self.step(&state, &LanguageInput::symbol(symbol))?;
                state = self.adapt(
                    &state,
                    &LanguageExperienceV5::Sequential {
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

impl AdaptiveContinuationModel for CohfieldLanguageModelV5 {
    type State = LanguageStateV5;
    type Fast = [f64; 4];
    type LocalCondition = [f64; 4];
    type RelationalConfiguration = LanguageRelationalConfigurationV5;
    type Input = LanguageInput;
    type Experience = LanguageExperienceV5;
    type ObservationProfile = LanguageObservationProfile;
    type Response = LanguageResponse;
    type Error = LanguageErrorV5;

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
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidHorizon,
            )));
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
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidParameter,
            )));
        }
        if !self.valid_state(state) {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidState,
            )));
        }

        match experience {
            LanguageExperienceV5::Sequential {
                predecessor,
                current,
            } => {
                let v4 = self.v4_model().adapt(
                    &self.to_v4_state(state),
                    &LanguageExperienceV4::Sequential {
                        predecessor: *predecessor,
                        current: *current,
                    },
                )?;
                Ok(self.apply_v4_state(state, v4))
            }
            LanguageExperienceV5::AssessConsequenceEquivalence(profile) => {
                let v4 = self.v4_model().adapt(
                    &self.to_v4_state(state),
                    &LanguageExperienceV4::AssessConsequenceEquivalence(*profile),
                )?;
                Ok(self.apply_v4_state(state, v4))
            }
            LanguageExperienceV5::RecognizeContext(cue) => self.recognize_context(state, cue),
            LanguageExperienceV5::InferConsequenceProfileFromContext => {
                self.infer_profile_from_context(state)
            }
        }
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        if !self.valid_parameters() {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidParameter,
            )));
        }
        if !self.valid_state(state) {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::InvalidState,
            )));
        }
        if profile.probes.is_empty() {
            return Err(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
                LanguageError::Base(LanguageError::EmptyProbeFamily),
            )));
        }

        let mut vectors =
            Vec::with_capacity(profile.probes.len() * (2 + profile.continuation_steps));
        for probe in &profile.probes {
            let mut local = LanguageStateV5::equalized_from(state);
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
