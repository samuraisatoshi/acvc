# ACVC operational assistant

Run `acvc assistant`. The command returns this contract in the computer's language. It does not depend on this file being open.

Detection, when choosing a folder by hand: `LC_ALL`, then `LC_MESSAGES`, then `LANG`. Skip an empty value, `C`, and `POSIX`. Map `pt` to `pt-BR/`, `en` to `en-US/`, and `es`, `fr`, `nl`, `el`, `pl`, `ru` to the folder of that name. Anything else uses `en-US/`.

An agent with the ASB MCP indexes the contract once. Submit the `documents` array from `acvc assistant` to `rag.index`. Later context search uses `rag.search`.

The executable is the source of truth. Do not open `.acvc/graph.sqlite`, the log, or the pack.
