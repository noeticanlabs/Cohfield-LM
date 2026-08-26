use crate::{AdaptiveContinuationModel, StateRoles};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SurfaceSymbol {
    A,
    B,
    C,
    D,
}

impl SurfaceSymbol {
    pub const ALL: [Self; 4] = [Self::A, Self::B, Self::C, Self::D];

    pub fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }

    pub fn one_hot(self) -> [f64; 4] {
        let mut out = [0.0; 4];
        out[self.index()] = 1.0;
        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageState {
    pub x: [f64; 4],
    pub theta: [f64; 4],
    pub psi: [[f64; 4]; 4],
}

impl LanguageState {
    pub fn initial() -> Self {
        Self {
            x: [0.0; 4],
            theta: [1.0; 4],
            psi: [[0.0; 4]; 4],
        }
    }

    pub fn equalized_from(state: &Self) -> Self {
        let mut next = state.clone();
        next.x = [0.0; 4];
        next.theta = [1.0; 4];
        next
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageInput {
    pub activity: [f64; 4],
}

impl LanguageInput {
    pub fn symbol(symbol: SurfaceSymbol) -> Self {
        Self {
            activity: symbol.one_hot(),
        }
    }

    pub fn zero() -> Self {
        Self { activity: [0.0; 4] }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LanguageExperience {
    pub predecessor: Option<SurfaceSymbol>,
    pub current: SurfaceSymbol,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageObservationProfile {
    pub probes: Vec<[SurfaceSymbol; 2]>,
    pub continuation_steps: usize,
}

impl LanguageObservationProfile {
    pub fn cf_lm_001() -> Self {
        Self {
            probes: vec![
                [SurfaceSymbol::A, SurfaceSymbol::C],
                [SurfaceSymbol::B, SurfaceSymbol::D],
                [SurfaceSymbol::C, SurfaceSymbol::A],
                [SurfaceSymbol::D, SurfaceSymbol::B],
            ],
            continuation_steps: 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageResponse {
    pub vectors: Vec<[f64; 4]>,
}

impl LanguageResponse {
    pub fn flattened(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.vectors.len() * 4);
        for vector in &self.vectors {
            out.extend_from_slice(vector);
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LanguageError {
    InvalidState,
    InvalidParameter,
    InvalidHorizon,
    EmptyProbeFamily,
    EmptyExposure,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohfieldLanguageModelV1 {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
}

impl Default for CohfieldLanguageModelV1 {
    fn default() -> Self {
        Self {
            beta: 0.50,
            input_gain: 0.50,
            relational_gain: 0.20,
            psi_decay: 0.02,
            psi_gain: 0.08,
        }
    }
}

impl CohfieldLanguageModelV1 {
    pub fn without_adaptation() -> Self {
        Self {
            psi_gain: 0.0,
            ..Self::default()
        }
    }

    fn valid_state(&self, state: &LanguageState) -> bool {
        state.x.iter().all(|v| v.is_finite())
            && state.theta.iter().all(|v| v.is_finite())
            && state
                .psi
                .iter()
                .flat_map(|row| row.iter())
                .all(|v| v.is_finite())
            && state.theta == [1.0; 4]
            && self.valid_parameters()
    }

    fn valid_parameters(&self) -> bool {
        self.beta.is_finite()
            && self.input_gain.is_finite()
            && self.relational_gain.is_finite()
            && self.psi_decay.is_finite()
            && self.psi_gain.is_finite()
            && (0.0..=1.0).contains(&self.psi_decay)
            && self.psi_gain >= 0.0
    }

    fn step(&self, state: &LanguageState, input: &LanguageInput) -> Result<LanguageState, LanguageError> {
        if !self.valid_state(state) || input.activity.iter().any(|v| !v.is_finite()) {
            return Err(LanguageError::InvalidState);
        }

        let mut next = state.clone();
        let mut relational = [0.0; 4];
        for (target, value) in relational.iter_mut().enumerate() {
            for source in 0..4 {
                *value += state.psi[source][target] * state.x[source];
            }
        }

        for (i, x) in next.x.iter_mut().enumerate() {
            *x = self.beta * state.x[i]
                + self.input_gain * input.activity[i]
                + self.relational_gain * relational[i];
        }
        Ok(next)
    }

    pub fn expose(
        &self,
        initial: &LanguageState,
        pattern: &[SurfaceSymbol],
        repeats: usize,
    ) -> Result<LanguageState, LanguageError> {
        if pattern.is_empty() || repeats == 0 {
            return Err(LanguageError::EmptyExposure);
        }
        if !self.valid_state(initial) {
            return Err(LanguageError::InvalidState);
        }

        let mut state = initial.clone();
        let mut predecessor = None;
        for _ in 0..repeats {
            for &symbol in pattern {
                state = self.step(&state, &LanguageInput::symbol(symbol))?;
                state = self.adapt(
                    &state,
                    &LanguageExperience {
                        predecessor,
                        current: symbol,
                    },
                )?;
                predecessor = Some(symbol);
            }
        }
        Ok(state)
    }

    pub fn response_distance(left: &LanguageResponse, right: &LanguageResponse) -> Option<f64> {
        let a = left.flattened();
        let b = right.flattened();
        if a.is_empty() || a.len() != b.len() {
            return None;
        }
        Some(
            a.iter()
                .zip(b.iter())
                .map(|(x, y)| (x - y) * (x - y))
                .sum::<f64>()
                .sqrt(),
        )
    }

    pub fn psi_frobenius_distance(left: &LanguageState, right: &LanguageState) -> f64 {
        left.psi
            .iter()
            .zip(right.psi.iter())
            .flat_map(|(a, b)| a.iter().zip(b.iter()))
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }
}

impl AdaptiveContinuationModel for CohfieldLanguageModelV1 {
    type State = LanguageState;
    type Fast = [f64; 4];
    type LocalCondition = [f64; 4];
    type RelationalConfiguration = [[f64; 4]; 4];
    type Input = LanguageInput;
    type Experience = LanguageExperience;
    type ObservationProfile = LanguageObservationProfile;
    type Response = LanguageResponse;
    type Error = LanguageError;

    fn roles(
        &self,
        state: &Self::State,
    ) -> StateRoles<Self::Fast, Self::LocalCondition, Self::RelationalConfiguration> {
        StateRoles {
            fast: state.x,
            local_condition: state.theta,
            relational_configuration: state.psi,
        }
    }

    fn evolve(
        &self,
        state: &Self::State,
        input: &Self::Input,
        horizon: f64,
    ) -> Result<Self::State, Self::Error> {
        if !horizon.is_finite() || horizon < 0.0 || horizon.fract() != 0.0 {
            return Err(LanguageError::InvalidHorizon);
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
            return Err(LanguageError::InvalidState);
        }
        if !self.valid_parameters() {
            return Err(LanguageError::InvalidParameter);
        }

        let mut next = state.clone();
        for row in &mut next.psi {
            for value in row {
                *value *= 1.0 - self.psi_decay;
            }
        }

        if let Some(predecessor) = experience.predecessor {
            next.psi[predecessor.index()][experience.current.index()] += self.psi_gain;
        }

        Ok(next)
    }

    fn observe(
        &self,
        state: &Self::State,
        profile: &Self::ObservationProfile,
    ) -> Result<Self::Response, Self::Error> {
        if !self.valid_state(state) {
            return Err(LanguageError::InvalidState);
        }
        if profile.probes.is_empty() {
            return Err(LanguageError::EmptyProbeFamily);
        }

        let mut vectors = Vec::with_capacity(profile.probes.len() * (2 + profile.continuation_steps));
        for probe in &profile.probes {
            let mut local = LanguageState::equalized_from(state);
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
