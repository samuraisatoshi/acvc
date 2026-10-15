import { spawn } from "child_process";

export class AcvcError extends Error {
  readonly context: unknown;

  constructor(message: string, context: unknown) {
    super(message);
    this.name = "AcvcError";
    this.context = context;
  }
}

export function runAcvc(binary: string, args: string[], cwd: string): Promise<unknown> {
  return new Promise((resolve, reject) => {
    const child = spawn(binary, args, { cwd });
    const out: Buffer[] = [];
    const err: Buffer[] = [];
    child.stdout.on("data", (chunk: Buffer) => out.push(chunk));
    child.stderr.on("data", (chunk: Buffer) => err.push(chunk));
    child.on("error", reject);
    child.on("close", (code) => {
      const stdout = Buffer.concat(out).toString("utf8");
      const stderr = Buffer.concat(err).toString("utf8");
      if (code !== 0) {
        reject(failure(stderr, code));
        return;
      }
      try {
        resolve(JSON.parse(stdout));
      } catch (cause) {
        reject(new AcvcError("acvc stdout is not json", { stdout, cause: String(cause) }));
      }
    });
  });
}

function failure(stderr: string, code: number | null): AcvcError {
  try {
    const parsed = JSON.parse(stderr) as { error?: string; context?: unknown };
    if (parsed.error) return new AcvcError(parsed.error, parsed.context ?? {});
  } catch {
    return new AcvcError(stderr.trim() || `acvc exited ${code}`, { code });
  }
  return new AcvcError(stderr.trim() || `acvc exited ${code}`, { code });
}
