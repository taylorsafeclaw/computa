//! Audio capture boundary for Computa.
//!
//! Real microphone capture (cpal) lands in a follow-up plan. For now this crate
//! defines the `AudioSource` trait (the seam consumers depend on) and a pure
//! sample-conversion helper that downstream transcription needs.

/// A source of mono f32 PCM audio frames at a known sample rate.
pub trait AudioSource {
    /// Sample rate in Hz of frames returned by [`AudioSource::next_frame`].
    fn sample_rate(&self) -> u32;
    /// Returns the next chunk of mono samples, or `None` when the source ends.
    fn next_frame(&mut self) -> Option<Vec<f32>>;
}

/// Placeholder source that yields no audio. Replaced by a cpal-backed source later.
pub struct SilentSource {
    pub sample_rate: u32,
}

impl AudioSource for SilentSource {
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn next_frame(&mut self) -> Option<Vec<f32>> {
        None
    }
}

/// Converts interleaved i16 PCM samples to normalized f32 in `[-1.0, 1.0]`.
pub fn i16_to_f32(samples: &[i16]) -> Vec<f32> {
    samples
        .iter()
        .map(|&s| s as f32 / i16::MAX as f32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i16_to_f32_normalizes_extremes_and_zero() {
        let out = i16_to_f32(&[i16::MAX, 0, i16::MIN + 1]);
        assert_eq!(out.len(), 3);
        assert!((out[0] - 1.0).abs() < 1e-6);
        assert!(out[1].abs() < 1e-6);
        assert!((out[2] + 1.0).abs() < 1e-3);
    }

    #[test]
    fn silent_source_reports_rate_and_no_frames() {
        let mut s = SilentSource {
            sample_rate: 16_000,
        };
        assert_eq!(s.sample_rate(), 16_000);
        assert!(s.next_frame().is_none());
    }
}
