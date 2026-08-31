//! CF-LM Corpus Pilot v0.02 — order-2 visible-history continuation.
//!
//! This experimental runtime preserves the v0.01 first-order relation and adds
//! a sparse order-2 path relation over visible bytes. No tokenization, embedding,
//! hidden teacher state, or validation/test adaptation is introduced.

use crate::corpus_bridge_v001::{CorpusRecord, BYTE_COUNT};
use std::collections::HashMap;

const PAIR_COUNT: usize = BYTE_COUNT * BYTE_COUNT;

fn pair_index(a: u8, b: u8) -> usize { a as usize * BYTE_COUNT + b as usize }
fn path_key(a: u8, b: u8, c: u8) -> u32 { ((a as u32) << 16) | ((b as u32) << 8) | c as u32 }

#[derive(Clone, Debug)]
pub struct HistoryState {
    pair: Vec<f64>,
    path: HashMap<u32, f64>,
}

impl HistoryState {
    pub fn initial() -> Self { Self { pair: vec![0.0; PAIR_COUNT], path: HashMap::new() } }
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
        Self { beta: 0.50, input_gain: 0.50, pair_gain: 0.20, path_gain: 0.20, decay: 0.02, learn_gain: 0.08 }
    }
}

impl HistoryModel {
    fn decay_all(&self, s: &mut HistoryState) {
        let d = 1.0 - self.decay;
        for v in &mut s.pair { *v *= d; }
        s.path.retain(|_, v| { *v *= d; v.abs() > 1e-15 });
    }

    fn adapt(&self, s: &mut HistoryState, prev2: Option<u8>, prev1: u8, cur: u8) {
        self.decay_all(s);
        s.pair[pair_index(prev1, cur)] += self.learn_gain;
        if let Some(a) = prev2 { *s.path.entry(path_key(a, prev1, cur)).or_insert(0.0) += self.learn_gain; }
    }

    pub fn train(&self, records: &[CorpusRecord], epochs: usize) -> HistoryState {
        let mut s = HistoryState::initial();
        for _ in 0..epochs {
            for r in records {
                let mut p2 = None;
                let mut p1 = None;
                for b in r.input.iter().chain(r.target.iter()).copied() {
                    if let Some(a) = p1 { self.adapt(&mut s, p2, a, b); }
                    p2 = p1; p1 = Some(b);
                }
            }
        }
        s
    }

    pub fn continuation_field(&self, s: &HistoryState, input: &[u8], path_enabled: bool) -> Vec<f64> {
        let mut x = vec![0.0; BYTE_COUNT];
        let mut p2 = None;
        let mut p1 = None;
        for b in input.iter().copied() {
            x = self.step(s, &x, p2, p1, Some(b), path_enabled);
            p2 = p1; p1 = Some(b);
        }
        self.step(s, &x, p2, p1, None, path_enabled)
    }

    fn step(&self, s: &HistoryState, x: &[f64], p2: Option<u8>, p1: Option<u8>, input: Option<u8>, path_enabled: bool) -> Vec<f64> {
        let mut next = vec![0.0; BYTE_COUNT];
        for source in 0..BYTE_COUNT {
            if x[source] == 0.0 { continue; }
            let row = source * BYTE_COUNT;
            for target in 0..BYTE_COUNT {
                let w = s.pair[row + target];
                if w != 0.0 { next[target] += self.pair_gain * w * x[source]; }
            }
        }
        for i in 0..BYTE_COUNT { next[i] += self.beta * x[i]; }
        if path_enabled {
            if let (Some(a), Some(b)) = (p2, p1) {
                for target in 0..BYTE_COUNT {
                    if let Some(w) = s.path.get(&path_key(a, b, target as u8)) {
                        next[target] += self.path_gain * *w;
                    }
                }
            }
        }
        if let Some(b) = input { next[b as usize] += self.input_gain; }
        next
    }

    pub fn learned_paths(s: &HistoryState) -> usize { s.path.len() }
}