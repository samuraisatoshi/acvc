# ACVC assistant

This folder is the contract for an agent that will use ACVC, or explain ACVC to another agent, in a Cursor, Claude Code, OpenCode, or equivalent session.

The command `acvc assistant` returns this contract in the computer's language. An agent with the ASB MCP indexes it once: submit the `documents` array to `rag.index`. Later search uses `rag.search`.

Read in this order, and stop when the question already has an answer:

1. `objetivo.md` — why ACVC exists.
2. `recursos.md` — what the executable stores and which commands exist.
3. `como-usar.md` — the order of a session.
4. `boas-praticas.md` — what to do every time.
5. `exemplos.md` — correct uses.
6. `mau-uso.md` — uses that break the contract.
7. `trade-offs.md` — when to use it and when not to.

Rules of this folder:

- The source of truth is the `acvc` executable. Do not open `.acvc/graph.sqlite`, the log, or the pack.
- The machine format is JSON. Human text is `--format text`.
- The installed binary is `acvc` on `PATH`. The page, TUI, VS Code, and Obsidian call that executable. Do not reimplement the graph.
- Do not invent a token count, a metric, a rejection, or an identifier. If the command fails, show the error.
