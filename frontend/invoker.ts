import { isTauri } from "@tauri-apps/api/core";
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import type { Command } from "./types/_gen/Command";
import { GreetState } from "./types/_gen/GreetState";

export async function Invoker<TPayload extends InvokeArgs, TResponse>(
  command: Command,
  payload: TPayload
): Promise<GreetState> {
  if (isTauri()) {
    const text: string = await invoke(command.toLowerCase(), payload);
    return { message: text };
  } else {
    console.log(`${command.toLowerCase()}?${JSON.stringify(payload)}`);
    const query = new URLSearchParams(
      payload as Record<string, string>
    ).toString();

    const res = await fetch(
      `http://localhost:8080/${command.toLowerCase()}?${query}`,
      {
        method: "GET",
        headers: { "Content-Type": "application/json" },
      }
    );

    const text = await res.text();
    return { message: text };
  }
}
