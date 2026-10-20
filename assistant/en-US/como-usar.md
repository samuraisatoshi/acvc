# How to use

The steps below use the command names. An attempt is a hypothesis. A measurement is a trial. The attempts nobody has beaten on every number are the frontier.

1. `acvc context --campaign NAME` before proposing, measuring, or discarding.
2. If the campaign does not exist: `acvc init --repo .` and `acvc campaign create NAME --metric metric:higher`.
3. The first hypothesis is `seed` of the folder that holds the hypothesis files, with `--premise`, `--assumption`, `--intent`, and `--agent`.
4. A file changed: `record`. Without `--parent`, the parent is the hypothesis the workspace points at. `record` then points the workspace at the new hypothesis, so the next `record` chains onto it. Two children of the seed need `--parent` with the seed id, or `materialize` of the seed before each `record`. Do not `seed` again.
5. There is a real number for the declared metric: `score`. An incomplete trial does not enter the frontier.
6. `frontier` shows what still competes. `why` explains one hypothesis. `recall --query` finds it by the premise text.
7. Two concrete measurements: `compare`.
8. The hypothesis died for a known motive: `reject --reason`. Without a motive, do not reject. The reading already says the motive was not recorded.
9. The session spent tokens and that number is known: `spend`. Do not estimate.
10. Use `promote --dest` to materialize the selected variant in a folder, or `promote --git` to create a commit in an existing Git worktree.

Identifier prefix: 8 to 64 hexadecimal characters, unambiguous.

For another agent on the same machine, the channel is the executable or `acvc mcp`. The page and the TUI are for the person. Do not replace them with a direct read of the database.
