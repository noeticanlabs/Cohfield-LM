//! CF-LM Corpus Pilot v0.03 — trajectory-trace conditioning.
//!
//! This runtime preserves the first-order byte relation and adds a bounded
//! exponentially retained visible-state trace. Relations are sparse and share
//! one global decay scale, preserving the multiplicative-decay law efficiently.

use crate::corpus_bridge_v001::{CorpusRecord, BYTE_COUNT};
use std::collections::HashMap;

const RENORMALIZE_AT: f64 = 1e-100;

fn pair_key(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) | b as u16
}

fn decode_pair(key: u16) -> (usize, usize) {
    (((key >> 8) & 0xff) as usize, (key & 0xff) as usize)
}

#[derive(Clone, Debug)]
pub struct TraceState {
    pair_unscaled: HashMap<u16, f64>,
    omega_unscaled: HashMap<u16, f64>,
    scale: f64,
    pub adaptation_step: u64,
}

impl TraceState {
    pub fn initial() -> Self {
        Self {
            pair_unscaled: HashMap::new(),
            omega_unscaled: HashMap::new(),
            scale: 1.0,
            adaptation_step: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TraceModel {
    pub beta: f64,
    pub input_gain: f64,
    pub pair_gain: f64,
    pub trace_gain: f64,
    pub history_retention: f64,
    pub relation_decay: f64,
    pub learn_gain: f64,
}

impl Default for TraceModel {
    fn default() -> Self {
        Self {
            beta: 0.50,
            input_gain: 0.50,
            pair_gain: 0.20,
            trace_gain: 0.20,
            history_retention: 0.85,
            relation_decay: 0.02,
            learn_gain: 0.08,
        }
    }
}

impl TraceModel {
    fn renormalize_if_needed(&self, state: &mut TraceState) {
        if state.scale >= RENORMALIZE_AT {
            return;
        }
        let scale = state.scale;
        state.pair_unscaled.retain(|_, value| {
            *value *= scale;
            value.abs() > 1e-15
        });
        state.omega_unscaled.retain(|_, value| {
            *value *= scale;
            value.abs() > 1e-15
        });
        state.scale = 1.0;
    }

    fn update_trace(&self, history: &mut [f64], field: &[f64]) {
        let retention = self.history_retention;
        for index in 0..BYTE_COUNT {
            history[index] = retention * history[index] + (1.0 - retention) * field[index];
        }
    }

    fn adapt(&self, state: &mut TraceState, history: &[f64], previous: u8, current: u8) {
        state.scale *= 1.0 - self.relation_decay;
        state.adaptation_step += 1;
        self.renormalize_if_needed(state);
        let increment = self.learn_gain / state.scale;

        *state
            .pair_unscaled
            .entry(pair_key(previous, current))
            .or_insert(0.0) += increment;
        for source in 0..BYTE_COUNT {
            let activity = history[source];
            if activity != 0.0 {
                *state
                    .omega_unscaled
                    .entry(pair_key(source as u8, current))
                    .or_insert(0.0) += increment * activity;
            }
        }
    }

    pub fn train(&self, records: &[CorpusRecord], epochs: usize) -> TraceState {
        let mut state = TraceState::initial();
        for _ in 0..epochs {
            for record in records {
                let mut field = vec![0.0; BYTE_COUNT];
                let mut history = vec![0.0; BYTE_COUNT];
                let mut previous = None;
                for byte in record.input.iter().chain(record.target.iter()).copied() {
                    self.update_trace(&mut history, &field);
                    if let Some(prev) = previous {
                        self.adapt(&mut state, &history, prev, byte);
                    }
                    field.fill(0.0);
                    field[byte as usize] = 1.0;
                    previous = Some(byte);
                }
            }
        }
        state
    }

    pub fn continuation_field(
        &self,
        state: &TraceState,
        input: &[u8],
        trace_enabled: bool,
    ) -> Vec<f64> {
        let mut field = vec![0.0; BYTE_COUNT];
        let mut history = vec![0.0; BYTE_COUNT];
        for byte in input.iter().copied() {
            self.update_trace(&mut history, &field);
            field = self.step(state, &field, &history, Some(byte), trace_enabled);
        }
        self.update_trace(&mut history, &field);
        self.step(state, &field, &history, None, trace_enabled)
    }

    fn step(
        &self,
        state: &TraceState,
        field: &[f64],
        history: &[f64],
        input: Option<u8>,
        trace_enabled: bool,
    ) -> Vec<f64> {
        let mut next = vec![0.0; BYTE_COUNT];
        for (&key, &unscaled) in &state.pair_unscaled {
            let (source, target) = decode_pair(key);
            let activity = field[source];
            if activity != 0.0 {
                next[target] += self.pair_gain * (unscaled * state.scale) * activity;
            }
        }
        if trace_enabled {
            for (&key, &unscaled) in &state.omega_unscaled {
                let (source, target) = decode_pair(key);
                let activity = history[source];
                if activity != 0.0 {
                    next[target] += self.trace_gain * (unscaled * state.scale) * activity;
                }
            }
        }
        for index in 0..BYTE_COUNT {
            next[index] += self.beta * field[index];
        }
        if let Some(byte) = input {
            next[byte as usize] += self.input_gain;
        }
        next
    }

    pub fn learned_pairs(state: &TraceState) -> usize {
        state.pair_unscaled.len()
    }

    pub fn learned_trace_relations(state: &TraceState) -> usize {
        state.omega_unscaled.len()
    }
}
