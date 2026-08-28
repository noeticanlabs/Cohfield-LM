//! CF-LM Teacher Bridge v0.05 — experience-derived role discovery.
//!
//! v0.04 showed selective held-out transfer when the source and target role
//! families were supplied by the experiment. v0.05 removes those family arrays
//! from the runtime mechanism. The teacher now supplies only visible pair
//! experiences. Two neutral relation anchors (`R1`, `R2`) are part of that
//! experience, but the runtime is not told what either anchor means or which
//! symbols belong to a source or target family.
//!
//! The mechanism is deliberately generic:
//!
//! 1. discover candidate role sets as repeated outgoing neighborhoods in `Psi`;
//! 2. infer the directed relation schema between two discovered role sets from
//!    visible cross-role edges;
//! 3. derive member correspondence from cosine overlap of incoming `Psi`
//!    signatures;
//! 4. apply the learned schema to a held-out member pair during teacher-off
//!    continuation.
//!
//! The withheld edge `B3 -> C3` is never taught and remains exactly zero.
//! `C3` is visible through independent structural experience, so this is still
//! relation transfer to a known target identity, not invention of a new symbol.

const N: usize = 11;
const EDGE_EPS: f64 = 1.0e-12;
const MIN_ROLE_SIZE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum S5 {
    A1,
    A2,
    A3,
    B1,
    B2,
    B3,
    C1,
    C2,
    C3,
    R1,
    R2,
}

impl S5 {
    pub const ALL: [S5; N] = [
        S5::A1,
        S5::A2,
        S5::A3,
        S5::B1,
        S5::B2,
        S5::B3,
        S5::C1,
        S5::C2,
        S5::C3,
        S5::R1,
        S5::R2,
    ];

    pub fn index(self) -> usize {
        match self {
            S5::A1 => 0,
            S5::A2 => 1,
            S5::A3 => 2,
            S5::B1 => 3,
            S5::B2 => 4,
            S5::B3 => 5,
            S5::C1 => 6,
            S5::C2 => 7,
            S5::C3 => 8,
            S5::R1 => 9,
            S5::R2 => 10,
        }
    }

