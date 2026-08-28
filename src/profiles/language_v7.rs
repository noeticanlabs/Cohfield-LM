use crate::{AdaptiveContinuationModel, StateRoles};

use super::language::{LanguageInput, LanguageObservationProfile, LanguageResponse, SurfaceSymbol};
use super::language_v2::InternalEquivalenceProfile;
use super::language_v3::ConsequenceEquivalenceAssessment;
use super::language_v5::{ContextRecognitionRecordV5, ContextSelectionRecordV5};
use super::language_v6::{
    ApplicabilityTeachingRecordV6, CohfieldLanguageModelV6, LanguageErrorV6, LanguageExperienceV6,
    LanguageRelationalConfigurationV6, LanguageStateV6, LearnedApplicabilitySelectionRecordV6,
};

#[derive(Clone, Debug, PartialEq)]
pub struct OutcomePredictionErrorV7 {
    pub profile: InternalEquivalenceProfile,
    pub error: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsequenceGroundedApplicabilityRecordV7 {
    pub epoch: u64,
    pub context_epoch: u64,
    pub activity: [f64; 4],
    pub observed_consequence: [f64; 5],
    pub candidate_errors: Vec<OutcomePredictionErrorV7>,
    pub inferred_profile: InternalEquivalenceProfile,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutcomeApplicabilityPrototypeV7 {
    pub profile: InternalEquivalenceProfile,
    pub activity: [f64; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutcomeApplicabilityDistanceV7 {
    pub profile: InternalEquivalenceProfile,
    pub distance: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutcomeApplicabilitySelectionRecordV7 {
    pub epoch: u64,
    pub context_epoch: u64,
    pub candidate_distances: Vec<OutcomeApplicabilityDistanceV7>,
    pub selected_profile: InternalEquivalenceProfile,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageRelationalConfigurationV7 {
    pub sequential: [[f64; 4]; 4],
    pub selected_profile: Option<InternalEquivalenceProfile>,
    pub assessment_history: Vec<ConsequenceEquivalenceAssessment>,
    pub current_context_epoch: Option<u64>,
    pub context_history: Vec<ContextRecognitionRecordV5>,
    pub projection_selection_history: Vec<ContextSelectionRecordV5>,
    pub applicability_history: Vec<ApplicabilityTeachingRecordV6>,
    pub learned_selection_history: Vec<LearnedApplicabilitySelectionRecordV6>,
    pub outcome_applicability_history: Vec<ConsequenceGroundedApplicabilityRecordV7>,
    pub outcome_selection_history: Vec<OutcomeApplicabilitySelectionRecordV7>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageStateV7 {
    pub x: [f64; 4],
    pub theta: [f64; 4],
    pub relational: LanguageRelationalConfigurationV7,
}

impl LanguageStateV7 {
    pub fn initial() -> Self {
        Self {
            x: [0.0; 4],
            theta: [1.0; 4],
            relational: LanguageRelationalConfigurationV7 {
                sequential: [[0.0; 4]; 4],
                selected_profile: None,
                assessment_history: Vec::new(),
                current_context_epoch: None,
                context_history: Vec::new(),
                projection_selection_history: Vec::new(),
                applicability_history: Vec::new(),
                learned_selection_history: Vec::new(),
                outcome_applicability_history: Vec::new(),
                outcome_selection_history: Vec::new(),
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
pub enum LanguageErrorV7 {
    BaseV6(LanguageErrorV6),
    ProfileNotAssessed,
    InvalidObservedConsequence,
    UnsupportedOutcome,
    AmbiguousOutcome,
    NoOutcomeApplicabilityExperience,
    UnsupportedOutcomeApplicability,
    AmbiguousOutcomeApplicability,
}

impl From<LanguageErrorV6> for LanguageErrorV7 {
    fn from(value: LanguageErrorV6) -> Self {
        Self::BaseV6(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageExperienceV7 {
    Sequential {
        predecessor: Option<SurfaceSymbol>,
        current: SurfaceSymbol,
    },
    AssessConsequenceEquivalence(InternalEquivalenceProfile),
    RecognizeContext(Vec<SurfaceSymbol>),
    RecordContextApplicability(InternalEquivalenceProfile),
    InferConsequenceProfileFromLearnedApplicability,
    RecordObservedConsequence([f64; 5]),
    InferConsequenceProfileFromOutcomeApplicability,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohfieldLanguageModelV7 {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub equivalence_coupling: f64,
    pub maximum_applicability_distance: f64,
    pub minimum_applicability_margin: f64,
    pub maximum_outcome_prediction_error: f64,
    pub minimum_outcome_error_margin: f64,
}

impl Default for CohfieldLanguageModelV7 {
    fn default() -> Self {
        let v6 = CohfieldLanguageModelV6::default();
        Self {
            beta: v6.beta,
            input_gain: v6.input_gain,
            relational_gain: v6.relational_gain,
            psi_decay: v6.psi_decay,
            psi_gain: v6.psi_gain,
            equivalence_coupling: v6.equivalence_coupling,
            maximum_applicability_distance: v6.maximum_applicability_distance,
            minimum_applicability_margin: v6.minimum_applicability_margin,
            maximum_outcome_prediction_error: 0.020,
            minimum_outcome_error_margin: 0.010,
        }
    }
}

impl CohfieldLanguageModelV7 {
    fn v6_model(&self) -> CohfieldLanguageModelV6 {
        CohfieldLanguageModelV6 {
            beta: self.beta,
            input_gain: self.input_gain,
            relational_gain: self.relational_gain,
            psi_decay: self.psi_decay,
            psi_gain: self.psi_gain,
            equivalence_coupling: self.equivalence_coupling,
            maximum_applicability_distance: self.maximum_applicability_distance,
            minimum_applicability_margin: self.minimum_applicability_margin,
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
            && self.maximum_outcome_prediction_error.is_finite()
            && self.minimum_outcome_error_margin.is_finite()
            && (0.0..=1.0).contains(&self.psi_decay)
            && self.psi_gain >= 0.0
            && self.equivalence_coupling >= 0.0
            && self.maximum_applicability_distance >= 0.0
            && self.minimum_applicability_margin >= 0.0
            && self.maximum_outcome_prediction_error >= 0.0
            && self.minimum_outcome_error_margin >= 0.0
    }

    fn to_v6_state(&self, state: &LanguageStateV7) -> LanguageStateV6 {
        LanguageStateV6 {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV6 {
                sequential: state.relational.sequential,
                selected_profile: state.relational.selected_profile,
                assessment_history: state.relational.assessment_history.clone(),
                current_context_epoch: state.relational.current_context_epoch,
                context_history: state.relational.context_history.clone(),
                projection_selection_history: state.relational.projection_selection_history.clone(),
                applicability_history: state.relational.applicability_history.clone(),
                learned_selection_history: state.relational.learned_selection_history.clone(),
            },
        }
    }

    fn apply_v6_state(&self, state: &LanguageStateV7, v6: LanguageStateV6) -> LanguageStateV7 {
        let mut next = state.clone();
        next.x = v6.x;
        next.theta = v6.theta;
        next.relational.sequential = v6.relational.sequential;
        next.relational.selected_profile = v6.relational.selected_profile;
        next.relational.assessment_history = v6.relational.assessment_history;
        next.relational.current_context_epoch = v6.relational.current_context_epoch;
        next.relational.context_history = v6.relational.context_history;
        next.relational.projection_selection_history = v6.relational.projection_selection_history;
        next.relational.applicability_history = v6.relational.applicability_history;
        next.relational.learned_selection_history = v6.relational.learned_selection_history;
        next
    }

    fn parent_state_valid(&self, state: &LanguageStateV7) -> bool {
        self.v6_model()
            .evolve(&self.to_v6_state(state), &LanguageInput::zero(), 0.0)
            .is_ok()
    }

    fn valid_state(&self, state: &LanguageStateV7) -> bool {
        if !self.parent_state_valid(state) {
            return false;
        }

        let outcome_records_valid =
            state
                .relational
                .outcome_applicability_history
                .iter()
                .all(|record| {
                    record
                        .observed_consequence
                        .iter()
                        .all(|value| value.is_finite())
                        && record
                            .candidate_errors
                            .iter()
                            .all(|entry| entry.error.is_finite())
                        && state.relational.context_history.iter().any(|context| {
                            context.epoch == record.context_epoch
                                && context.activity == record.activity
                        })
                });
        let selection_records_valid =
            state
                .relational
                .outcome_selection_history
                .iter()
                .all(|record| {
                    record
                        .candidate_distances
                        .iter()
                        .all(|entry| entry.distance.is_finite())
                        && state
                            .relational
                            .context_history
                            .iter()
                            .any(|context| context.epoch == record.context_epoch)
                });

        outcome_records_valid && selection_records_valid
    }

    pub fn migrate_from_v6(
        &self,
        state: &LanguageStateV6,
    ) -> Result<LanguageStateV7, LanguageErrorV7> {
        let next = LanguageStateV7 {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV7 {
                sequential: state.relational.sequential,
                selected_profile: state.relational.selected_profile,
                assessment_history: state.relational.assessment_history.clone(),
                current_context_epoch: state.relational.current_context_epoch,
                context_history: state.relational.context_history.clone(),
                projection_selection_history: state.relational.projection_selection_history.clone(),
                applicability_history: state.relational.applicability_history.clone(),
                learned_selection_history: state.relational.learned_selection_history.clone(),
                outcome_applicability_history: Vec::new(),
                outcome_selection_history: Vec::new(),
            },
        };
        if !self.valid_parameters() || !self.valid_state(&next) {
            return Err(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::BaseV4(
                    super::language_v4::LanguageErrorV4::Base(
                        super::language::LanguageError::InvalidState,
                    ),
                ),
            )));
        }
        Ok(next)
    }

    pub fn equivalence_for_profile(
        &self,
        state: &LanguageStateV7,
        profile: InternalEquivalenceProfile,
    ) -> Result<Option<[[bool; 4]; 4]>, LanguageErrorV7> {
        self.v6_model()
            .equivalence_for_profile(&self.to_v6_state(state), profile)
            .map_err(Into::into)
    }

    pub fn selected_equivalence(
        &self,
        state: &LanguageStateV7,
    ) -> Result<[[bool; 4]; 4], LanguageErrorV7> {
        self.v6_model()
            .selected_equivalence(&self.to_v6_state(state))
            .map_err(Into::into)
    }

    fn current_context<'a>(
        &self,
        state: &'a LanguageStateV7,
    ) -> Result<&'a ContextRecognitionRecordV5, LanguageErrorV7> {
        let epoch = state
            .relational
            .current_context_epoch
            .ok_or(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::NoRecognizedContext,
            )))?;
        state
            .relational
            .context_history
            .iter()
            .find(|record| record.epoch == epoch)
            .ok_or(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::NoRecognizedContext,
            )))
    }

    fn ensure_profile_assessed(
        &self,
        state: &LanguageStateV7,
        profile: InternalEquivalenceProfile,
    ) -> Result<(), LanguageErrorV7> {
        if self.equivalence_for_profile(state, profile)?.is_none() {
            return Err(LanguageErrorV7::ProfileNotAssessed);
        }
        Ok(())
    }

    fn assessed_profiles(
        &self,
        state: &LanguageStateV7,
    ) -> Result<Vec<InternalEquivalenceProfile>, LanguageErrorV7> {
        let mut profiles = Vec::new();
        for record in &state.relational.assessment_history {
            if !profiles.contains(&record.profile) {
                self.ensure_profile_assessed(state, record.profile)?;
                profiles.push(record.profile);
            }
        }
        if profiles.is_empty() {
            return Err(LanguageErrorV7::ProfileNotAssessed);
        }
        Ok(profiles)
    }

    pub fn predicted_consequence_signature(
        &self,
        state: &LanguageStateV7,
        profile: InternalEquivalenceProfile,
    ) -> Result<[f64; 5], LanguageErrorV7> {
        self.ensure_profile_assessed(state, profile)?;
        let mut witness = state.clone();
        witness.relational.selected_profile = Some(profile);
        witness = LanguageStateV7::equalized_from(&witness);
        witness = self.evolve(&witness, &LanguageInput::symbol(SurfaceSymbol::D), 1.0)?;

        let mut out = [0.0; 5];
        out[0] = witness.x[SurfaceSymbol::A.index()];
        for value in out.iter_mut().skip(1) {
            witness = self.evolve(&witness, &LanguageInput::zero(), 1.0)?;
            *value = witness.x[SurfaceSymbol::A.index()];
        }
        Ok(out)
    }

    fn euclidean5(left: [f64; 5], right: [f64; 5]) -> f64 {
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }

    fn record_observed_consequence(
        &self,
        state: &LanguageStateV7,
        observed: [f64; 5],
    ) -> Result<LanguageStateV7, LanguageErrorV7> {
        if !self.valid_parameters() || !self.valid_state(state) {
            return Err(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::BaseV4(
                    super::language_v4::LanguageErrorV4::Base(
                        super::language::LanguageError::InvalidState,
                    ),
                ),
            )));
        }
        if observed.iter().any(|value| !value.is_finite()) {
            return Err(LanguageErrorV7::InvalidObservedConsequence);
        }

        let context = self.current_context(state)?.clone();
        let profiles = self.assessed_profiles(state)?;
        let candidate_errors: Vec<OutcomePredictionErrorV7> = profiles
            .iter()
            .map(|&profile| {
                self.predicted_consequence_signature(state, profile)
                    .map(|predicted| OutcomePredictionErrorV7 {
                        profile,
                        error: Self::euclidean5(predicted, observed),
                    })
            })
            .collect::<Result<_, _>>()?;

        let minimum_error = candidate_errors
            .iter()
            .map(|entry| entry.error)
            .fold(f64::INFINITY, f64::min);
        let winners: Vec<usize> = candidate_errors
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| (entry.error == minimum_error).then_some(index))
            .collect();
        if winners.len() != 1 {
            return Err(LanguageErrorV7::AmbiguousOutcome);
        }
        if minimum_error > self.maximum_outcome_prediction_error {
            return Err(LanguageErrorV7::UnsupportedOutcome);
        }

        let winner = winners[0];
        let runner_up = candidate_errors
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != winner)
            .map(|(_, entry)| entry.error)
            .fold(f64::INFINITY, f64::min);
        if runner_up.is_finite() && runner_up - minimum_error <= self.minimum_outcome_error_margin {
            return Err(LanguageErrorV7::AmbiguousOutcome);
        }

        let inferred_profile = candidate_errors[winner].profile;
        let next_epoch = state
            .relational
            .outcome_applicability_history
            .last()
            .map(|record| record.epoch + 1)
            .unwrap_or(1);

        let mut next = state.clone();
        next.relational.outcome_applicability_history.push(
            ConsequenceGroundedApplicabilityRecordV7 {
                epoch: next_epoch,
                context_epoch: context.epoch,
                activity: context.activity,
                observed_consequence: observed,
                candidate_errors,
                inferred_profile,
            },
        );
        Ok(next)
    }

    pub fn outcome_applicability_prototypes(
        &self,
        state: &LanguageStateV7,
    ) -> Result<Vec<OutcomeApplicabilityPrototypeV7>, LanguageErrorV7> {
        if !self.valid_state(state) {
            return Err(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::BaseV4(
                    super::language_v4::LanguageErrorV4::Base(
                        super::language::LanguageError::InvalidState,
                    ),
                ),
            )));
        }
        if state.relational.outcome_applicability_history.is_empty() {
            return Err(LanguageErrorV7::NoOutcomeApplicabilityExperience);
        }

        let mut profiles = Vec::new();
        for record in &state.relational.outcome_applicability_history {
            if !profiles.contains(&record.inferred_profile) {
                self.ensure_profile_assessed(state, record.inferred_profile)?;
                profiles.push(record.inferred_profile);
            }
        }

        let mut prototypes = Vec::with_capacity(profiles.len());
        for profile in profiles {
            let mut sum = [0.0; 4];
            let mut count = 0usize;
            for record in state
                .relational
                .outcome_applicability_history
                .iter()
                .filter(|record| record.inferred_profile == profile)
            {
                for (index, value) in sum.iter_mut().enumerate() {
                    *value += record.activity[index];
                }
                count += 1;
            }
            if count == 0 {
                return Err(LanguageErrorV7::NoOutcomeApplicabilityExperience);
            }
            for value in &mut sum {
                *value /= count as f64;
            }
            prototypes.push(OutcomeApplicabilityPrototypeV7 {
                profile,
                activity: sum,
            });
        }
        Ok(prototypes)
    }

    fn euclidean4(left: [f64; 4], right: [f64; 4]) -> f64 {
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }

    fn infer_profile_from_outcome_applicability(
        &self,
        state: &LanguageStateV7,
    ) -> Result<LanguageStateV7, LanguageErrorV7> {
        if !self.valid_parameters() || !self.valid_state(state) {
            return Err(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::BaseV4(
                    super::language_v4::LanguageErrorV4::Base(
                        super::language::LanguageError::InvalidState,
                    ),
                ),
            )));
        }
        let context = self.current_context(state)?.clone();
        let prototypes = self.outcome_applicability_prototypes(state)?;
        let candidate_distances: Vec<OutcomeApplicabilityDistanceV7> = prototypes
            .iter()
            .map(|prototype| OutcomeApplicabilityDistanceV7 {
                profile: prototype.profile,
                distance: Self::euclidean4(context.activity, prototype.activity),
            })
            .collect();

        let minimum_distance = candidate_distances
            .iter()
            .map(|entry| entry.distance)
            .fold(f64::INFINITY, f64::min);
        if minimum_distance > self.maximum_applicability_distance {
            return Err(LanguageErrorV7::UnsupportedOutcomeApplicability);
        }

        let winners: Vec<usize> = candidate_distances
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| (entry.distance == minimum_distance).then_some(index))
            .collect();
        if winners.len() != 1 {
            return Err(LanguageErrorV7::AmbiguousOutcomeApplicability);
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
            return Err(LanguageErrorV7::AmbiguousOutcomeApplicability);
        }

