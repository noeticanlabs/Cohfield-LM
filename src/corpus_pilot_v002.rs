//! CF-LM Corpus Pilot v0.02 — order-2 visible-history continuation.
//!
//! This experimental runtime preserves the v0.01 first-order relation and adds
//! a sparse order-2 path relation over visible bytes. Global relation decay is
//! represented by a shared scale factor so corpus exposure remains practical
//! while preserving the same multiplicative-decay equation.

use crate::corpus_bridge_v001::{CorpusRecord, BYTE_COUNT};
use std::collections::HashMap;

const PAIR_COUNT: usize = BYTE_COUNT * BYTE_COUNT;
const RENORMALIZE_AT: f64 = 1e-100;

fn pair_index(a: u8, b: u8) -> usize {
    a as usize * BYTE_COUNT + b as usize
}

fn path_key(a: u8, b: u8, c: u8) -> u32 {
    ((a as u32) << 16) | ((b as u32) << 8) | c as u32
}

#[derive(Clone, Debug)]
pub struct HistoryState {
    pair_unscaled: Vec<f64>,
    path_unscaled: HashMap<u32, f64>,
    scale: f64,
    pub adaptation_step: u64,
}

impl HistoryState {
    pub fn initial() -> Self {
        Self {
            pair_unscaled: vec![0.0; PAIR_COUNT],
            path_unscaled: HashMap::new(),
            scale: 1.0,
            adaptation_step: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HistoryModel {
    pub beta: f64,
    pub input_gain: f64,
    pub pair_gain: f64,
    pub path_gain: f64,
    pub decay: f64,
    pub learn_gain: f64,
}

impl Default for HistoryModel {
    fn default() -> Self {
        Self {
            beta: 0.50,
            input_gain: 0.50,
            pair_gain: 0.20,
            path_gain: 0.20,
            decay: 0.02,
            learn_gain: 0.08,
        }
    }
}

impl HistoryModel {
    fn renormalize_if_needed(&self, state: &mut HistoryState) {
        if state.scale >= RENORMALIZE_AT {
            return;
        }
        let scale = state.scale;
        for value in &mut state.pair_unscaled {
            *value *= scale;
        }
        state.path_unscaled.retain(|_, value| {
            *value *= scale;
            value.abs() > 1e-15
        });
        state.scale = 1.0;
    }

    fn adapt(&self, state: &mut HistoryState, prev2: Option<u8>, prev1: u8, current: u8) {
        state.scale *= 1.0 - self.decay;
        state.adaptation_step += 1;
        self.renormalize_if_needed(state);

        let inverse_scale = 1.0 / state.scale;
        state.pair_unscaled[pair_index(prev1, current)] += self.learn_gain * inverse_scale;
        if let Some(first) = prev2 {
            *state
                .path_unscaled
                .entry(path_key(first, prev1, current))
                .or_insert(0.0) += self.learn_gain * inverse_scale;
        }
    }

    pub fn train(&self, records: &[CorpusRecord], epochs: usize) -> HistoryState {
        let mut state = HistoryState::initial();
        for _ in 0..epochs {
            for record in records {
                let mut prev2 = None;
                let mut prev1 = None;
                for byte in record.input.iter().chain(record.target.iter()).copied() {
                    if let Some(previous) = prev1 {
                        self.adapt(&mut state, prev2, previous, byte);
                    }
                    prev2 = prev1;
                    prev1 = Some(byte);
                }
            }
        }
        state
    }

    fn pair_weight(state: &HistoryState, source: usize, target: usize) -> f64 {
        state.pair_unscaled[source * BYTE_COUNT + target] * state.scale
    }

    fn path_weight(state: &HistoryState, a: u8, b: u8, target: u8) -> f64 {
        state
            .path_unscaled
            .get(&path_key(a, b, target))
            .copied()
            .unwrap_or(0.0)
            * state.scale
    }

    pub fn continuation_field(
        &self,
        state: &HistoryState,
        input: &[u8],
        path_enabled: bool,
    ) -> Vec<f64> {
        let mut field = vec![0.0; BYTE_COUNT];
        let mut prev2 = None;
        let mut prev1 = None;
        for byte in input.iter().copied() {
            field = self.step(state, &field, prev2, prev1, Some(byte), path_enabled);
            prev2 = prev1;
            prev1 = Some(byte);
        }
        self.step(state, &field, prev2, prev1, None, path_enabled)
    }

    fn step(
        &self,
        state: &HistoryState,
        field: &[f64],
        prev2: Option<u8>,
        prev1: Option<u8>,
        input: Option<u8>,
        path_enabled: bool,
    ) -> Vec<f64> {
        let mut next = vec![0.0; BYTE_COUNT];
        for source in 0..BYTE_COUNT {
            let activity = field[source];
            if activity == 0.0 {
                continue;
            }
            for target in 0..BYTE_COUNT {
                let weight = Self::pair_weight(state, source, target);
                if weight != 0.0 {
                    next[target] += self.pair_gain * weight * activity;
                }
            }
        }
        for index in 0..BYTE_COUNT {
            next[index] += self.beta * field[index];
        }
        if path_enabled {
            if let (Some(first), Some(second)) = (prev2, prev1) {
                for target in 0..BYTE_COUNT {
                    let weight = Self::path_weight(state, first, second, target as u8);
                    if weight != 0.0 {
                        next[target] += self.path_gain * weight;
                    }
                }
            }
        }
        if let Some(byte) = input {
            next[byte as usize] += self.input_gain;
        }
        next
    }

    pub fn learned_paths(state: &HistoryState) -> usize {
        state.path_unscaled.len()
    }
}
