//! CF-LM Corpus Pilot v0.03 — trajectory-trace conditioning.
//!
//! Preregistered successor to v0.02. This module is intentionally not wired
//! into the crate yet; v0.02 must complete its execution gate before v0.03 is
//! interpreted comparatively.

use crate::corpus_bridge_v001::{CorpusRecord, BYTE_COUNT};

#[derive(Clone, Debug)]
pub struct TraceState {
    pub pair: Vec<f64>,
    pub omega: Vec<f64>,
}

impl TraceState {
    pub fn initial() -> Self {
        Self {
            pair: vec![0.0; BYTE_COUNT * BYTE_COUNT],
            omega: vec![0.0; BYTE_COUNT * BYTE_COUNT],
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

    fn decay_relations(&self, state: &mut TraceState) {
        let d = 1.0 - self.relation_decay;
        for value in &mut state.pair {
            *value *= d;
        }
        for value in &mut state.omega {
            *value *= d;
        }
    }

    fn update_trace(&self, h: &mut [f64], x: &[f64]) {
        let r = self.history_retention;
        for i in 0..BYTE_COUNT {
            h[i] = r * h[i] + (1.0 - r) * x[i];
        }
    }

    fn adapt(&self, state: &mut TraceState, h: &[f64], previous: u8, current: u8) {
        self.decay_relations(state);
        state.pair[Self::relation_index(previous as usize, current as usize)] += self.learn_gain;
        for source in 0..BYTE_COUNT {
            let activity = h[source];
            if activity != 0.0 {
                state.omega[Self::relation_index(source, current as usize)] += self.learn_gain * activity;
            }
        }
    }

    pub fn train(&self, records: &[CorpusRecord], epochs: usize) -> TraceState {
        let mut state = TraceState::initial();
        for _ in 0..epochs {
            for record in records {
                let mut x = vec![0.0; BYTE_COUNT];
                let mut h = vec![0.0; BYTE_COUNT];
                let mut previous: Option<u8> = None;
                for byte in record.input.iter().chain(record.target.iter()).copied() {
                    self.update_trace(&mut h, &x);
                    if let Some(prev) = previous {
                        self.adapt(&mut state, &h, prev, byte);
                    }
                    x.fill(0.0);
                    x[byte as usize] = 1.0;
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
        let mut x = vec![0.0; BYTE_COUNT];
        let mut h = vec![0.0; BYTE_COUNT];
        for byte in input.iter().copied() {
            self.update_trace(&mut h, &x);
            x = self.step(state, &x, &h, Some(byte), trace_enabled);
        }
        self.update_trace(&mut h, &x);
        self.step(state, &x, &h, None, trace_enabled)
    }

    fn step(
        &self,
        state: &TraceState,
        x: &[f64],
        h: &[f64],
        input: Option<u8>,
        trace_enabled: bool,
    ) -> Vec<f64> {
        let mut next = vec![0.0; BYTE_COUNT];
        for source in 0..BYTE_COUNT {
            if x[source] != 0.0 {
                for target in 0..BYTE_COUNT {
                    next[target] += self.pair_gain
                        * state.pair[Self::relation_index(source, target)]
                        * x[source];
                }
            }
            if trace_enabled && h[source] != 0.0 {
                for target in 0..BYTE_COUNT {
                    next[target] += self.trace_gain
                        * state.omega[Self::relation_index(source, target)]
                        * h[source];
                }
            }
        }
        for i in 0..BYTE_COUNT {
            next[i] += self.beta * x[i];
        }
        if let Some(byte) = input {
            next[byte as usize] += self.input_gain;
        }
        next
    }
}