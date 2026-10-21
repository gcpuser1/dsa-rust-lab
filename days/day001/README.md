# Day 001 — Find the slowest request

## Real problem

A service emits a sequence of request latencies. Return the largest latency and the index of its first occurrence. An empty batch has no result. For `[12, 80, 25, 80]`, return `Some((1, 80))`.

Use `i32` so we can test a general numeric algorithm with negative values too; real latency validation is a separate concern.

## Before coding

1. What information must you keep as you read the sequence?
2. Can any unseen value change your current answer?
3. What should happen for empty input and tied maxima?
4. Draw a table with columns: index, current value, best index, best value. Trace the example by hand.

Do not read later prompts until you have answered the current one. Record reasoning in [notes.md](notes.md).

## Implement

Edit `src/day001.rs`. Borrow a slice; do not sort, clone or mutate the input. Use explicit loops if they help you reason. Run:

```sh
cargo test day001 -- --include-ignored
```

The initial `todo!()` is intentional. Once all five tests pass, remove their ignore attributes and run the README checks. Add a test for a case you think is missing.

## Explain back

- After processing the first k elements, what exactly does your stored answer mean?
- Why does checking each value suffice? Could a correct general algorithm skip a value?
- How many comparisons does your approach use for n nonempty values?
- What extra memory does it need?
- Why accept `&[i32]` rather than take ownership of a `Vec<i32>`?

## Transfer and review

Change the requirement to return the **last** occurrence of the maximum. Explain the smallest necessary change before coding it. Finish by describing one assumption and one mistake. Do not mark secure until a later independent re-solve.

Timebox: 5 min recall/baseline, 15 min questions/trace, 30 min coding, 15 min tests/explanation, 5 min notes.
