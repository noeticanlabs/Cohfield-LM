use crate::profiles::language::{
    CohfieldLanguageModelV1, LanguageError, LanguageExperience, LanguageInput, LanguageState,
    SurfaceSymbol,
};
use crate::AdaptiveContinuationModel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TeacherEpisodeV001 {
    pub source: SurfaceSymbol,
    pub target: SurfaceSymbol,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TeacherCurriculumV001 {
    pub episodes: Vec<TeacherEpisodeV001>,
    pub epochs: usize,
}

impl TeacherCurriculumV001 {
    /// Frozen LLM-authored v0.01 curriculum.
    ///
    /// The teacher exposes only local pairs. It never exposes the full
    /// A->B->C->D chain as one training episode and it is absent during
    /// evaluation.
    pub fn llm_authored() -> Self {
        Self {
            episodes: vec![
                TeacherEpisodeV001 {
                    source: SurfaceSymbol::A,
                    target: SurfaceSymbol::B,
                },
                TeacherEpisodeV001 {
                    source: SurfaceSymbol::B,
                    target: SurfaceSymbol::C,
                },
                TeacherEpisodeV001 {
                    source: SurfaceSymbol::C,
                    target: SurfaceSymbol::D,
                },
            ],
            epochs: 64,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TeacherOffProbeV001 {
    pub trajectory: Vec<[f64; 4]>,
}

impl TeacherOffProbeV001 {
    pub fn activation(&self, step: usize, symbol: SurfaceSymbol) -> Option<f64> {
        self.trajectory.get(step).map(|x| x[symbol.index()])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CfLmTeacherBridgeV001;

impl CfLmTeacherBridgeV001 {
    fn expose_pair(
        model: &CohfieldLanguageModelV1,
        persistent: &LanguageState,
        episode: TeacherEpisodeV001,
    ) -> Result<LanguageState, LanguageError> {
        // Equalize only the fast/local roles between episodes. Persistent Psi survives.
        let mut state = LanguageState::equalized_from(persistent);
        let mut predecessor = None;

        for symbol in [episode.source, episode.target] {
            state = model.evolve(&state, &LanguageInput::symbol(symbol), 1.0)?;
            state = model.adapt(
                &state,
                &LanguageExperience {
                    predecessor,
                    current: symbol,
                },
            )?;
            predecessor = Some(symbol);
        }

        Ok(state)
    }

    pub fn train(
        &self,
        model: &CohfieldLanguageModelV1,
        initial: &LanguageState,
        curriculum: &TeacherCurriculumV001,
    ) -> Result<LanguageState, LanguageError> {
        if curriculum.episodes.is_empty() || curriculum.epochs == 0 {
            return Err(LanguageError::EmptyExposure);
        }

        let mut state = initial.clone();
        for _ in 0..curriculum.epochs {
            for &episode in &curriculum.episodes {
                state = Self::expose_pair(model, &state, episode)?;
            }
        }
        Ok(state)
    }

    /// Teacher-off continuation. No target, correction, operator, relation identity,
    /// or adaptation event is supplied during this phase.
    pub fn probe_teacher_off(
        &self,
        model: &CohfieldLanguageModelV1,
        trained: &LanguageState,
        start: SurfaceSymbol,
        continuation_steps: usize,
    ) -> Result<TeacherOffProbeV001, LanguageError> {
        let mut state = LanguageState::equalized_from(trained);
        let mut trajectory = Vec::with_capacity(continuation_steps + 1);

        state = model.evolve(&state, &LanguageInput::symbol(start), 1.0)?;
        trajectory.push(state.x);

        for _ in 0..continuation_steps {
            state = model.evolve(&state, &LanguageInput::zero(), 1.0)?;
            trajectory.push(state.x);
        }

        Ok(TeacherOffProbeV001 { trajectory })
    }
}
