//! CF-LM Teacher Bridge v0.04 — role-preserving structural binding.
//!
//! v0.03 established a boundary: plain composition cannot infer an unlearned
//! edge, CF-LM-015-style member abstraction can broadcast learned member
//! consequences, and a pooled target mechanism is non-specific. v0.04 asks the
//! next narrower question: can a learned relation schema preserve *which*
//! target corresponds to an active source without storing the withheld direct
//! edge?
//!
//! The teacher supplies visible structural experience only. The held-out
//! `B3 -> C3` relation is never taught. Unlike v0.03, `C3` is not an unseen
//! symbol: it appears only through the anchor episode `A3 -> C3`. The runtime
//! must therefore identify C3 through learned relational structure, not invent
//! an unseen symbol.
//!
//! Known source/target families (`B_FAMILY`, `C_FAMILY`) remain designer-supplied
//! in this bounded experiment. Pair identity is *not* supplied. Pairing is
//! derived from cosine overlap between each symbol's learned incoming-relation
//! signature. A global B->C schema strength is learned from the two visible
//! B->C examples (`B1->C1`, `B2->C2`).

use crate::teacher_bridge_v003::{
    run as run_v3, Mechanism as V3Mechanism, V3Curriculum, V3Episode, V3Model, V3State, S,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V4Mechanism {
    Plain,
    StructuralBinding,
}

pub struct V4Curriculum;

impl V4Curriculum {
    /// Frozen v0.04 curriculum. B3->C3 is deliberately withheld.
    pub fn llm_authored() -> V3Curriculum {
        V3Curriculum {
            episodes: vec![
                V3Episode {
                    source: S::A1,
                    target: S::B1,
                },
                V3Episode {
                    source: S::A2,
                    target: S::B2,
                },
                V3Episode {
                    source: S::A3,
                    target: S::B3,
                },
                V3Episode {
                    source: S::A1,
                    target: S::C1,
                },
                V3Episode {
                    source: S::A2,
                    target: S::C2,
                },
                V3Episode {
                    source: S::A3,
                    target: S::C3,
                },
                V3Episode {
                    source: S::B1,
                    target: S::C1,
                },
                V3Episode {
                    source: S::B2,
                    target: S::C2,
                },
            ],
            epochs: 64,
        }
    }

    /// Structural anchors remain but no visible B->C schema examples exist.
    pub fn anchors_only() -> V3Curriculum {
        V3Curriculum {
            episodes: vec![
                V3Episode {
                    source: S::A1,
                    target: S::B1,
                },
                V3Episode {
                    source: S::A2,
                    target: S::B2,
                },
                V3Episode {
                    source: S::A3,
                    target: S::B3,
                },
                V3Episode {
                    source: S::A1,
                    target: S::C1,
                },
                V3Episode {
                    source: S::A2,
                    target: S::C2,
                },
                V3Episode {
                    source: S::A3,
                    target: S::C3,
                },
            ],
            epochs: 64,
        }
    }

    /// Remove only A3->C3. This destroys the structural evidence pairing B3
    /// with C3 while leaving the two visible B->C schema examples intact.
    pub fn without_third_target_anchor() -> V3Curriculum {
        V3Curriculum {
            episodes: vec![
                V3Episode {
                    source: S::A1,
                    target: S::B1,
                },
                V3Episode {
                    source: S::A2,
                    target: S::B2,
                },
                V3Episode {
                    source: S::A3,
                    target: S::B3,
                },
                V3Episode {
                    source: S::A1,
                    target: S::C1,
                },
                V3Episode {
                    source: S::A2,
                    target: S::C2,
                },
                V3Episode {
                    source: S::B1,
                    target: S::C1,
                },
                V3Episode {
                    source: S::B2,
                    target: S::C2,
                },
            ],
            epochs: 64,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V4State {
    pub base: V3State,
    /// Learned structural pairing evidence. Only B->C entries are populated.
    pub slot_affinity: [[f64; 9]; 9],
    /// Global B->C schema strength learned from visible B->C examples.
    pub binding_gain: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct V4Model {
    pub base: V3Model,
    pub mechanism: V4Mechanism,
}

#[derive(Clone, Debug, PartialEq)]
pub struct V4Probe {
    pub trajectory: Vec<[f64; 9]>,
}

impl V4Probe {
    pub fn activation(&self, step: usize, symbol: S) -> Option<f64> {
        self.trajectory
            .get(step)
            .map(|frame| frame[symbol.index()])
    }
}

#[derive(Clone, Debug)]
pub struct V4Runner {
    pub model: V4Model,
    pub state: V4State,
}

fn incoming_signature_cosine(state: &V3State, left: S, right: S) -> f64 {
    let mut dot = 0.0;
    let mut left_norm = 0.0;
    let mut right_norm = 0.0;
    for predecessor in S::ALL {
        let l = state.psi[predecessor.index()][left.index()];
        let r = state.psi[predecessor.index()][right.index()];
        dot += l * r;
        left_norm += l * l;
        right_norm += r * r;
    }
    if left_norm <= f64::EPSILON || right_norm <= f64::EPSILON {
        0.0
    } else {
        dot / (left_norm.sqrt() * right_norm.sqrt())
    }
}

fn derive_slot_affinity(state: &V3State) -> [[f64; 9]; 9] {
    let mut affinity = [[0.0; 9]; 9];
    for &source in &S::B_FAMILY {
        for &target in &S::C_FAMILY {
            affinity[source.index()][target.index()] =
                incoming_signature_cosine(state, source, target);
        }
    }
    affinity
}

fn derive_binding_gain(state: &V3State, curriculum: &V3Curriculum) -> f64 {
    let mut total = 0.0;
    let mut count = 0usize;
    for episode in &curriculum.episodes {
        if episode.source.is_b() && episode.target.is_c() {
            total += state.psi[episode.source.index()][episode.target.index()];
            count += 1;
        }
    }
    if count == 0 {
        0.0
    } else {
        total / count as f64
    }
}

impl V4Model {
    fn step(&self, state: &V4State, input: [f64; 9]) -> V4State {
        let mut next = state.clone();
        let mut relational = [0.0; 9];
        for (target, value) in relational.iter_mut().enumerate() {
            for (source, source_activity) in state.base.x.iter().enumerate() {
                *value += state.base.psi[source][target] * source_activity;
            }
        }

        for (i, x) in next.base.x.iter_mut().enumerate() {
            *x = self.base.beta * state.base.x[i]
                + self.base.input_gain * input[i]
                + self.base.relational_gain * relational[i];
        }

        if self.mechanism == V4Mechanism::StructuralBinding {
            for &source in &S::B_FAMILY {
                let source_activity = state.base.x[source.index()];
                for &target in &S::C_FAMILY {
                    let affinity = state.slot_affinity[source.index()][target.index()];
                    next.base.x[target.index()] +=
                        self.base.relational_gain * state.binding_gain * affinity * source_activity;
                }
            }
        }
        next
    }

    pub fn probe_teacher_off(
        &self,
        trained: &V4State,
        start: S,
        continuation_steps: usize,
    ) -> V4Probe {
        let mut state = trained.clone();
        state.base = V3State::equalized_from(&trained.base);
        let mut trajectory = Vec::with_capacity(continuation_steps + 1);

        let mut input = [0.0; 9];
        input[start.index()] = 1.0;
        state = self.step(&state, input);
        trajectory.push(state.base.x);

        for _ in 0..continuation_steps {
            state = self.step(&state, [0.0; 9]);
            trajectory.push(state.base.x);
        }
        V4Probe { trajectory }
    }
}

pub fn run(mechanism: V4Mechanism, curriculum: &V3Curriculum) -> V4Runner {
    // Train through the already-frozen v0.03 plain substrate. v0.04 adds no new
    // adaptation to Psi; it derives a structural binding layer only afterward.
    let v3 = run_v3(V3Mechanism::Plain, curriculum);
    let slot_affinity = derive_slot_affinity(&v3.state);
    let binding_gain = derive_binding_gain(&v3.state, curriculum);
    let model = V4Model {
        base: v3.model,
        mechanism,
    };
    let state = V4State {
        base: v3.state,
        slot_affinity,
        binding_gain,
    };
    V4Runner { model, state }
}
