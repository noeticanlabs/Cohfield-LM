//! CF-LM Teacher Bridge v0.02 — withheld-combination prototype.
//!
//! Reuses the verified v0.01 bridge mechanics (per-pair exposure with
//! persistent `Psi` survival, teacher-off zero-input continuation) and adds a
//! branching LLM-authored curriculum in which entire two-hop combinations are
//! withheld from training and must be resolved by CF-LM alone after the
//! teacher is removed.

use crate::profiles::language::{
    CohfieldLanguageModelV1, LanguageError, LanguageState, SurfaceSymbol,
};
use crate::teacher_bridge::{CfLmTeacherBridgeV001, TeacherEpisodeV001, TeacherOffProbeV001};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TeacherCurriculumV002 {
    pub episodes: Vec<TeacherEpisodeV001>,
    pub epochs: usize,
}

impl TeacherCurriculumV002 {
    /// Frozen LLM-authored v0.02 branching curriculum.
    ///
    /// The teacher exposes only the local pairs A->B, B->C, and B->D. The
    /// two-hop readings A->C and A->D are never exposed as episodes, the
    /// v0.01 edge C->D is deliberately absent, and no episode ever reaches A
    /// as a target, so C->A and D->A are structurally underivable.
    pub fn llm_authored_branching() -> Self {
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
                    source: SurfaceSymbol::B,
                    target: SurfaceSymbol::D,
                },
            ],
            epochs: 64,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CfLmTeacherBridgeV002;

impl CfLmTeacherBridgeV002 {
    /// Train through visible teacher experience only. Delegates to the
    /// verified v0.01 bridge mechanics; no new adaptation law is introduced.
    pub fn train(
        &self,
        model: &CohfieldLanguageModelV1,
        initial: &LanguageState,
        curriculum: &TeacherCurriculumV002,
    ) -> Result<LanguageState, LanguageError> {
        CfLmTeacherBridgeV001.train(
            model,
            initial,
            &crate::teacher_bridge::TeacherCurriculumV001 {
                episodes: curriculum.episodes.clone(),
                epochs: curriculum.epochs,
            },
        )
    }

    /// Teacher-off continuation. No target, correction, relation identity, or
    /// adaptation event is supplied during this phase.
    pub fn probe_teacher_off(
        &self,
        model: &CohfieldLanguageModelV1,
        trained: &LanguageState,
        start: SurfaceSymbol,
        continuation_steps: usize,
    ) -> Result<TeacherOffProbeV001, LanguageError> {
        CfLmTeacherBridgeV001.probe_teacher_off(model, trained, start, continuation_steps)
    }
}
