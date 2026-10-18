# How to use

1. `acvc context --campaign NAME` before proposing, measuring, or discarding.
2. If the campaign does not exist: `acvc init --repo .` and `acvc campaign create NAME --metric metric:higher`.
3. The first hypothesis is `seed` of the folder that holds the hypothesis files, with `--premise`, `--assumption`, `--intent`, and `--agent`.
4. A file changed: `record` from the parent. Do not `seed` again.
5. There is a real number for the declared metric: `score`. An incomplete trial does not enter the frontier.
6. `frontier` shows what still competes. `why` explains one hypothesis. `recall --query` finds it by the premise text.
7. Two concrete measurements: `compare`.
8. The hypothesis died for a known motive: `reject --reason`. Without a motive, do not reject. The reading already says the motive was not recorded.
9. The session spent tokens and that number is known: `spend`. Do not estimate.
10. The variant that won enters Git with `promote` into a checkout that already exists.

Identifier prefix: 8 to 64 hexadecimal characters, unambiguous.

For another agent on the same machine, the channel is the executable or `acvc mcp`. The page and the TUI are for the person. Do not replace them with a direct read of the database.
