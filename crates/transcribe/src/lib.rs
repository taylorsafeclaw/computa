//! Transcription boundary for Computa.
//!
//! Real local inference (whisper-rs, Metal) lands in a follow-up plan. For now
//! this crate defines the `Transcriber` trait so the rest of the app can be wired
//! and tested against a mock, plus a `NullTranscriber` placeholder.

/// Error returned by a [`Transcriber`].
#[derive(Debug, PartialEq, Eq)]
pub enum TranscribeError {
    /// The model or backend was not available.
    Unavailable,
}

/// Turns mono f32 PCM samples into text.
pub trait Transcriber {
    /// Transcribe the given samples (assumed 16 kHz mono) into text.
    fn transcribe(&self, samples: &[f32]) -> Result<String, TranscribeError>;
}

/// Placeholder transcriber that always returns an empty transcript.
pub struct NullTranscriber;

impl Transcriber for NullTranscriber {
    fn transcribe(&self, _samples: &[f32]) -> Result<String, TranscribeError> {
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_transcriber_returns_empty_string() {
        let t = NullTranscriber;
        assert_eq!(t.transcribe(&[0.0, 0.1, -0.1]), Ok(String::new()));
    }
}
