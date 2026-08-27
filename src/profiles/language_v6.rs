use crate::{AdaptiveContinuationModel, StateRoles};

use super::language::{
    LanguageError, LanguageInput, LanguageObservationProfile, LanguageResponse, SurfaceSymbol,
};
use super::language_v2::InternalEquivalenceProfile;
use super::language_v3::ConsequenceEquivalenceAssessment;
use super::language_v4::LanguageErrorV4;
use super::language_v5::{
    CohfieldLanguageModelV5, ContextRecognitionRecordV5, ContextSelectionRecordV5,
    LanguageErrorV5, LanguageExperienceV5, LanguageRelationalConfigurationV5, LanguageStateV5,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ApplicabilityTeachingRecordV6 {
    pub epoch: u64,
    pub context_epoch: u64,
    pub profile: InternalEquivalenceProfile,
    pub activity: [f64; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfileApplicabilityPrototypeV6 {
    pub profile: InternalEquivalenceProfile,
    pub activity: [f64; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfileApplicabilityDistanceV6 {
    pub profile: InternalEquivalenceProfile,
    pub distance: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LearnedApplicabilitySelectionRecordV6 {
    pub epoch: u64,
    pub context_epoch: u64,
    pub candidate_distances: Vec<ProfileApplicabilityDistanceV6>,
    pub selected_profile: InternalEquivalenceProfile,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageRelationalConfigurationV6 {
    pub sequential: [[f64; 4]; 4],
    pub selected_profile: Option<InternalEquivalenceProfile>,
    pub assessment_history: Vec<ConsequenceEquivalenceAssessment>,
    pub current_context_epoch: Option<u64>,
    pub context_history: Vec<ContextRecognitionRecordV5>,
    pub projection_selection_history: Vec<ContextSelectionRecordV5>,
    pub applicability_history: Vec<ApplicabilityTeachingRecordV6>,
    pub learned_selection_history: Vec<LearnedApplicabilitySelectionRecordV6>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageStateV6 {
    pub x: [f64; 4],
    pub theta: [f64; 4],
    pub relational: LanguageRelationalConfigurationV6,
}

impl LanguageStateV6 {
    pub fn initial() -> Self {
        Self {
            x: [0.0; 4],
            theta: [1.0; 4],
            relational: LanguageRelationalConfigurationV6 {
                sequential: [[0.0; 4]; 4],
                selected_profile: None,
                assessment_history: Vec::new(),
                current_context_epoch: None,
                context_history: Vec::new(),
                projection_selection_history: Vec::new(),
                applicability_history: Vec::new(),
                learned_selection_history: Vec::new(),
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
pub enum LanguageErrorV6 {
    BaseV5(LanguageErrorV5),
    ProfileNotAssessed,
    NoApplicabilityExperience,
    UnsupportedApplicability,
    AmbiguousApplicability,
}

impl From<LanguageErrorV5> for LanguageErrorV6 {
    fn from(value: LanguageErrorV5) -> Self {
        Self::BaseV5(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageExperienceV6 {
    Sequential {
        predecessor: Option<SurfaceSymbol>,
        current: SurfaceSymbol,
    },
    AssessConsequenceEquivalence(InternalEquivalenceProfile),
    RecognizeContext(Vec<SurfaceSymbol>),
    RecordContextApplicability(InternalEquivalenceProfile),
    InferConsequenceProfileFromLearnedApplicability,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohfieldLanguageModelV6 {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub equivalence_coupling: f64,
    pub maximum_applicability_distance: f64,
    pub minimum_applicability_margin: f64,
}

impl Default for CohfieldLanguageModelV6 {
    fn default() -> Self {
        let v5 = CohfieldLanguageModelV5::default();
        Self {
            beta: v5.beta,
            input_gain: v5.input_gain,
            relational_gain: v5.relational_gain,
            psi_decay: v5.psi_decay,
            psi_gain: v5.psi_gain,
            equivalence_coupling: v5.equivalence_coupling,
            maximum_applicability_distance: 0.50,
            minimum_applicability_margin: 0.25,
        }
    }
}

impl CohfieldLanguageModelV6 {
    fn invalid_parameter() -> LanguageErrorV6 {
        LanguageErrorV6::BaseV5(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
            LanguageError::InvalidParameter,
        )))
    }

    fn invalid_state() -> LanguageErrorV6 {
        LanguageErrorV6::BaseV5(LanguageErrorV5::BaseV4(LanguageErrorV4::Base(
            LanguageError::InvalidState,
        )))
    }

    fn v5_model(&self) -> CohfieldLanguageModelV5 {
        CohfieldLanguageModelV5 {
            beta: self.beta,
            input_gain: self.input_gain,
            relational_gain: self.relational_gain,
            psi_decay: self.psi_decay,
            psi_gain: self.psi_gain,
            equivalence_coupling: self.equivalence_coupling,
            ..CohfieldLanguageModelV5::default()
        }
    }

    fn valid_parameters(&self) -> bool {
        self.beta.is_finite()
            && self.input_gain.is_finite()
            && self.relational_gain.is_finite()
            && self.psi_decay.is_finite()
            && self.psi_gain.is_finite()
            && self.equivalence_coupling.is_finite()
            && self.maximum_applicability_distance.is_finite()
            && self.minimum_applicability_margin.is_finite()
            && (0.0..=1.0).contains(&self.psi_decay)
            && self.psi_gain >= 0.0
            && self.equivalence_coupling >= 0.0
            && self.maximum_applicability_distance >= 0.0
            && self.minimum_applicability_margin >= 0.0
    }

    fn context_for_epoch(
        state: &LanguageStateV6,
        epoch: u64,
    ) -> Option<&ContextRecognitionRecordV5> {
        state
            .relational
            .context_history
            .iter()
            .find(|record| record.epoch == epoch)
    }

    fn context_reference_valid(state: &LanguageStateV6) -> bool {
        match state.relational.current_context_epoch {
            Some(epoch) => Self::context_for_epoch(state, epoch).is_some(),
            None => true,
        }
    }

    fn applicability_references_valid(state: &LanguageStateV6) -> bool {
        state.relational.applicability_history.iter().all(|record| {
            Self::context_for_epoch(state, record.context_epoch)
                .map(|context| context.activity == record.activity)
                .unwrap_or(false)
        })
    }

    fn learned_selection_references_valid(state: &LanguageStateV6) -> bool {
        state.relational.learned_selection_history.iter().all(|record| {
            Self::context_for_epoch(state, record.context_epoch).is_some()
                && record
                    .candidate_distances
                    .iter()
                    .any(|candidate| candidate.profile == record.selected_profile)
        })
    }

    fn valid_state(&self, state: &LanguageStateV6) -> bool {
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
                .projection_selection_history
                .iter()
                .flat_map(|record| record.candidate_scores.iter())
                .all(|entry| entry.score.is_finite())
            && state
                .relational
                .applicability_history
                .iter()
                .flat_map(|record| record.activity.iter())
                .all(|value| value.is_finite())
            && state
                .relational
                .learned_selection_history
                .iter()
                .flat_map(|record| record.candidate_distances.iter())
                .all(|entry| entry.distance.is_finite())
            && state.theta == [1.0; 4]
            && Self::context_reference_valid(state)
            && Self::applicability_references_valid(state)
            && Self::learned_selection_references_valid(state)
    }

    fn to_v5_state(&self, state: &LanguageStateV6) -> LanguageStateV5 {
        LanguageStateV5 {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV5 {
                sequential: state.relational.sequential,
                selected_profile: state.relational.selected_profile,
                assessment_history: state.relational.assessment_history.clone(),
                current_context_epoch: state.relational.current_context_epoch,
                context_history: state.relational.context_history.clone(),
                selection_history: state.relational.projection_selection_history.clone(),
            },
        }
    }

    fn apply_v5_state(&self, state: &LanguageStateV6, v5: LanguageStateV5) -> LanguageStateV6 {
        let mut next = state.clone();
        next.x = v5.x;
        next.theta = v5.theta;
        next.relational.sequential = v5.relational.sequential;
        next.relational.selected_profile = v5.relational.selected_profile;
        next.relational.assessment_history = v5.relational.assessment_history;
        next.relational.current_context_epoch = v5.relational.current_context_epoch;
        next.relational.context_history = v5.relational.context_history;
        next.relational.projection_selection_history = v5.relational.selection_history;
        next
    }

    pub fn migrate_from_v5(
        &self,
        state: &LanguageStateV5,
    ) -> Result<LanguageStateV6, LanguageErrorV6> {
        if !self.valid_parameters() {
            return Err(Self::invalid_parameter());
        }
        let next = LanguageStateV6 {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV6 {
                sequential: state.relational.sequential,
                selected_profile: state.relational.selected_profile,
                assessment_history: state.relational.assessment_history.clone(),
                current_context_epoch: state.relational.current_context_epoch,
                context_history: state.relational.context_history.clone(),
                projection_selection_history: state.relational.selection_history.clone(),
                applicability_history: Vec::new(),
                learned_selection_history: Vec::new(),
            },
        };
        if !self.valid_state(&next) {
            return Err(Self::invalid_state());
        }
        Ok(next)
    }

    pub fn equivalence_for_profile(
        &self,
        state: &LanguageStateV6,
        profile: InternalEquivalenceProfile,
    ) -> Result<Option<[[bool; 4]; 4]>, LanguageErrorV6> {
        self.v5_model()
            .equivalence_for_profile(&self.to_v5_state(state), profile)
            .map_err(Into::into)
    }

    pub fn selected_equivalence(
        &self,
        state: &LanguageStateV6,
    ) -> Result<[[bool; 4]; 4], LanguageErrorV6> {
        self.v5_model()
            .selected_equivalence(&self.to_v5_state(state))
            .map_err(Into::into)
    }

    fn current_context<'a>(
        &self,
        state: &'a LanguageStateV6,
    ) -> Result<&'a ContextRecognitionRecordV5, LanguageErrorV6> {
        let epoch = state
            .relational
            .current_context_epoch
            .ok_or(LanguageErrorV6::BaseV5(LanguageErrorV5::NoRecognizedContext))?;
        Self::context_for_epoch(state, epoch)
            .ok_or(LanguageErrorV6::BaseV5(LanguageErrorV5::NoRecognizedContext))
    }

    fn ensure_profile_assessed(
        &self,
        state: &LanguageStateV6,
        profile: InternalEquivalenceProfile,
    ) -> Result<(), LanguageErrorV6> {
        if self.equivalence_for_profile(state, profile)?.is_none() {
            return Err(LanguageErrorV6::ProfileNotAssessed);
        }
        Ok(())
    }

    fn record_context_applicability(
        &self,
        state: &LanguageStateV6,
        profile: InternalEquivalenceProfile,
    ) -> Result<LanguageStateV6, LanguageErrorV6> {
        if !self.valid_state(state) {
            return Err(Self::invalid_state());
        }
        self.ensure_profile_assessed(state, profile)?;
        let context = self.current_context(state)?.clone();
        let next_epoch = state
            .relational
            .applicability_history
            .last()
            .map(|record| record.epoch + 1)
            .unwrap_or(1);

        let mut next = state.clone();
        next.relational
            .applicability_history
            .push(ApplicabilityTeachingRecordV6 {
                epoch: next_epoch,
                context_epoch: context.epoch,
                profile,
                activity: context.activity,
            });
        Ok(next)
    }

    pub fn applicability_prototypes(
        &self,
        state: &LanguageStateV6,
    ) -> Result<Vec<ProfileApplicabilityPrototypeV6>, LanguageErrorV6> {
        if !self.valid_state(state) {
            return Err(Self::invalid_state());
        }
        if state.relational.applicability_history.is_empty() {
            return Err(LanguageErrorV6::NoApplicabilityExperience);
        }

        let mut profiles = Vec::new();
        for record in &state.relational.applicability_history {
            if !profiles.contains(&record.profile) {
                self.ensure_profile_assessed(state, record.profile)?;
                profiles.push(record.profile);
            }
        }

        let mut prototypes = Vec::with_capacity(profiles.len());
        for profile in profiles {
            let mut sum = [0.0; 4];
            let mut count = 0usize;
            for record in state
                .relational
                .applicability_history
                .iter()
                .filter(|record| record.profile == profile)
            {
                for (index, value) in sum.iter_mut().enumerate() {
                    *value += record.activity[index];
                }
                count += 1;
            }
            if count == 0 {
                return Err(LanguageErrorV6::NoApplicabilityExperience);
            }
            for value in &mut sum {
                *value /= count as f64;
            }
            prototypes.push(ProfileApplicabilityPrototypeV6 {
                profile,
                activity: sum,
            });
        }
        Ok(prototypes)
    }

    fn euclidean(left: [f64; 4], right: [f64; 4]) -> f64 {
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }

    fn infer_profile_from_learned_applicability(
        &self,
        state: &LanguageStateV6,
    ) -> Result<LanguageStateV6, LanguageErrorV6> {
        if !self.valid_parameters() {
            return Err(Self::invalid_parameter());
        }
        if !self.valid_state(state) {
            return Err(Self::invalid_state());
        }
        let context = self.current_context(state)?.clone();
        let prototypes = self.applicability_prototypes(state)?;
        let candidate_distances: Vec<ProfileApplicabilityDistanceV6> = prototypes
            .iter()
            .map(|prototype| ProfileApplicabilityDistanceV6 {
                profile: prototype.profile,
                distance: Self::euclidean(context.activity, prototype.activity),
            })
            .collect();

        let minimum_distance = candidate_distances
            .iter()
            .map(|entry| entry.distance)
            .fold(f64::INFINITY, f64::min);
        if minimum_distance > self.maximum_applicability_distance {
            return Err(LanguageErrorV6::UnsupportedApplicability);
        }

        let winners: Vec<usize> = candidate_distances
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| (entry.distance == minimum_distance).then_some(index))
            .collect();
        if winners.len() != 1 {
            return Err(LanguageErrorV6::AmbiguousApplicability);
        }

        let winner = winners[0];
        let runner_up = candidate_distances
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != winner)
            .map(|(_, entry)| entry.distance)
            .fold(f64::INFINITY, f64::min);
        if runner_up.is_finite()
            && runner_up - minimum_distance <= self.minimum_applicability_margin
        {
            return Err(LanguageErrorV6::AmbiguousApplicability);
        }

        let selected_profile = candidate_distances[winner].profile;
        let next_epoch = state
            .relational
            .learned_selection_history
            .last()
            .map(|record| record.epoch + 1)
            .unwrap_or(1);

        let mut next = state.clone();
        next.relational.selected_profile = Some(selected_profile);
        next.relational
            .learned_selection_history
            .push(LearnedApplicabilitySelectionRecordV6 {
                epoch: next_epoch,
                context_epoch: context.epoch,
                candidate_distances,
                selected_profile,
            });
        Ok(next)
    }

    pub fn expose(
        &self,
        initial: &LanguageStateV6,
        pattern: &[SurfaceSymbol],
        repeats: usize,
    ) -> Result<LanguageStateV6, LanguageErrorV6> {
        if !self.valid_parameters() {
            return Err(Self::invalid_parameter());
        }
        if !self.valid_state(initial) {
            return Err(Self::invalid_state());
        }
        let v5 = self
            .v5_model()
            .expose(&self.to_v5_state(initial), pattern, repeats)?;
        Ok(self.apply_v5_state(initial, v5))
    }
}

impl AdaptiveContinuationModel for CohfieldLanguageModelV6 {
    type State = LanguageStateV6;
    type Fast = [f64; 4];
    type LocalCondition = [f64; 4];
    type RelationalConfiguration = LanguageRelationalConfigurationV6;
    type Input = LanguageInput;
    type Experience = LanguageExperienceV6;
    type ObservationProfile = LanguageObservationProfile;
    type Response = LanguageResponse;
    type Error = LanguageErrorV6;

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
        if !self.valid_parameters() {
            return Err(Self::invalid_parameter());
        }
        if !self.valid_state(state) {
            return Err(Self::invalid_state());
        }
        let v5 = self
            .v5_model()
            .evolve(&self.to_v5_state(state), input, horizon)?;
        Ok(self.apply_v5_state(state, v5))
    }

    fn adapt(
        &self,
        state: &Self::State,
        experience: &Self::Experience,
    ) -> Result<Self::State, Self::Error> {
        if !self.valid_parameters() {
            return Err(Self::invalid_parameter());
        }
        if !self.valid_state(state) {
            return Err(Self::invalid_state());
        }

        match experience {
            LanguageExperienceV6::Sequential {
                predecessor,
                current,
            } => {
                let v5 = self.v5_model().adapt(
                    &self.to_v5_state(state),
                    &LanguageExperienceV5::Sequential {
                        predecessor: *predecessor,
                        current: *current,
                    },
                )?;
                Ok(self.apply_v5_state(state, v5))
            }
            LanguageExperienceV6::AssessConsequenceEquivalence(profile) => {
                let v5 = self.v5_model().adapt(
                    &self.to_v5_state(state),
                    &LanguageExperienceV5::AssessConsequenceEquivalence(*profile),
                )?;
                Ok(self.apply_v5_state(state, v5))
            }
            LanguageExperienceV6::RecognizeContext(cue) => {
                let v5 = self.v5_model().adapt(
                    &self.to_v5_state(state),
                    &LanguageExperienceV5::RecognizeContext(cue.clone()),
                )?;
                Ok(self.apply_v5_state(state, v5))
            }
            LanguageExperienceV6::RecordContextApplicability(profile) => {
                self.record_context_applicability(state, *profile)
            }
            LanguageExperienceV6::InferConsequenceProfileFromLearnedApplicability => {
                self.infer_profile_from_learned_applicability(state)
            }
        }
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        if !self.valid_parameters() {
            return Err(Self::invalid_parameter());
        }
        if !self.valid_state(state) {
            return Err(Self::invalid_state());
        }
        self.v5_model()
            .observe(&self.to_v5_state(state), profile)
            .map_err(Into::into)
    }
}