        let selected_profile = candidate_distances[winner].profile;
        let next_epoch = state
            .relational
            .outcome_selection_history
            .last()
            .map(|record| record.epoch + 1)
            .unwrap_or(1);

        let mut next = state.clone();
        next.relational.selected_profile = Some(selected_profile);
        next.relational
            .outcome_selection_history
            .push(OutcomeApplicabilitySelectionRecordV7 {
                epoch: next_epoch,
                context_epoch: context.epoch,
                candidate_distances,
                selected_profile,
            });
        Ok(next)
    }

    pub fn expose(
        &self,
        initial: &LanguageStateV7,
        pattern: &[SurfaceSymbol],
        repeats: usize,
    ) -> Result<LanguageStateV7, LanguageErrorV7> {
        if !self.valid_parameters() || !self.valid_state(initial) {
            return Err(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::BaseV4(
                    super::language_v4::LanguageErrorV4::Base(
                        super::language::LanguageError::InvalidState,
                    ),
                ),
            )));
        }
        let v6 = self
            .v6_model()
            .expose(&self.to_v6_state(initial), pattern, repeats)?;
        Ok(self.apply_v6_state(initial, v6))
    }
}

impl AdaptiveContinuationModel for CohfieldLanguageModelV7 {
    type State = LanguageStateV7;
    type Fast = [f64; 4];
    type LocalCondition = [f64; 4];
    type RelationalConfiguration = LanguageRelationalConfigurationV7;
    type Input = LanguageInput;
    type Experience = LanguageExperienceV7;
    type ObservationProfile = LanguageObservationProfile;
    type Response = LanguageResponse;
    type Error = LanguageErrorV7;

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
        if !self.valid_parameters() || !self.valid_state(state) {
            return Err(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::BaseV4(
                    super::language_v4::LanguageErrorV4::Base(
                        super::language::LanguageError::InvalidState,
                    ),
                ),
            )));
        }
        let v6 = self
            .v6_model()
            .evolve(&self.to_v6_state(state), input, horizon)?;
        Ok(self.apply_v6_state(state, v6))
    }

    fn adapt(
        &self,
        state: &Self::State,
        experience: &Self::Experience,
    ) -> Result<Self::State, Self::Error> {
        if !self.valid_parameters() || !self.valid_state(state) {
            return Err(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::BaseV4(
                    super::language_v4::LanguageErrorV4::Base(
                        super::language::LanguageError::InvalidState,
                    ),
                ),
            )));
        }

        match experience {
            LanguageExperienceV7::Sequential {
                predecessor,
                current,
            } => {
                let v6 = self.v6_model().adapt(
                    &self.to_v6_state(state),
                    &LanguageExperienceV6::Sequential {
                        predecessor: *predecessor,
                        current: *current,
                    },
                )?;
                Ok(self.apply_v6_state(state, v6))
            }
            LanguageExperienceV7::AssessConsequenceEquivalence(profile) => {
                let v6 = self.v6_model().adapt(
                    &self.to_v6_state(state),
                    &LanguageExperienceV6::AssessConsequenceEquivalence(*profile),
                )?;
                Ok(self.apply_v6_state(state, v6))
            }
            LanguageExperienceV7::RecognizeContext(cue) => {
                let v6 = self.v6_model().adapt(
                    &self.to_v6_state(state),
                    &LanguageExperienceV6::RecognizeContext(cue.clone()),
                )?;
                Ok(self.apply_v6_state(state, v6))
            }
            LanguageExperienceV7::RecordContextApplicability(profile) => {
                let v6 = self.v6_model().adapt(
                    &self.to_v6_state(state),
                    &LanguageExperienceV6::RecordContextApplicability(*profile),
                )?;
                Ok(self.apply_v6_state(state, v6))
            }
            LanguageExperienceV7::InferConsequenceProfileFromLearnedApplicability => {
                let v6 = self.v6_model().adapt(
                    &self.to_v6_state(state),
                    &LanguageExperienceV6::InferConsequenceProfileFromLearnedApplicability,
                )?;
                Ok(self.apply_v6_state(state, v6))
            }
            LanguageExperienceV7::RecordObservedConsequence(observed) => {
                self.record_observed_consequence(state, *observed)
            }
            LanguageExperienceV7::InferConsequenceProfileFromOutcomeApplicability => {
                self.infer_profile_from_outcome_applicability(state)
            }
        }
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        if !self.valid_parameters() || !self.valid_state(state) {
            return Err(LanguageErrorV7::BaseV6(LanguageErrorV6::BaseV5(
                super::language_v5::LanguageErrorV5::BaseV4(
                    super::language_v4::LanguageErrorV4::Base(
                        super::language::LanguageError::InvalidState,
                    ),
                ),
            )));
        }
        self.v6_model()
            .observe(&self.to_v6_state(state), profile)
            .map_err(Into::into)
    }
}
