# Trade-offs

Use this when there are two or more measured attempts and the numbers disagree. One straight path, with no second attempt, does not need a campaign.

## Use ACVC when

- The session tests more than one hypothesis and the next one needs to know which still competes.
- The premise changes, and the motive for abandoning one matters as much as the number.
- The latest trial is not, by itself, the best.
- Several metrics pull in opposite directions and both hypotheses should stay visible.
- Another agent, hours later, will recall instead of rereading the chat.

## Do not use it when

- The work is a single linear path and Git already tells the story that matters.
- There is no declared metric and no premise. Without those, ACVC becomes a second Git with no frontier.
- The question is only "what is in the working tree now". That is `git status` or `acvc status`, not a new campaign.
- The cost of writing premise, cause, and measurement is higher than the cost of forgetting the attempt.

## What you gain

Hypothesis memory, a Pareto frontier, premise recall, a motive kept apart from the score, and a Git deposit of only the variant that won.

## What you pay

An `.acvc` store beside Git. The discipline of not opening that store by hand. A metric declared before the trial. The scan has its own list, so a large file enters the hash unless it is in `.acvcignore` or on the fixed skip list. The remote copies objects. It does not do the merge Git would do.

## Agents in Cursor, Claude Code, and OpenCode

The resource those agents should call is `acvc` on `PATH`, or the `acvc mcp` server if the session already registered it. Both speak JSON. Neither opens SQLite. This folder states the contract. The executable's code remains the implementation.
