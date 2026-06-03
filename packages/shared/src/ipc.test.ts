import { describe, it, expect, vi } from "vitest";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(() => Promise.resolve()),
}));

import { invoke } from "@tauri-apps/api/core";
import { injectText, Commands } from "./ipc";

describe("injectText", () => {
  it("invokes the inject_text command with a text arg", async () => {
    await injectText("hello");
    expect(invoke).toHaveBeenCalledWith(Commands.injectText, { text: "hello" });
  });
});
