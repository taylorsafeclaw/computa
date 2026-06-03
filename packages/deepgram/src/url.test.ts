import { describe, it, expect } from "vitest";
import { buildDeepgramUrl } from "./url";

describe("buildDeepgramUrl", () => {
  it("encodes required params with defaults", () => {
    const url = buildDeepgramUrl({ sampleRate: 16000 });
    expect(url).toContain("wss://api.deepgram.com/v1/listen?");
    expect(url).toContain("encoding=linear16");
    expect(url).toContain("sample_rate=16000");
    expect(url).toContain("channels=1");
    expect(url).toContain("model=nova-2");
    expect(url).toContain("interim_results=true");
  });

  it("includes language when provided", () => {
    const url = buildDeepgramUrl({ sampleRate: 48000, language: "en-US" });
    expect(url).toContain("language=en-US");
    expect(url).toContain("sample_rate=48000");
  });
});
