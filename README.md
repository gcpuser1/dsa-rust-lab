# DSA Rust Lab

100 days of practical data structures and algorithms in Rust, with 60–75 minutes per day.

**Goal:** understand internals, derive correct algorithms, and solve unfamiliar problems independently. Interview readiness is measured by repeated performance, not a streak or problem count.

## Start here

1. Read [the curriculum](docs/CURRICULUM.md) and [teaching agreement](docs/LEARNING.md).
2. Follow the [100-day roadmap](docs/ROADMAP.md). Start with [Day 001](days/day001/README.md).
3. Record evidence in [progress.csv](progress.csv); use the [session template](templates/session.md).
4. See the [research review](docs/RESEARCH.md) for sources, additions and deliberate exclusions.

## Continue with Kiro

Read or paste [KIRO_CONTEXT.md](KIRO_CONTEXT.md): it contains the complete curriculum, all 100 sessions, teaching method and tutor instructions. Open this repository in Kiro to use the included `.kiro/steering/dsa-tutor.md` file and AGENTS.md.

Start with: "Read KIRO_CONTEXT.md and SESSION_STATE.md. Act as my Socratic DSA tutor. Resume the current day, ask one question at a time, and wait for my attempt before showing a solution."

[SESSION_STATE.md](SESSION_STATE.md) and progress.csv record live progress; the guide's initial handoff is a dated snapshot.

## Run locally

Install Rust from https://rustup.rs if needed. This project uses Rust 2024 (compiler 1.85+), has no third-party dependencies, and uses the installed toolchain. CI tracks stable Rust.

```sh
cargo test --locked
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo run --example vec_capacity
```

Day 001 is intentionally unsolved. Its five tests are ignored until you start; a green scaffold check does not mean the exercise is complete.

```sh
cargo test day001 -- --include-ignored
```

This command initially fails at the exercise's `todo!()`. After solving it, remove the five `#[ignore]` attributes and rerun the normal checks. Do not weaken the tests to make a solution pass.

## Learning method

Real problem → questions → visual trace → derive an approach → implement → test → explain back → delayed transfer problem.

One main coding task each day. Review and assessment tasks count. No full solution before a genuine attempt; use progressively stronger hints and reconstruct any studied solution from memory.

## Repository map

| Path | Purpose |
|---|---|
| `docs/` | Scope, sequence, teaching method, research and assessment |
| `days/day001/` | First session brief; future briefs added as learning progresses |
| `src/` | Compiled Rust exercises; Day 001 scaffold included |
| `examples/` | Small practical experiments |
| `templates/` | Session notes and future Rust exercise template |
| `progress.csv` | 100 planned sessions, initially not started |
| `.github/workflows/` | Formatting, lint and test checks |

Later sessions are planned in the roadmap, not pre-solved. Reschedule rather than rush when a prerequisite is weak. Keep employer data, credentials and real production logs out of this learning repository.
