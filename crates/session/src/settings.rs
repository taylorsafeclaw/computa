//! User-facing settings and their JSON persistence. Pure; the native layer
//! supplies the on-disk path (the OS app-config dir).

use crate::orchestrator::TriggerMode;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Which transcription provider is active. Whisper is the zero-config default;
/// Deepgram is opt-in and gated on a keychain'd API key (handled in Plan 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    Whisper,
    Deepgram,
}

/// Persisted user settings. The trigger key itself is fixed for M1 (right-Option
/// / Fn) and bound later in the settings UI milestone (M3), so it is not here yet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub mode: TriggerMode,
    pub provider: Provider,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mode: TriggerMode::PushToTalk,
            provider: Provider::Whisper,
        }
    }
}

impl Settings {
    /// Load settings from `path`, falling back to defaults if the file is missing
    /// or unparseable (never panics — a corrupt file must not brick the app).
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_default()
    }

    /// Persist settings to `path` as pretty JSON, creating parent dirs as needed.
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self).expect("Settings is serializable");
        std::fs::write(path, json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn default_is_ptt_and_whisper() {
        let s = Settings::default();
        assert_eq!(s.mode, TriggerMode::PushToTalk);
        assert_eq!(s.provider, Provider::Whisper);
    }

    #[test]
    fn load_returns_default_when_file_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("settings.json");
        assert_eq!(Settings::load(&path), Settings::default());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested/settings.json"); // parent created on save
        let s = Settings {
            mode: TriggerMode::Toggle,
            provider: Provider::Deepgram,
        };
        s.save(&path).unwrap();
        assert_eq!(Settings::load(&path), s);
    }

    #[test]
    fn load_returns_default_on_corrupt_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(&path, "{ not valid json").unwrap();
        assert_eq!(Settings::load(&path), Settings::default());
    }
}
