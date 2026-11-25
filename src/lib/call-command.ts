import { invoke } from "@tauri-apps/api/core"

export type CommandError = {
  code: string
  message: string
}

const fallbackError: CommandError = {
  code: "internal",
  message: "Something went wrong. Please try again.",
}

function isCommandError(value: unknown): value is CommandError {
  return (
    typeof value === "object" &&
    value !== null &&
    "code" in value &&
    typeof value.code === "string" &&
    "message" in value &&
    typeof value.message === "string"
  )
}

export async function callCommand<Result>(
  name: string,
  args?: Record<string, unknown>
): Promise<Result> {
  try {
    return await invoke<Result>(name, args)
  } catch (error) {
    throw isCommandError(error) ? error : fallbackError
  }
}
