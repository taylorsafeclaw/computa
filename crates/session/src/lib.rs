//! Session orchestration for Computa (OS-independent).
//!
//! Turns trigger key edges into start/stop session commands for both
//! push-to-talk and toggle modes, and persists user settings. The native layer
//! (rdev/cpal/enigo) drives this brain; it imports no OS crates so it is fully
//! unit-testable.

pub mod orchestrator;
pub mod settings;

pub use orchestrator::{KeyEdge, Orchestrator, SessionCommand, SessionState, TriggerMode};
pub use settings::{Provider, Settings};
