export * from "./url";

/** Configuration for the (not-yet-implemented) streaming client. */
export interface DeepgramClientConfig {
  apiKey: string;
  sampleRate: number;
}

/**
 * Placeholder for the live streaming client. The real implementation (opening
 * the websocket via buildDeepgramUrl and piping PCM frames from Rust) lands in a
 * follow-up plan. Kept as a typed seam so the UI can be wired against it.
 */
export function createDeepgramClient(config: DeepgramClientConfig) {
  return {
    config,
    connect(): never {
      throw new Error("Deepgram streaming not implemented yet");
    },
  };
}
