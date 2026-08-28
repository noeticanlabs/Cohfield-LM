use crate::{AdaptiveContinuationModel, StateRoles};

use super::language::{LanguageInput, LanguageObservationProfile, LanguageResponse, SurfaceSymbol};
use super::language_v2::InternalEquivalenceProfile;
use super::language_v7::{
    CohfieldLanguageModelV7, LanguageErrorV7, LanguageExperienceV7,
    LanguageRelationalConfigurationV7, LanguageStateV7,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DerivedAbstractionIdentityV8 {
    pub profile: InternalEquivalenceProfile,
    pub members: [bool; 4],
}

impl DerivedAbstractionIdentityV8 {
    pub fn contains(self, symbol: SurfaceSymbol) -> bool {
        self.members[symbol.index()]
    }

    pub fn member_count(self) -> usize {
        self.members.iter().filter(|&&member| member).count()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DerivedAbstractionFormationRecordV8 {
    pub epoch: u64,
    pub abstraction: DerivedAbstractionIdentityV8,
    pub source_assessment_epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbstractionToSymbolRelationV8 {
    pub abstraction: DerivedAbstractionIdentityV8,
    pub target: SurfaceSymbol,
    pub weight: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageRelationalConfigurationV8 {
    pub parent: LanguageRelationalConfigurationV7,
    pub derived_abstractions: Vec<DerivedAbstractionIdentityV8>,
    pub abstraction_formation_history: Vec<DerivedAbstractionFormationRecordV8>,
    pub abstraction_relations: Vec<AbstractionToSymbolRelationV8>,
    pub active_derived_abstraction: Option<DerivedAbstractionIdentityV8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageStateV8 {
    pub x: [f64; 4],
    pub theta: [f64; 4],
    pub relational: LanguageRelationalConfigurationV8,
}

impl LanguageStateV8 {
    pub fn equalized_from(state: &Self) -> Self {
        let mut next = state.clone();
        next.x = [0.0; 4];
        next.theta = [1.0; 4];
        next
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageErrorV8 {
    BaseV7(LanguageErrorV7),
    ProfileNotAssessed,
    NonEquivalenceAssessment,
    NoNontrivialAbstraction,
    UnknownDerivedAbstraction,
    InvalidDerivedAbstractionState,
}

impl From<LanguageErrorV7> for LanguageErrorV8 {
    fn from(value: LanguageErrorV7) -> Self {
        Self::BaseV7(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageExperienceV8 {
    Parent(LanguageExperienceV7),
    FormDerivedAbstractions(InternalEquivalenceProfile),
    ActivateDerivedAbstraction(DerivedAbstractionIdentityV8),
    DeactivateDerivedAbstraction,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohfieldLanguageModelV8 {
    pub parent: CohfieldLanguageModelV7,
}

impl Default for CohfieldLanguageModelV8 {
    fn default() -> Self {
        Self {
            parent: CohfieldLanguageModelV7::default(),
        }
    }
}

impl CohfieldLanguageModelV8 {
    fn to_v7_state(&self, state: &LanguageStateV8) -> LanguageStateV7 {
        LanguageStateV7 {
            x: state.x,
            theta: state.theta,
            relational: state.relational.parent.clone(),
        }
    }

    fn apply_v7_state(&self, state: &LanguageStateV8, v7: LanguageStateV7) -> LanguageStateV8 {
        let mut next = state.clone();
        next.x = v7.x;
        next.theta = v7.theta;
        next.relational.parent = v7.relational;
        next
    }

    pub fn migrate_from_v7(&self, state: &LanguageStateV7) -> Result<LanguageStateV8, LanguageErrorV8> {
        let next = LanguageStateV8 {
            x: state.x,
            theta: state.theta,
            relational: LanguageRelationalConfigurationV8 {
                parent: state.relational.clone(),
                derived_abstractions: Vec::new(),
                abstraction_formation_history: Vec::new(),
                abstraction_relations: Vec::new(),
                active_derived_abstraction: None,
            },
        };
        if !self.valid_state(&next) {
            return Err(LanguageErrorV8::InvalidDerivedAbstractionState);
        }
        Ok(next)
    }

    pub fn equivalence_for_profile(
        &self,
        state: &LanguageStateV8,
        profile: InternalEquivalenceProfile,
    ) -> Result<Option<[[bool; 4]; 4]>, LanguageErrorV8> {
        self.parent
            .equivalence_for_profile(&self.to_v7_state(state), profile)
            .map_err(Into::into)
    }

    fn latest_assessment_epoch(
        &self,
        state: &LanguageStateV8,
        profile: InternalEquivalenceProfile,
    ) -> Option<u64> {
        state
            .relational
            .parent
            .assessment_history
            .iter()
            .filter(|record| record.profile == profile)
            .map(|record| record.epoch)
            .max()
    }

    fn relation_value(matrix: &[[bool; 4]; 4], left: usize, right: usize) -> bool {
        left == right || matrix[left][right]
    }

    fn valid_equivalence_relation(matrix: &[[bool; 4]; 4]) -> bool {
        for left in 0..4 {
            for right in 0..4 {
                if matrix[left][right] != matrix[right][left] {
                    return false;
                }
            }
        }
        for left in 0..4 {
            for middle in 0..4 {
                for right in 0..4 {
                    if Self::relation_value(matrix, left, middle)
                        && Self::relation_value(matrix, middle, right)
                        && !Self::relation_value(matrix, left, right)
                    {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn derived_identities(
        &self,
        state: &LanguageStateV8,
        profile: InternalEquivalenceProfile,
    ) -> Result<Vec<DerivedAbstractionIdentityV8>, LanguageErrorV8> {
        let matrix = self
            .equivalence_for_profile(state, profile)?
            .ok_or(LanguageErrorV8::ProfileNotAssessed)?;
        if !Self::valid_equivalence_relation(&matrix) {
            return Err(LanguageErrorV8::NonEquivalenceAssessment);
        }

        let mut visited = [false; 4];
        let mut identities = Vec::new();
        for source in 0..4 {
            if visited[source] {
                continue;
            }
            let mut members = [false; 4];
            for candidate in 0..4 {
                if Self::relation_value(&matrix, source, candidate) {
                    members[candidate] = true;
                    visited[candidate] = true;
                }
            }
            let member_count = members.iter().filter(|&&member| member).count();
            if member_count >= 2 {
                identities.push(DerivedAbstractionIdentityV8 { profile, members });
            }
        }
        if identities.is_empty() {
            return Err(LanguageErrorV8::NoNontrivialAbstraction);
        }
        Ok(identities)
    }

    fn form_derived_abstractions(
        &self,
        state: &LanguageStateV8,
        profile: InternalEquivalenceProfile,
    ) -> Result<LanguageStateV8, LanguageErrorV8> {
        if !self.valid_state(state) {
            return Err(LanguageErrorV8::InvalidDerivedAbstractionState);
        }
        let source_assessment_epoch = self
            .latest_assessment_epoch(state, profile)
            .ok_or(LanguageErrorV8::ProfileNotAssessed)?;
        let identities = self.derived_identities(state, profile)?;

        let mut next = state.clone();
        for identity in identities {
            if !next.relational.derived_abstractions.contains(&identity) {
                next.relational.derived_abstractions.push(identity);
            }
            let provenance_exists = next
                .relational
                .abstraction_formation_history
                .iter()
                .any(|record| {
                    record.abstraction == identity
                        && record.source_assessment_epoch == source_assessment_epoch
                });
            if !provenance_exists {
                let epoch = next
                    .relational
                    .abstraction_formation_history
                    .last()
                    .map(|record| record.epoch + 1)
                    .unwrap_or(1);
                next.relational
                    .abstraction_formation_history
                    .push(DerivedAbstractionFormationRecordV8 {
                        epoch,
                        abstraction: identity,
                        source_assessment_epoch,
                    });
            }
        }
        Ok(next)
    }

    fn activate_derived_abstraction(
        &self,
        state: &LanguageStateV8,
        identity: DerivedAbstractionIdentityV8,
    ) -> Result<LanguageStateV8, LanguageErrorV8> {
        if !state.relational.derived_abstractions.contains(&identity) {
            return Err(LanguageErrorV8::UnknownDerivedAbstraction);
        }
        let mut next = state.clone();
        next.relational.active_derived_abstraction = Some(identity);
        Ok(next)
    }

    fn abstraction_relation_weight(
        state: &LanguageStateV8,
        abstraction: DerivedAbstractionIdentityV8,
        target: SurfaceSymbol,
    ) -> f64 {
        state
            .relational
            .abstraction_relations
            .iter()
            .find(|relation| relation.abstraction == abstraction && relation.target == target)
            .map(|relation| relation.weight)
            .unwrap_or(0.0)
    }

    pub fn learned_abstraction_relation(
        &self,
        state: &LanguageStateV8,
        abstraction: DerivedAbstractionIdentityV8,
        target: SurfaceSymbol,
    ) -> Result<f64, LanguageErrorV8> {
        if !state.relational.derived_abstractions.contains(&abstraction) {
            return Err(LanguageErrorV8::UnknownDerivedAbstraction);
        }
        Ok(Self::abstraction_relation_weight(state, abstraction, target))
    }

    fn apply_abstraction_learning(
        &self,
        state: &LanguageStateV8,
        predecessor: Option<SurfaceSymbol>,
        current: SurfaceSymbol,
    ) -> LanguageStateV8 {
        let mut next = state.clone();
        for relation in &mut next.relational.abstraction_relations {
            relation.weight *= 1.0 - self.parent.psi_decay;
        }

        let Some(predecessor) = predecessor else {
            return next;
        };

        let matching: Vec<DerivedAbstractionIdentityV8> = next
            .relational
            .derived_abstractions
            .iter()
            .copied()
            .filter(|identity| identity.contains(predecessor))
            .collect();

        for abstraction in matching {
            if let Some(relation) = next
                .relational
                .abstraction_relations
                .iter_mut()
                .find(|relation| {
                    relation.abstraction == abstraction && relation.target == current
                })
            {
                relation.weight += self.parent.psi_gain;
            } else {
                next.relational
                    .abstraction_relations
                    .push(AbstractionToSymbolRelationV8 {
                        abstraction,
                        target: current,
                        weight: self.parent.psi_gain,
                    });
            }
        }
        next
    }

    fn abstraction_activation(
        state: &LanguageStateV8,
        abstraction: DerivedAbstractionIdentityV8,
    ) -> f64 {
        let mut sum = 0.0;
        let mut count = 0usize;
        for (index, &member) in abstraction.members.iter().enumerate() {
            if member {
                sum += state.x[index];
                count += 1;
            }
        }
        if count == 0 {
            0.0
        } else {
            sum / count as f64
        }
    }

    fn step(
        &self,
        state: &LanguageStateV8,
        input: &LanguageInput,
    ) -> Result<LanguageStateV8, LanguageErrorV8> {
        if !self.valid_state(state) {
            return Err(LanguageErrorV8::InvalidDerivedAbstractionState);
        }
        let parent_next = self
            .parent
            .evolve(&self.to_v7_state(state), input, 1.0)?;
        let mut next = self.apply_v7_state(state, parent_next);

        if let Some(abstraction) = state.relational.active_derived_abstraction {
            let activation = Self::abstraction_activation(state, abstraction);
            for target in SurfaceSymbol::ALL {
                let weight = Self::abstraction_relation_weight(state, abstraction, target);
                next.x[target.index()] += self.parent.relational_gain * weight * activation;
            }
        }
        Ok(next)
    }

    fn valid_state(&self, state: &LanguageStateV8) -> bool {
        if self
            .parent
            .evolve(&self.to_v7_state(state), &LanguageInput::zero(), 0.0)
            .is_err()
        {
            return false;
        }
        if state
            .relational
            .derived_abstractions
            .iter()
            .any(|identity| identity.member_count() < 2)
        {
            return false;
        }
        for (index, identity) in state.relational.derived_abstractions.iter().enumerate() {
            if state.relational.derived_abstractions[..index].contains(identity) {
                return false;
            }
        }
        if state
            .relational
            .abstraction_formation_history
            .iter()
            .any(|record| {
                !state.relational.derived_abstractions.contains(&record.abstraction)
                    || !state
                        .relational
                        .parent
                        .assessment_history
                        .iter()
                        .any(|assessment| {
                            assessment.profile == record.abstraction.profile
                                && assessment.epoch == record.source_assessment_epoch
                        })
            })
        {
            return false;
        }
        if state.relational.abstraction_relations.iter().any(|relation| {
            !relation.weight.is_finite()
                || !state
                    .relational
                    .derived_abstractions
                    .contains(&relation.abstraction)
        }) {
            return false;
        }
        for (index, relation) in state.relational.abstraction_relations.iter().enumerate() {
            if state.relational.abstraction_relations[..index]
                .iter()
                .any(|earlier| {
                    earlier.abstraction == relation.abstraction && earlier.target == relation.target
                })
            {
                return false;
            }
        }
        match state.relational.active_derived_abstraction {
            Some(identity) => state.relational.derived_abstractions.contains(&identity),
            None => true,
        }
    }

    pub fn expose(
        &self,
        initial: &LanguageStateV8,
        pattern: &[SurfaceSymbol],
        repeats: usize,
    ) -> Result<LanguageStateV8, LanguageErrorV8> {
        if pattern.is_empty() || repeats == 0 || !self.valid_state(initial) {
            return Err(LanguageErrorV8::InvalidDerivedAbstractionState);
        }
        let mut state = initial.clone();
        let mut predecessor = None;
        for _ in 0..repeats {
            for &symbol in pattern {
                state = self.step(&state, &LanguageInput::symbol(symbol))?;
                state = self.adapt(
                    &state,
                    &LanguageExperienceV8::Parent(LanguageExperienceV7::Sequential {
                        predecessor,
                        current: symbol,
                    }),
                )?;
                predecessor = Some(symbol);
            }
        }
        Ok(state)
    }
}

impl AdaptiveContinuationModel for CohfieldLanguageModelV8 {
    type State = LanguageStateV8;
    type Fast = [f64; 4];
    type LocalCondition = [f64; 4];
    type RelationalConfiguration = LanguageRelationalConfigurationV8;
    type Input = LanguageInput;
    type Experience = LanguageExperienceV8;
    type ObservationProfile = LanguageObservationProfile;
    type Response = LanguageResponse;
    type Error = LanguageErrorV8;

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
            return Err(LanguageErrorV8::InvalidDerivedAbstractionState);
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
        if !self.valid_state(state) {
            return Err(LanguageErrorV8::InvalidDerivedAbstractionState);
        }
        match experience {
            LanguageExperienceV8::Parent(parent_experience) => {
                let parent_next = self
                    .parent
                    .adapt(&self.to_v7_state(state), parent_experience)?;
                let next = self.apply_v7_state(state, parent_next);
                if let LanguageExperienceV7::Sequential {
                    predecessor,
                    current,
                } = parent_experience
                {
                    Ok(self.apply_abstraction_learning(&next, *predecessor, *current))
                } else {
                    Ok(next)
                }
            }
            LanguageExperienceV8::FormDerivedAbstractions(profile) => {
                self.form_derived_abstractions(state, *profile)
            }
            LanguageExperienceV8::ActivateDerivedAbstraction(identity) => {
                self.activate_derived_abstraction(state, *identity)
            }
            LanguageExperienceV8::DeactivateDerivedAbstraction => {
                let mut next = state.clone();
                next.relational.active_derived_abstraction = None;
                Ok(next)
            }
        }
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        if !self.valid_state(state) || profile.probes.is_empty() {
            return Err(LanguageErrorV8::InvalidDerivedAbstractionState);
        }
        let mut vectors =
            Vec::with_capacity(profile.probes.len() * (2 + profile.continuation_steps));
        for probe in &profile.probes {
            let mut local = LanguageStateV8::equalized_from(state);
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
