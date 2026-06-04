//! The trigger → session command state machine. Pure; no OS deps.

use serde::{Deserialize, Serialize};

/// How the trigger key maps to recording start/stop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerMode {
    /// Hold to record; release to stop. Key-down starts, key-up stops.
    PushToTalk,
    /// Press to start, press again to stop. Key-up is ignored.
    Toggle,
}

/// A trigger key transition observed by the native layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEdge {
    Down,
    Up,
}

/// Whether a recording session is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Idle,
    Recording,
}

/// A command the native layer must act on (begin/end capture).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionCommand {
    Start,
    Stop,
}

/// Stateful mapping of trigger key edges to session commands.
pub struct Orchestrator {
    mode: TriggerMode,
    state: SessionState,
}

impl Orchestrator {
    /// Create an orchestrator in the idle state with the given trigger mode.
    pub fn new(mode: TriggerMode) -> Self {
        Self {
            mode,
            state: SessionState::Idle,
        }
    }

    /// Current recording state.
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Change the trigger mode (e.g. user flips the setting). Does not affect
    /// an in-progress session's state.
    pub fn set_mode(&mut self, mode: TriggerMode) {
        self.mode = mode;
    }

    /// Apply a trigger key edge, returning a command if state changed.
    pub fn on_key_edge(&mut self, edge: KeyEdge) -> Option<SessionCommand> {
        use KeyEdge::*;
        use SessionState::*;
        use TriggerMode::*;

        match (self.mode, edge, self.state) {
            // Push-to-talk: down starts, up stops.
            (PushToTalk, Down, Idle) => {
                self.state = Recording;
                Some(SessionCommand::Start)
            }
            (PushToTalk, Up, Recording) => {
                self.state = Idle;
                Some(SessionCommand::Stop)
            }
            // Toggle: each down flips; up ignored.
            (Toggle, Down, Idle) => {
                self.state = Recording;
                Some(SessionCommand::Start)
            }
            (Toggle, Down, Recording) => {
                self.state = Idle;
                Some(SessionCommand::Stop)
            }
            // Auto-repeat downs, toggle-mode ups, stray idle ups: no-op.
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ptt_down_starts_up_stops() {
        let mut o = Orchestrator::new(TriggerMode::PushToTalk);
        assert_eq!(o.state(), SessionState::Idle);
        assert_eq!(o.on_key_edge(KeyEdge::Down), Some(SessionCommand::Start));
        assert_eq!(o.state(), SessionState::Recording);
        assert_eq!(o.on_key_edge(KeyEdge::Up), Some(SessionCommand::Stop));
        assert_eq!(o.state(), SessionState::Idle);
    }

    #[test]
    fn ptt_ignores_autorepeat_down_while_recording() {
        let mut o = Orchestrator::new(TriggerMode::PushToTalk);
        o.on_key_edge(KeyEdge::Down);
        // Held-key auto-repeat fires extra Downs; they must not restart.
        assert_eq!(o.on_key_edge(KeyEdge::Down), None);
        assert_eq!(o.state(), SessionState::Recording);
    }

    #[test]
    fn ptt_ignores_stray_up_while_idle() {
        let mut o = Orchestrator::new(TriggerMode::PushToTalk);
        assert_eq!(o.on_key_edge(KeyEdge::Up), None);
        assert_eq!(o.state(), SessionState::Idle);
    }

    #[test]
    fn toggle_down_alternates_up_ignored() {
        let mut o = Orchestrator::new(TriggerMode::Toggle);
        assert_eq!(o.on_key_edge(KeyEdge::Down), Some(SessionCommand::Start));
        assert_eq!(o.on_key_edge(KeyEdge::Up), None); // up ignored in toggle
        assert_eq!(o.state(), SessionState::Recording);
        assert_eq!(o.on_key_edge(KeyEdge::Down), Some(SessionCommand::Stop));
        assert_eq!(o.state(), SessionState::Idle);
    }

    #[test]
    fn set_mode_changes_mapping() {
        let mut o = Orchestrator::new(TriggerMode::PushToTalk);
        o.set_mode(TriggerMode::Toggle);
        o.on_key_edge(KeyEdge::Down); // start
        assert_eq!(o.on_key_edge(KeyEdge::Up), None); // toggle ignores up
        assert_eq!(o.state(), SessionState::Recording);
    }
}
