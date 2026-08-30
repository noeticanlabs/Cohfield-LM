//! CF-LM Corpus Bridge v0.01.
//!
//! This module consumes the governed length-prefixed byte-visible curriculum
//! produced by Training-data `export_cflm_teacher_data_v001.py`. It deliberately
//! transfers no teacher weights, embeddings, hidden states, logits, or reasoning.
//! Only visible input/target bytes cross the boundary.

use std::error::Error;
use std::fmt;

pub const MAGIC: &[u8] = b"CFLM-TEACHER-DATA-V001\n";
pub const BYTE_COUNT: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusRecord {
    pub input: Vec<u8>,
    pub target: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CorpusPackError {
    BadMagic,
    TruncatedInputLength,
    TruncatedInputPayload,
    TruncatedTargetLength,
    TruncatedTargetPayload,
}

impl fmt::Display for CorpusPackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::BadMagic => "bad CF-LM teacher-data magic",
            Self::TruncatedInputLength => "truncated input length",
            Self::TruncatedInputPayload => "truncated input payload",
            Self::TruncatedTargetLength => "truncated target length",
            Self::TruncatedTargetPayload => "truncated target payload",
        };
        f.write_str(message)
    }
}

impl Error for CorpusPackError {}

fn read_u64_be(
    bytes: &[u8],
    pos: &mut usize,
    error: CorpusPackError,
) -> Result<usize, CorpusPackError> {
    if *pos + 8 > bytes.len() {
        return Err(error);
    }
    let mut raw = [0u8; 8];
    raw.copy_from_slice(&bytes[*pos..*pos + 8]);
    *pos += 8;
    Ok(u64::from_be_bytes(raw) as usize)
}

pub fn parse_pack(bytes: &[u8]) -> Result<Vec<CorpusRecord>, CorpusPackError> {
    if !bytes.starts_with(MAGIC) {
        return Err(CorpusPackError::BadMagic);
    }

    let mut pos = MAGIC.len();
    let mut records = Vec::new();
    while pos < bytes.len() {
        let input_len = read_u64_be(bytes, &mut pos, CorpusPackError::TruncatedInputLength)?;
        if pos + input_len > bytes.len() {
            return Err(CorpusPackError::TruncatedInputPayload);
        }
        let input = bytes[pos..pos + input_len].to_vec();
        pos += input_len;

        let target_len = read_u64_be(bytes, &mut pos, CorpusPackError::TruncatedTargetLength)?;
        if pos + target_len > bytes.len() {
            return Err(CorpusPackError::TruncatedTargetPayload);
        }
        let target = bytes[pos..pos + target_len].to_vec();
        pos += target_len;
        records.push(CorpusRecord { input, target });
    }
    Ok(records)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ByteLanguageState {
    pub x: Vec<f64>,
    /// Row-major source->target persistent relation matrix.
    pub psi: Vec<f64>,
}

impl ByteLanguageState {
    pub fn initial() -> Self {
        Self {
            x: vec![0.0; BYTE_COUNT],
            psi: vec![0.0; BYTE_COUNT * BYTE_COUNT],
        }
    }

    pub fn equalized_from(state: &Self) -> Self {
        Self {
            x: vec![0.0; BYTE_COUNT],
            psi: state.psi.clone(),
        }
    }

    pub fn relation(&self, source: u8, target: u8) -> f64 {
        self.psi[relation_index(source, target)]
    }
}

fn relation_index(source: u8, target: u8) -> usize {
    source as usize * BYTE_COUNT + target as usize
}

#[derive(Clone, Debug, PartialEq)]
pub struct ByteLanguageModel {
    pub beta: f64,
    pub input_gain: f64,
    pub relational_gain: f64,
    pub psi_decay: f64,
    pub psi_gain: f64,
    pub adaptation_enabled: bool,
}

impl Default for ByteLanguageModel {
    fn default() -> Self {
        Self {
            beta: 0.50,
            input_gain: 0.50,
            relational_gain: 0.20,
            psi_decay: 0.02,
            psi_gain: 0.08,
            adaptation_enabled: true,
        }
    }
}

impl ByteLanguageModel {
    pub fn without_adaptation() -> Self {
        Self {
            adaptation_enabled: false,
            ..Self::default()
        }
    }

    fn step(&self, state: &ByteLanguageState, input: Option<u8>) -> ByteLanguageState {
        let mut next = ByteLanguageState::equalized_from(state);
        let mut relational = vec![0.0; BYTE_COUNT];
        for source in 0..BYTE_COUNT {
            let source_activity = state.x[source];
            if source_activity == 0.0 {
                continue;
            }
            let row = source * BYTE_COUNT;
            for (target, value) in relational.iter_mut().enumerate() {
                *value += state.psi[row + target] * source_activity;
            }
        }

        for index in 0..BYTE_COUNT {
            next.x[index] = self.beta * state.x[index] + self.relational_gain * relational[index];
        }
        if let Some(byte) = input {
            next.x[byte as usize] += self.input_gain;
        }
        next
    }

    fn adapt_pair(&self, state: &ByteLanguageState, source: u8, target: u8) -> ByteLanguageState {
        if !self.adaptation_enabled {
            return state.clone();
        }
        let mut next = state.clone();
        for value in &mut next.psi {
            *value *= 1.0 - self.psi_decay;
        }
        next.psi[relation_index(source, target)] += self.psi_gain;
        next
    }

    /// Observe one governed record. Fast state and predecessor history are reset
    /// at each record boundary, so the end of one example never learns an
    /// artificial edge into the next example.
    fn observe_record(
        &self,
        state: &ByteLanguageState,
        record: &CorpusRecord,
    ) -> ByteLanguageState {
        let mut working = ByteLanguageState::equalized_from(state);
        let mut predecessor: Option<u8> = None;
        for byte in record.input.iter().chain(record.target.iter()).copied() {
            working = self.step(&working, Some(byte));
            if let Some(prev) = predecessor {
                working = self.adapt_pair(&working, prev, byte);
            }
            predecessor = Some(byte);
        }
        ByteLanguageState::equalized_from(&working)
    }

    /// Adapt only from the training records supplied by the caller.
    pub fn train(&self, records: &[CorpusRecord], epochs: usize) -> ByteLanguageState {
        let mut state = ByteLanguageState::initial();
        for _ in 0..epochs {
            for record in records {
                state = self.observe_record(&state, record);
            }
        }
        state
    }

    /// Present evaluation input with adaptation disabled. This constructs the
    /// transient state from visible prompt bytes but leaves persistent `psi`
    /// untouched.
    pub fn present_input(&self, trained: &ByteLanguageState, input: &[u8]) -> ByteLanguageState {
        let mut state = ByteLanguageState::equalized_from(trained);
        for byte in input.iter().copied() {
            state = self.step(&state, Some(byte));
        }
        state
    }

    /// Continue with zero external input after the teacher has been removed.
    pub fn teacher_off(&self, start: &ByteLanguageState, steps: usize) -> Vec<Vec<f64>> {
        let mut state = start.clone();
        let mut trajectory = Vec::with_capacity(steps + 1);
        trajectory.push(state.x.clone());
        for _ in 0..steps {
            state = self.step(&state, None);
            trajectory.push(state.x.clone());
        }
        trajectory
    }
}

pub fn activation(snapshot: &[f64], byte: u8) -> f64 {
    snapshot.get(byte as usize).copied().unwrap_or(0.0)
}
