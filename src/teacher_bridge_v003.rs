//! CF-LM Teacher Bridge v0.03 — larger-alphabet structural-transfer batch.
//!
//! The v0.03 batch asks whether experience across several instances can produce
//! a reusable relational abstraction that applies to an unseen instance — the
//! first place the LLM teacher starts testing *structural generalization*
//! rather than graph traversal of already-taught local edges (v0.01/v0.02).
//!
//! The surface is a bridge-scoped synthetic alphabet of nine symbols over a
//! `[f64;9]` substrate that replicates the verified CF-LM `language.rs`
//! dynamics. The shared four-symbol profile is deliberately NOT changed, so the
//! full inherited CF-LM regression gate stays green.
//!
//! Curriculum:
//! ```text
//!   A1->B1, A2->B2, A3->B3      (three B-node instances across a shared A family)
//!   B1->C1, B2->C2              (B->C relation on the first two instances)
//!   B3->C3                      (WITHHELD — never taught; Psi[B3,C3] == 0)
//! ```
//!
//! Three exact-matched mechanisms (same curriculum, epochs, initialization,
//! evaluation; the toggle is the only difference):
//!
//! - `Plain` (`Mechanism::Plain`): plain relational composition of learned edges.
//!   Preregistered expectation: **no** C activation on the held-out member.
//! - `Member` (`Mechanism::MemberAbstraction`): CF-LM-015-style derived
//!   abstraction over the B family with mean-member activation and per-target
//!   abstraction->symbol relations. The unseen sibling B3 leverages the pooled
//!   taught consequences of its siblings, but a wholly-unseen target (C3) stays
//!   silent. Positive-transfer hypothesis, hedged.
//! - `Target` (`Mechanism::Target`): `Member` plus a discriminated target-pool
//!   generalization so the pooled B->C relation can reach the withheld new
//!   target C3. Documented as a *theorized* generalization (not the frozen
//!   CF-LM-015 primitive). The only route to B3 -> C3 without storing a direct
//!   Psi[B3,C3] edge.
//!
//! The CF-LM-015 formation semantics are mirrored: an abstraction is formed
//! only after the visible curriculum; abstraction-to-symbol relations are built
//! by member experience using the same decay/gain constants as `Psi`.

/// Bridge-scoped surface symbol for v0.03 (nine symbols over three families).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum S {
    A1,
    A2,
    A3,
    B1,
    B2,
    B3,
    C1,
    C2,
    C3,
}

impl S {
    pub const ALL: [S; 9] = [
        S::A1,
        S::A2,
        S::A3,
        S::B1,
        S::B2,
        S::B3,
        S::C1,
        S::C2,
        S::C3,
    ];
    pub const B_FAMILY: [S; 3] = [S::B1, S::B2, S::B3];
    pub const C_FAMILY: [S; 3] = [S::C1, S::C2, S::C3];

    pub fn index(self) -> usize {
        match self {
            S::A1 => 0,
            S::A2 => 1,
            S::A3 => 2,
            S::B1 => 3,
            S::B2 => 4,
            S::B3 => 5,
            S::C1 => 6,
            S::C2 => 7,
            S::C3 => 8,
        }
    }

    pub fn one_hot(self) -> [f64; 9] {
        let mut out = [0.0; 9];
        out[self.index()] = 1.0;
        out
    }

    pub fn is_b(self) -> bool {
        matches!(self, S::B1 | S::B2 | S::B3)
    }

    pub fn is_c(self) -> bool {
        matches!(self, S::C1 | S::C2 | S::C3)
    }
}

/// The single matched toggle of the v0.03 batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mechanism {
    /// Plain composition of learned edges only. Expected null transfer.
    Plain,
    /// CF-LM-015-style B-family abstraction, per-target relations.
    MemberAbstraction,
    /// B-family abstraction with pooled target generalization (theorized).
    Target,
}

#[derive(Clone, Debug, PartialEq)]
pub struct V3State {
    pub x: [f64; 9],
    pub theta: [f64; 9],
    pub psi: [[f64; 9]; 9],
    /// Formed (derived) active abstraction over the B-family.
    pub b_abstraction: bool,
    /// Abstraction->symbol relations (Member arm): index = target symbol.
    pub w_abs_b: [f64; 9],
    /// Pooled B-family -> C-family relation (Target arm).
    pub w_pool_c: f64,
}

impl V3State {
    pub fn initial() -> Self {
        Self {
            x: [0.0; 9],
            theta: [1.0; 9],
            psi: [[0.0; 9]; 9],
            b_abstraction: false,
            w_abs_b: [0.0; 9],
            w_pool_c: 0.0,
        }
    }