    pub fn one_hot(self) -> [f64; N] {
        let mut out = [0.0; N];
        out[self.index()] = 1.0;
        out
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V5Mechanism {
    Plain,
    DiscoveredBinding,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V5Episode {
    pub source: S5,
    pub target: S5,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct V5Curriculum {
    pub episodes: Vec<V5Episode>,
    pub epochs: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct V5RelabeledCurriculum {
    pub curriculum: V5Curriculum,
    pub held_out_source: S5,
    pub held_out_target: S5,
}

impl V5Curriculum {
    /// Frozen v0.05 curriculum. `R1` and `R2` are neutral surface anchors; their
    /// role meaning is not exposed to the discovery algorithm.
    pub fn llm_authored() -> Self {
        Self {
            episodes: vec![
                V5Episode {
                    source: S5::A1,
                    target: S5::B1,
                },
                V5Episode {
                    source: S5::A2,
                    target: S5::B2,
                },
                V5Episode {
                    source: S5::A3,
                    target: S5::B3,
                },
                V5Episode {
                    source: S5::A1,
                    target: S5::C1,
                },
                V5Episode {
                    source: S5::A2,
                    target: S5::C2,
                },
                V5Episode {
                    source: S5::A3,
                    target: S5::C3,
                },
                V5Episode {
                    source: S5::R1,
                    target: S5::B1,
                },
                V5Episode {
                    source: S5::R1,
                    target: S5::B2,
                },
                V5Episode {
                    source: S5::R1,
                    target: S5::B3,
                },
                V5Episode {
                    source: S5::R2,
                    target: S5::C1,
                },
                V5Episode {
                    source: S5::R2,
                    target: S5::C2,
                },
                V5Episode {
                    source: S5::R2,
                    target: S5::C3,
                },
                V5Episode {
                    source: S5::B1,
                    target: S5::C1,
                },
                V5Episode {
                    source: S5::B2,
                    target: S5::C2,
                },
            ],
            epochs: 64,
        }
    }

    pub fn without_role_anchors() -> Self {
        let mut base = Self::llm_authored();
        base.episodes
            .retain(|episode| !matches!(episode.source, S5::R1 | S5::R2));
        base
    }

    pub fn without_schema_examples() -> Self {
        let mut base = Self::llm_authored();
        base.episodes.retain(|episode| {
            !matches!(
                (episode.source, episode.target),
                (S5::B1, S5::C1) | (S5::B2, S5::C2)
            )
        });
        base
    }

    pub fn without_third_target_anchor() -> Self {
        let mut base = Self::llm_authored();
        base.episodes
            .retain(|episode| (episode.source, episode.target) != (S5::A3, S5::C3));
        base
    }

    pub fn with_swapped_third_correspondence() -> Self {
        let mut base = Self::llm_authored();
        for episode in &mut base.episodes {
            if (episode.source, episode.target) == (S5::A3, S5::C3) {
                episode.target = S5::C2;
            }
        }
        base
    }

    /// Apply an arbitrary bijective surface relabeling. This control verifies
    /// that the discovery mechanism depends on relation structure rather than
    /// the enum names or fixed symbol positions.
    pub fn relabeled() -> V5RelabeledCurriculum {
        let permutation = [
            S5::C2,
            S5::R1,
            S5::B3,
            S5::A1,
            S5::C3,
            S5::R2,
            S5::B1,
            S5::A3,
            S5::C1,
            S5::B2,
            S5::A2,
        ];
        let map = |symbol: S5| permutation[symbol.index()];
        let base = Self::llm_authored();
        V5RelabeledCurriculum {
            curriculum: V5Curriculum {
                episodes: base
                    .episodes
                    .iter()
                    .map(|episode| V5Episode {
                        source: map(episode.source),
                        target: map(episode.target),
                    })
                    .collect(),
                epochs: base.epochs,
            },
            held_out_source: map(S5::B3),
            held_out_target: map(S5::C3),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V5Input {
    pub activity: [f64; N],
}

impl V5Input {
    pub fn symbol(symbol: S5) -> Self {
        Self {
            activity: symbol.one_hot(),
        }
    }

    pub fn zero() -> Self {
        Self { activity: [0.0; N] }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V5Experience {
    pub predecessor: Option<S5>,
    pub current: S5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V5Role {
    pub anchor: S5,
    pub members: [bool; N],
}

impl V5Role {
    pub fn contains(self, symbol: S5) -> bool {
        self.members[symbol.index()]
    }

    pub fn member_count(self) -> usize {
        self.members.iter().filter(|&&member| member).count()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V5Structure {
    pub discovered_roles: Vec<V5Role>,
    pub source_role_index: Option<usize>,
    pub target_role_index: Option<usize>,
    pub slot_affinity: [[f64; N]; N],
    pub binding_gain: f64,
}

impl V5Structure {
    fn empty() -> Self {
        Self {
            discovered_roles: Vec::new(),
            source_role_index: None,
            target_role_index: None,
            slot_affinity: [[0.0; N]; N],
            binding_gain: 0.0,
        }
    }

    pub fn source_role(&self) -> Option<V5Role> {
        self.source_role_index
            .and_then(|index| self.discovered_roles.get(index).copied())
    }

    pub fn target_role(&self) -> Option<V5Role> {
        self.target_role_index
            .and_then(|index| self.discovered_roles.get(index).copied())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V5State {
    pub x: [f64; N],
    pub theta: [f64; N],
    pub psi: [[f64; N]; N],
    pub structure: V5Structure,
}

impl V5State {
    pub fn initial() -> Self {
        Self {
            x: [0.0; N],
            theta: [1.0; N],
            psi: [[0.0; N]; N],
            structure: V5Structure::empty(),
        }
    }

    pub fn equalized_from(state: &Self) -> Self {
        let mut next = state.clone();
        next.x = [0.0; N];
        next.theta = [1.0; N];
        next
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V5Model {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub mechanism: V5Mechanism,
}

impl V5Model {
    pub fn with_mechanism(mechanism: V5Mechanism) -> Self {
        Self {
            beta: 0.50,
            input_gain: 0.50,
            relational_gain: 0.20,
            psi_decay: 0.02,
            psi_gain: 0.08,
            mechanism,
        }
    }

    fn step(&self, state: &V5State, input: &V5Input) -> V5State {
        let mut next = state.clone();
        let mut relational = [0.0; N];
        for (target, value) in relational.iter_mut().enumerate() {
            for (source, source_activity) in state.x.iter().enumerate() {
                *value += state.psi[source][target] * source_activity;
            }
        }

        for (index, x) in next.x.iter_mut().enumerate() {
            *x = self.beta * state.x[index]
                + self.input_gain * input.activity[index]
                + self.relational_gain * relational[index];
        }

        if self.mechanism == V5Mechanism::DiscoveredBinding {
            if let (Some(source_role), Some(target_role)) =
                (state.structure.source_role(), state.structure.target_role())
            {
                for source in S5::ALL {
                    if !source_role.contains(source) {
                        continue;
                    }
                    let source_activity = state.x[source.index()];
                    for target in S5::ALL {
                        if !target_role.contains(target) {
                            continue;
                        }
                        let affinity =
                            state.structure.slot_affinity[source.index()][target.index()];
                        next.x[target.index()] += self.relational_gain
                            * state.structure.binding_gain
                            * affinity
                            * source_activity;
                    }
                }
            }
        }
        next
    }

    fn adapt(&self, state: &V5State, experience: &V5Experience) -> V5State {
        let mut next = state.clone();
        for row in &mut next.psi {
            for value in row {
                *value *= 1.0 - self.psi_decay;
            }
        }
        if let Some(predecessor) = experience.predecessor {
            next.psi[predecessor.index()][experience.current.index()] += self.psi_gain;
        }
        next
    }

    fn discover_candidate_roles(&self, psi: &[[f64; N]; N]) -> Vec<V5Role> {
        let mut roles = Vec::new();
        for anchor in S5::ALL {
            let mut members = [false; N];
            for target in S5::ALL {
                members[target.index()] = psi[anchor.index()][target.index()] > EDGE_EPS;
            }
            let role = V5Role { anchor, members };
            if role.member_count() >= MIN_ROLE_SIZE {
                roles.push(role);
            }
        }
        roles
    }

    fn incoming_signature_cosine(&self, psi: &[[f64; N]; N], left: S5, right: S5) -> f64 {
        let mut dot = 0.0;
        let mut left_norm = 0.0;
        let mut right_norm = 0.0;
        for predecessor in S5::ALL {
            let l = psi[predecessor.index()][left.index()];
            let r = psi[predecessor.index()][right.index()];
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

    fn cross_role_evidence(
        &self,
        psi: &[[f64; N]; N],
        source_role: V5Role,
        target_role: V5Role,
    ) -> (usize, f64, f64) {
        let mut count = 0usize;
        let mut total = 0.0;
        for source in S5::ALL {
            if !source_role.contains(source) {
                continue;
            }
            for target in S5::ALL {
                if !target_role.contains(target) {
                    continue;
                }
                let weight = psi[source.index()][target.index()];
                if weight > EDGE_EPS {
                    count += 1;
                    total += weight;
                }
            }
        }
        let mean = if count == 0 {
            0.0
        } else {
            total / count as f64
        };
        (count, total, mean)
    }

    fn discover_structure(&self, psi: &[[f64; N]; N]) -> V5Structure {
        let discovered_roles = self.discover_candidate_roles(psi);
        let mut best_source = None;
        let mut best_target = None;
        let mut best_count = 0usize;
        let mut best_total = 0.0;
        let mut binding_gain = 0.0;

        for source_index in 0..discovered_roles.len() {
            for target_index in 0..discovered_roles.len() {
                if source_index == target_index {
                    continue;
                }
                let (count, total, mean) = self.cross_role_evidence(
                    psi,
                    discovered_roles[source_index],
                    discovered_roles[target_index],
                );
                if count > best_count || (count == best_count && total > best_total) {
                    best_source = Some(source_index);
                    best_target = Some(target_index);
                    best_count = count;
                    best_total = total;
                    binding_gain = mean;
                }
            }
        }

        if best_count == 0 {
            return V5Structure {
                discovered_roles,
                source_role_index: None,
                target_role_index: None,
                slot_affinity: [[0.0; N]; N],
                binding_gain: 0.0,
            };
        }

        let source_role = discovered_roles[best_source.unwrap()];
        let target_role = discovered_roles[best_target.unwrap()];
        let mut slot_affinity = [[0.0; N]; N];
        for source in S5::ALL {
            if !source_role.contains(source) {
                continue;
            }
            for target in S5::ALL {
                if target_role.contains(target) {
                    slot_affinity[source.index()][target.index()] =
                        self.incoming_signature_cosine(psi, source, target);
                }
            }
        }

        V5Structure {
            discovered_roles,
            source_role_index: best_source,
            target_role_index: best_target,
            slot_affinity,
            binding_gain,
        }
    }

    pub fn train(&self, curriculum: &V5Curriculum, initial: &V5State) -> V5State {
        let mut state = initial.clone();
        for _ in 0..curriculum.epochs {
            for &episode in &curriculum.episodes {
                let mut episode_state = V5State::equalized_from(&state);
                let mut predecessor = None;
                for symbol in [episode.source, episode.target] {
                    episode_state = self.step(&episode_state, &V5Input::symbol(symbol));
                    episode_state = self.adapt(
                        &episode_state,
                        &V5Experience {
                            predecessor,
                            current: symbol,
                        },
                    );
                    predecessor = Some(symbol);
                }
                state = episode_state;
            }
        }
        state.structure = self.discover_structure(&state.psi);
        state
    }

    pub fn probe_teacher_off(
        &self,
        trained: &V5State,
        start: S5,
        continuation_steps: usize,
    ) -> V5Probe {
        let mut state = V5State::equalized_from(trained);
        let mut trajectory = Vec::with_capacity(continuation_steps + 1);
        state = self.step(&state, &V5Input::symbol(start));
        trajectory.push(state.x);
        for _ in 0..continuation_steps {
            state = self.step(&state, &V5Input::zero());
            trajectory.push(state.x);
        }
        V5Probe { trajectory }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V5Probe {
    pub trajectory: Vec<[f64; N]>,
}

impl V5Probe {
    pub fn activation(&self, step: usize, symbol: S5) -> Option<f64> {
        self.trajectory.get(step).map(|frame| frame[symbol.index()])
    }
}

#[derive(Clone, Debug)]
pub struct V5Runner {
    pub model: V5Model,
    pub state: V5State,
}

pub fn run(mechanism: V5Mechanism, curriculum: &V5Curriculum) -> V5Runner {
    let model = V5Model::with_mechanism(mechanism);
    let state = model.train(curriculum, &V5State::initial());
    V5Runner { model, state }
}
