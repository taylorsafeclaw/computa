/** Options for a Deepgram live-streaming connection. */
export interface DeepgramStreamOptions {
  model?: string;
  language?: string;
  /** Audio sample rate in Hz (must match the PCM frames sent from Rust). */
  sampleRate: number;
  /** Number of audio channels. */
  channels?: number;
  /** Emit interim (non-final) results. */
  interimResults?: boolean;
}

const LISTEN_ENDPOINT = "wss://api.deepgram.com/v1/listen";

/** Builds the Deepgram live-streaming websocket URL from options (pure function). */
export function buildDeepgramUrl(opts: DeepgramStreamOptions): string {
  const params = new URLSearchParams({
    encoding: "linear16",
    sample_rate: String(opts.sampleRate),
    channels: String(opts.channels ?? 1),
    model: opts.model ?? "nova-2",
    interim_results: String(opts.interimResults ?? true),
  });
  if (opts.language) params.set("language", opts.language);
  return `${LISTEN_ENDPOINT}?${params.toString()}`;
}
