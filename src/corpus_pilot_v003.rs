//! CF-LM Corpus Pilot v0.03 — trajectory-trace conditioning.
//!
//! This runtime preserves the first-order byte relation and adds a bounded
//! exponentially retained visible-state trace. Global relation decay is
//! represented by a shared scale factor so the persistent equations remain
//! practical at corpus scale.

use crate::corpus_bridge_v001::{CorpusRecord, BYTE_COUNT};

const RELATION_COUNT: usize = BYTE_COUNT * BYTE_COUNT;
const RENORMALIZE_AT: f64 = 1e-100;

#[derive(Clone, Debug)]
pub struct TraceState {
    pair_unscaled: Vec<f64>,
    omega_unscaled: Vec<f64>,
    scale: f64,
    pub adaptation_step: u64,
}

impl TraceState {
    pub fn initial() -> Self {
        Self {
            pair_unscaled: vec![0.0; RELATION_COUNT],
            omega_unscaled: vec![0.0; RELATION_COUNT],
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
    fn relation_index(source: usize, target: usize) -> usize {
        source * BYTE_COUNT + target
    }

    fn renormalize_if_needed(&self, state: &mut TraceState) {
        if state.scale >= RENORMALIZE_AT {
            return;
        }
        let scale = state.scale;
        for value in &mut state.pair_unscaled {
            *value *= scale;
        }
        for value in &mut state.omega_unscaled {
            *value *= scale;
        }
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
        let inverse_scale = 1.0 / state.scale;

        state.pair_unscaled[Self::relation_index(previous as usize, current as usize)] +=
            self.learn_gain * inverse_scale;
        for source in 0..BYTE_COUNT {
            let activity = history[source];
            if activity != 0.0 {
                state.omega_unscaled[Self::relation_index(source, current as usize)] +=
                    self.learn_gain * activity * inverse_scale;
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

    fn pair_weight(state: &TraceState, source: usize, target: usize) -> f64 {
        state.pair_unscaled[Self::relation_index(source, target)] * state.scale
    }

    fn trace_weight(state: &TraceState, source: usize, target: usize) -> f64 {
        state.omega_unscaled[Self::relation_index(source, target)] * state.scale
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
        for source in 0..BYTE_COUNT {
            if field[source] != 0.0 {
                for target in 0..BYTE_COUNT {
                    let weight = Self::pair_weight(state, source, target);
                    if weight != 0.0 {
                        next[target] += self.pair_gain * weight * field[source];
                    }
                }
            }
            if trace_enabled && history[source] != 0.0 {
                for target in 0..BYTE_COUNT {
                    let weight = Self::trace_weight(state, source, target);
                    if weight != 0.0 {
                        next[target] += self.trace_gain * weight * history[source];
                    }
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
}
