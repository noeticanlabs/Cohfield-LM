//! CF-LM Corpus Pilot v0.01 — efficient first-order byte-relation boundary.
//!
//! This pilot deliberately keeps the same V1 persistent relation law used by
//! `corpus_bridge_v001`, but implements global decay lazily so real corpus
//! exposure does not require touching all 65,536 byte relations on every
//! observed byte pair.
//!
//! The scientific question is intentionally narrow: after governed corpus
//! exposure, does this first-order byte substrate show prompt-conditioned
//! holdout continuation beyond shuffled-target, rotated-prompt, and answer-
//! boundary-only controls? A null result is an informative composition/context
//! boundary, not a pipeline failure.

use crate::corpus_bridge_v001::{CorpusRecord, BYTE_COUNT};

pub const ANSWER_BOUNDARY: &[u8] = b"\n\nAssistant: ";
const RELATION_COUNT: usize = BYTE_COUNT * BYTE_COUNT;

fn relation_index(source: u8, target: u8) -> usize {
    source as usize * BYTE_COUNT + target as usize
}

#[derive(Clone, Debug, PartialEq)]
pub struct LazyByteState {
    /// Weight stored at `last_update_step[index]`.
    weights: Vec<f64>,
    last_update_step: Vec<u64>,
    pub adaptation_step: u64,
}

