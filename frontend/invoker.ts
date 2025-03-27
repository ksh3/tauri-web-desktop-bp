import { isTauri } from "@tauri-apps/api/core";
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import type { Command } from "./types/_gen/Command";
import { GreetState } from "./types/_gen/GreetState";

export async function Invoker<TPayload extends InvokeArgs, TResponse>(
  command: Command,
  payload: TPayload
): Promise<GreetState> {
  console.log(isTauri());
  // if (window.__TAURI__) {
  if (isTauri()) {
    const text: string = await invoke(command.toLowerCase(), payload);
    return { message: text };
  } else {
    const query = new URLSearchParams(
      payload as Record<string, string>
    ).toString();

    const res = await fetch(
      `http://localhost:3000/${command.toLowerCase()}?${query}`,
      {
        method: "GET",
        headers: { "Content-Type": "application/json" },
      }
    );

    const text = await res.text();
    return { message: text };
  }
}
