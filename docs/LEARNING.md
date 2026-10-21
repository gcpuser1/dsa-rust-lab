# Learning and assessment agreement

## Teaching loop

1. Start with a real engineering problem and explicit constraints.
2. Ask one substantive question; wait for the learner's reasoning.
3. Explore an obvious approach and a counterexample or limitation.
4. Draw a trace: sequence, memory diagram, pointer state, call tree or graph.
5. Derive the technique before introducing its name/formula where practical.
6. Implement a small Rust solution and predict experimental output.
7. Test, explain correctness/costs and teach the idea back.
8. Revisit it later through a changed requirement or unfamiliar formulation.

Be demanding about reasoning, constructive about mistakes. A wrong answer identifies the next teaching question. One correct response is not sufficient evidence of mastery.

## A 70-minute default

| Activity | Minutes |
|---|---:|
| Recall from an earlier session | 5 |
| Real problem, questions and visual derivation | 15 |
| Main coding attempt | 30 |
| Tests and explain-back | 15 |
| Evidence and next review | 5 |

On reviews/mocks, replace the lesson block with independent solving. On Rust-heavy days, use the same algorithm and simplify language friction. End with a concrete next step even if unfinished. A lighter day still gets one bounded coding task.

## Help policy

Make a genuine attempt before consulting an editorial. After 15–20 minutes of unproductive struggle, ask for a focused hint rather than stare indefinitely. Hint ladder: clarify an example → expose a counterexample → identify the needed information → suggest a structure → outline → full worked solution if requested. The exact timebox is adjustable.

Do not let code completion or an assistant generate the algorithm during independent work. Syntax/API help is allowed and recorded separately. After seeing a solution, close it, reconstruct it, explain it and later solve a variant. That session remains assisted until new evidence shows independence.

## Progress states

| State | Evidence |
|---|---|
| not_started | No attempt recorded |
| attempted | Reasoned attempt made, incomplete or incorrect |
| assisted | Completed with algorithmic help or a viewed solution |
| independent | Correct solution, reasoning and tests without algorithmic help |
| secure | Independent delayed re-solve plus unfamiliar transfer; internal mechanism, correctness and complexity explained |

Track Rust fluency separately in notes (blocked / developing / fluent for the task). CI and accepted platform submissions do not set these states automatically.

## Review without overload

Use next-day recall, then roughly 1-week and 3-week revisits for important or weak ideas. These are practical starting intervals, not a uniquely optimal scientific schedule. Prioritise retrieval failures and error patterns; do not schedule three full re-solves for every problem.

The roadmap includes review days. If review demand exceeds the time budget, replace a planned new/optional task. If a prerequisite fails twice, use the next session for a smaller corrective problem. Record rescheduling in progress.csv and keep the original day IDs as stable references.

Early practice is topic-focused. Once basic techniques are understood, mix old and new topics without labels so choosing the method becomes part of the problem.

## Explain-back questions

- What exact information does this structure retain?
- Why can this step safely discard information?
- What remains true after each iteration?
- What breaks if one assumption changes?
- How can I construct an input that defeats my approach?
- What are all the allocations, copies and recursive calls?
- Where would I use this in a real system, and what production constraints are missing from the toy exercise?

## Mock scoring

Score each dimension 0 (missing), 1 (prompted), or 2 (independent): requirements, derivation, correctness argument, implementation, testing, complexity/communication. A score is diagnostic; no single aggregate proves readiness. Record hints, elapsed time and errors. Final readiness requires several unseen representative problems with sound independent performance, including at least one delayed reassessment of a weakness.

## What to record

Update progress.csv only from actual work. Store explanation, attempts, trace, test output and help used in the day's notes. Never mark all days complete because the repository or tests were created. Use synthetic infrastructure examples, not employer data.