    /// Equalize only the fast/local roles; persistent abstraction and relations
    /// (Psi, W) survive exactly as in the v0.01/v0.02 bridge.
    pub fn equalized_from(state: &Self) -> Self {
        let mut next = state.clone();
        next.x = [0.0; 9];
        next.theta = [1.0; 9];
        next
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V3Input {
    pub activity: [f64; 9],
}

impl V3Input {
    pub fn symbol(symbol: S) -> Self {
        Self {
            activity: symbol.one_hot(),
        }
    }

    pub fn zero() -> Self {
        Self { activity: [0.0; 9] }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V3Experience {
    pub predecessor: Option<S>,
    pub current: S,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V3Episode {
    pub source: S,
    pub target: S,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct V3Curriculum {
    pub episodes: Vec<V3Episode>,
    pub epochs: usize,
}

impl V3Curriculum {
    /// The frozen LLM-authored v0.03 curriculum. `B3 -> C3` is never present.
    pub fn llm_authored() -> Self {
        Self {
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

    /// Direct-teaching positive control: same curriculum plus the withheld
    /// `B3 -> C3` from the start. Used only to prove the substrate CAN propagate
    /// a one-edge relation (the Plain arm can do this if taught directly).
    pub fn llm_authored_with_direct_b3_c3() -> Self {
        let mut base = Self::llm_authored();
        base.episodes.push(V3Episode {
            source: S::B3,
            target: S::C3,
        });
        base
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V3Model {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub mechanism: Mechanism,
}

impl Default for V3Model {
    fn default() -> Self {
        Self {
            beta: 0.50,
            input_gain: 0.50,
            relational_gain: 0.20,
            psi_decay: 0.02,
            psi_gain: 0.08,
            mechanism: Mechanism::Plain,
        }
    }
}

impl V3Model {
    pub fn with_mechanism(mechanism: Mechanism) -> Self {
        Self {
            mechanism,
            ..Self::default()
        }
    }

    /// One-step dynamics: the verified CV step followed by the optional
    /// abstraction-mediated additive continuation (CF-LM-015 semantics).
    fn step(&self, state: &V3State, input: &V3Input) -> V3State {
        let mut next = state.clone();
        let mut relational = [0.0; 9];
        for (target, value) in relational.iter_mut().enumerate() {
            for (source, source_activity) in state.x.iter().enumerate() {
                *value += state.psi[source][target] * source_activity;
            }
        }

        for (i, x) in next.x.iter_mut().enumerate() {
            *x = self.beta * state.x[i]
                + self.input_gain * input.activity[i]
                + self.relational_gain * relational[i];
        }

        if state.b_abstraction {
            let mean_b =
                (state.x[S::B1.index()] + state.x[S::B2.index()] + state.x[S::B3.index()]) / 3.0;
            match self.mechanism {
                Mechanism::Plain => (),
                Mechanism::MemberAbstraction => {
                    for (t, x) in next.x.iter_mut().enumerate() {
                        *x += self.relational_gain * state.w_abs_b[t] * mean_b;
                    }
                }
                Mechanism::Target => {
                    for &symbol in &S::C_FAMILY {
                        next.x[symbol.index()] += self.relational_gain * state.w_pool_c * mean_b;
                    }
                }
            }
        }

        next
    }

    fn adapt(&self, state: &V3State, experience: &V3Experience) -> V3State {
        let mut next = state.clone();
        for row in &mut next.psi {
            for value in row {
                *value *= 1.0 - self.psi_decay;
            }
        }

        if let Some(predecessor) = experience.predecessor {
            next.psi[predecessor.index()][experience.current.index()] += self.psi_gain;
        }

        // Abstraction-layer learning (CF-LM-015 uses the same decay/gain
        // constants). Built for the Member/Target arms; inert for Plain.
        for value in &mut next.w_abs_b {
            *value *= 1.0 - self.psi_decay;
        }
        next.w_pool_c *= 1.0 - self.psi_decay;
        if let Some(predecessor) = experience.predecessor {
            if predecessor.is_b() {
                next.w_abs_b[experience.current.index()] += self.psi_gain;
                if experience.current.is_c() {
                    next.w_pool_c += self.psi_gain;
                }
            }
        }

        next
    }

    /// CF-LM-015-style formation: derive an active abstraction over the B-family
    /// from the frozen consequence composition (the A -> B family). Idempotent.
    fn form_b_abstraction(&self, state: &V3State) -> V3State {
        let mut next = state.clone();
        next.b_abstraction = true;
        next
    }

    /// Train and return an immutable persistent state. The curriculum is exposed
    /// as isolated two-symbol teacher experiences; between episodes only the
    /// fast/local roles are equalized while persistent relations survive.
    pub fn train(&self, curriculum: &V3Curriculum, initial: &V3State) -> V3State {
        let mut state = initial.clone();
        for _ in 0..curriculum.epochs {
            for &episode in &curriculum.episodes {
                let mut episode_state = V3State::equalized_from(&state);
                let mut predecessor = None;
                for symbol in [episode.source, episode.target] {
                    episode_state = self.step(&episode_state, &V3Input::symbol(symbol));
                    episode_state = self.adapt(
                        &episode_state,
                        &V3Experience {
                            predecessor,
                            current: symbol,
                        },
                    );
                    predecessor = Some(symbol);
                }
                state = episode_state;
            }
        }

        if self.mechanism != Mechanism::Plain {
            state = self.form_b_abstraction(&state);
        }
        state
    }

    /// Teacher-off continuation: only a visible start symbol, then zero-input
    /// continuation steps. No target, correction, or adaptation event supplied.
    pub fn probe_teacher_off(
        &self,
        trained: &V3State,
        start: S,
        continuation_steps: usize,
    ) -> V3Probe {
        let mut state = V3State::equalized_from(trained);
        let mut trajectory = Vec::with_capacity(continuation_steps + 1);
        state = self.step(&state, &V3Input::symbol(start));
        trajectory.push(state.x);
        for _ in 0..continuation_steps {
            state = self.step(&state, &V3Input::zero());
            trajectory.push(state.x);
        }
        V3Probe { trajectory }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V3Probe {
    pub trajectory: Vec<[f64; 9]>,
}

impl V3Probe {
    pub fn activation(&self, step: usize, symbol: S) -> Option<f64> {
        self.trajectory.get(step).map(|frame| frame[symbol.index()])
    }
}

/// Convenience: a packaged v0.03 arm (model + trained state).
#[derive(Clone, Debug)]
pub struct V3Runner {
    pub model: V3Model,
    pub state: V3State,
}

pub fn run(mechanism: Mechanism, curriculum: &V3Curriculum) -> V3Runner {
    let model = V3Model::with_mechanism(mechanism);
    let state = model.train(curriculum, &V3State::initial());
    V3Runner { model, state }
}