impl LazyByteState {
    pub fn initial() -> Self {
        Self {
            weights: vec![0.0; RELATION_COUNT],
            last_update_step: vec![0; RELATION_COUNT],
            adaptation_step: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LazyByteModel {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub adaptation_enabled: bool,
}

impl Default for LazyByteModel {
    fn default() -> Self {
        Self {
            beta: 0.50,
            input_gain: 0.50,
            relational_gain: 0.20,
            psi_decay: 0.02,
            psi_gain: 0.08,
            adaptation_enabled: true,
        }
    }
}

impl LazyByteModel {
    pub fn without_adaptation() -> Self {
        Self {
            adaptation_enabled: false,
            ..Self::default()
        }
    }

    fn decay_factor(&self) -> f64 {
        1.0 - self.psi_decay
    }

    fn relation_at_step(
        &self,
        state: &LazyByteState,
        index: usize,
        at_step: u64,
    ) -> f64 {
        let stored = state.weights[index];
        if stored == 0.0 {
            return 0.0;
        }
        let last = state.last_update_step[index];
        let elapsed = at_step.saturating_sub(last);
        stored * self.decay_factor().powf(elapsed as f64)
    }

    pub fn relation(&self, state: &LazyByteState, source: u8, target: u8) -> f64 {
        self.relation_at_step(
            state,
            relation_index(source, target),
            state.adaptation_step,
        )
    }

    fn adapt_pair(&self, state: &mut LazyByteState, source: u8, target: u8) {
        if !self.adaptation_enabled {
            return;
        }
        let next_step = state.adaptation_step + 1;
        let index = relation_index(source, target);
        let decayed = self.relation_at_step(state, index, next_step);
        state.weights[index] = decayed + self.psi_gain;
        state.last_update_step[index] = next_step;
        state.adaptation_step = next_step;
    }

    /// Persistent training is exactly the v0.01 predecessor/current adaptation
    /// law. Fast-state evolution is omitted here because it has no causal input
    /// to that law and every record boundary is equalized in v0.01.
    pub fn train(&self, records: &[CorpusRecord], epochs: usize) -> LazyByteState {
        let mut state = LazyByteState::initial();
        for _ in 0..epochs {
            for record in records {
                let mut predecessor: Option<u8> = None;
                for byte in record.input.iter().chain(record.target.iter()).copied() {
                    if let Some(previous) = predecessor {
                        self.adapt_pair(&mut state, previous, byte);
                    }
                    predecessor = Some(byte);
                }
            }
        }
        state
    }

    fn step_field(
        &self,
        state: &LazyByteState,
        current: &[f64],
        input: Option<u8>,
    ) -> Vec<f64> {
        let mut next = vec![0.0; BYTE_COUNT];
        for source in 0..BYTE_COUNT {
            let source_activity = current[source];
            if source_activity == 0.0 {
                continue;
            }
            for target in 0..BYTE_COUNT {
                let relation = self.relation(state, source as u8, target as u8);
                if relation != 0.0 {
                    next[target] += self.relational_gain * relation * source_activity;
                }
            }
        }
        for index in 0..BYTE_COUNT {
            next[index] += self.beta * current[index];
        }
        if let Some(byte) = input {
            next[byte as usize] += self.input_gain;
        }
        next
    }

    pub fn continuation_field(&self, state: &LazyByteState, input: &[u8]) -> Vec<f64> {
        let mut field = vec![0.0; BYTE_COUNT];
        for byte in input.iter().copied() {
            field = self.step_field(state, &field, Some(byte));
        }
        self.step_field(state, &field, None)
    }

    pub fn adaptation_events(records: &[CorpusRecord], epochs: usize) -> u64 {
        let per_epoch: usize = records
            .iter()
            .map(|record| record.input.len() + record.target.len())
            .map(|visible| visible.saturating_sub(1))
            .sum();
        (per_epoch as u64).saturating_mul(epochs as u64)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HoldoutMetrics {
    pub samples: usize,
    pub mean_correct_activation: f64,
    pub mean_rank: f64,
    pub top1_or_tied_rate: f64,
    pub mean_rotated_prompt_correct_activation: f64,
    pub mean_boundary_only_correct_activation: f64,
    pub mean_field_l1_vs_rotated_prompt: f64,
    pub mean_field_l1_vs_boundary_only: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CorpusPilotReport {
    pub true_trained: HoldoutMetrics,
    pub shuffled_target_trained: HoldoutMetrics,
    pub untrained: HoldoutMetrics,
    pub pairing_activation_delta: f64,
    pub prompt_pair_activation_delta: f64,
    pub boundary_activation_delta: f64,
}

fn rank_descending(field: &[f64], target: u8) -> usize {
    let value = field[target as usize];
    1 + field.iter().filter(|&&candidate| candidate > value).count()
}

fn l1(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn boundary_only(input: &[u8]) -> &[u8] {
    if input.ends_with(ANSWER_BOUNDARY) {
        ANSWER_BOUNDARY
    } else {
        let start = input.len().saturating_sub(ANSWER_BOUNDARY.len());
        &input[start..]
    }
}

pub fn evaluate_holdout(
    model: &LazyByteModel,
    state: &LazyByteState,
    records: &[CorpusRecord],
) -> HoldoutMetrics {
    let eligible: Vec<&CorpusRecord> = records
        .iter()
        .filter(|record| !record.target.is_empty())
        .collect();
    if eligible.is_empty() {
        return HoldoutMetrics {
            samples: 0,
            mean_correct_activation: 0.0,
            mean_rank: 0.0,
            top1_or_tied_rate: 0.0,
            mean_rotated_prompt_correct_activation: 0.0,
            mean_boundary_only_correct_activation: 0.0,
            mean_field_l1_vs_rotated_prompt: 0.0,
            mean_field_l1_vs_boundary_only: 0.0,
        };
    }

    let mut correct = Vec::with_capacity(eligible.len());
    let mut ranks = Vec::with_capacity(eligible.len());
    let mut top1 = Vec::with_capacity(eligible.len());
    let mut rotated_correct = Vec::with_capacity(eligible.len());
    let mut boundary_correct = Vec::with_capacity(eligible.len());
    let mut rotated_l1 = Vec::with_capacity(eligible.len());
    let mut boundary_l1 = Vec::with_capacity(eligible.len());

    for (index, record) in eligible.iter().enumerate() {
        let target = record.target[0];
        let actual = model.continuation_field(state, &record.input);
        let rotated_prompt = &eligible[(index + 1) % eligible.len()].input;
        let rotated = model.continuation_field(state, rotated_prompt);
        let boundary = model.continuation_field(state, boundary_only(&record.input));

        let target_activation = actual[target as usize];
        let maximum = actual
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        correct.push(target_activation);
        ranks.push(rank_descending(&actual, target) as f64);
        top1.push(if target_activation >= maximum { 1.0 } else { 0.0 });
        rotated_correct.push(rotated[target as usize]);
        boundary_correct.push(boundary[target as usize]);
        rotated_l1.push(l1(&actual, &rotated));
        boundary_l1.push(l1(&actual, &boundary));
    }

    HoldoutMetrics {
        samples: eligible.len(),
        mean_correct_activation: mean(&correct),
        mean_rank: mean(&ranks),
        top1_or_tied_rate: mean(&top1),
        mean_rotated_prompt_correct_activation: mean(&rotated_correct),
        mean_boundary_only_correct_activation: mean(&boundary_correct),
        mean_field_l1_vs_rotated_prompt: mean(&rotated_l1),
        mean_field_l1_vs_boundary_only: mean(&boundary_l1),
    }
}

pub fn rotate_training_targets(records: &[CorpusRecord]) -> Vec<CorpusRecord> {
    if records.len() < 2 {
        return records.to_vec();
    }
    records
        .iter()
        .enumerate()
        .map(|(index, record)| CorpusRecord {
            input: record.input.clone(),
            target: records[(index + 1) % records.len()].target.clone(),
        })
        .collect()
}

pub fn run_pilot(
    train: &[CorpusRecord],
    holdout: &[CorpusRecord],
    epochs: usize,
) -> CorpusPilotReport {
    let model = LazyByteModel::default();
    let true_state = model.train(train, epochs);
    let shuffled_train = rotate_training_targets(train);
    let shuffled_state = model.train(&shuffled_train, epochs);
    let untrained_state = LazyByteState::initial();

    let true_trained = evaluate_holdout(&model, &true_state, holdout);
    let shuffled_target_trained = evaluate_holdout(&model, &shuffled_state, holdout);
    let untrained = evaluate_holdout(&model, &untrained_state, holdout);

    CorpusPilotReport {
        pairing_activation_delta: true_trained.mean_correct_activation
            - shuffled_target_trained.mean_correct_activation,
        prompt_pair_activation_delta: true_trained.mean_correct_activation
            - true_trained.mean_rotated_prompt_correct_activation,
        boundary_activation_delta: true_trained.mean_correct_activation
            - true_trained.mean_boundary_only_correct_activation,
        true_trained,
        shuffled_target_trained,
        untrained,
    }
}
