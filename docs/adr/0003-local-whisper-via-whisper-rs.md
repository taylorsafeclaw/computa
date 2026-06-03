# 3. Local transcription via whisper-rs (Metal)

Date: 2026-06-03
Status: Accepted

## Context

Local, offline transcription is a core feature. Candidates: whisper-rs
(whisper.cpp bindings) vs candle (pure Rust).

## Decision

Use whisper-rs with Metal acceleration on macOS, behind the `Transcriber` trait
in `crates/transcribe`. The scaffold ships only the trait + a `NullTranscriber`;
the whisper-rs dependency and model loading land in a follow-up plan.

## Consequences

- Heavy native build (whisper.cpp) is deferred out of the scaffold to keep CI fast.
- The trait seam lets the UI and pipeline be built/tested before real inference.
