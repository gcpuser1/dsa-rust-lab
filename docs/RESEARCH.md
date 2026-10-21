# Curriculum review — 2026-10-21

## Finding

The core scope remains appropriate for a first DSA course. Fundamental structures and techniques are durable; being current means using maintained language documentation, covering omissions and designing practice that tests transfer. No source establishes this exact 100-day Rust course as optimal. The schedule and prioritisation below are our synthesis for this learner and time budget.

## Evidence and decisions

| Primary source | What it establishes | Course decision |
|---|---|---|
| [NeetCode roadmap](https://neetcode.io/roadmap/) | Topic-dependent practice progression across core DSA areas | Use as coverage cross-check and problem bank; do not require completing the entire list |
| [NeetCode monotonic stack](https://neetcode.io/practice/problem-list/monotonic-stack) | Explicit practice category for monotonic stacks | Add next-greater-element/Daily Temperatures-style reasoning to the stack block |
| [MIT 6.006 syllabus](https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/pages/syllabus/) | An algorithms course assumes programming and discrete-maths foundations | Integrate prerequisite maths gradually; do not pretend the full university course fits this time budget |
| [MIT 6.006 2011 syllabus](https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-fall-2011/pages/syllabus/) | Algorithm descriptions include worked examples, correctness and complexity | Make those part of completion evidence |
| [Tech Interview Handbook study guides](https://www.techinterviewhandbook.org/algorithms/study-cheatsheet/) | Covers matrices, intervals, binary topics alongside common structures | Make these explicit rather than implicit |
| [Binary guide](https://www.techinterviewhandbook.org/algorithms/binary/) | Binary representation, bit operations and integer edge cases merit some practice, with lower priority than core topics | Include bounded bit coverage; avoid an extensive bit-trick syllabus |
| [Grind 75](https://www.techinterviewhandbook.org/grind75/) and [FAQ](https://www.techinterviewhandbook.org/grind75/faq) | Customisable practice; mixed topics and revisits rather than a single pass | Adapt selection to demonstrated weaknesses and use unlabelled mixed assessments |
| [LeetCode 75](https://leetcode.com/studyplan/leetcode-75/) and [Top Interview 150](https://leetcode.com/studyplan/top-interview-150/) | Existing lists have different coverage/time expectations | Curate a subset; do not add 150 problems to a one-task-per-day course |
| [Rust Book](https://doc.rust-lang.org/book/) and [2024 edition guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html) | Official current language reference; Rust 2024 introduced in 1.85 | Use edition 2024 and stable tools; rust-version 1.85 is the edition floor, not a promise to test every older compiler |
| [Rust ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) and [UTF-8 strings](https://doc.rust-lang.org/book/ch08-02-strings.html) | Ownership/borrowing and byte vs character distinctions matter to implementation | Separate Rust mistakes from DSA mistakes; require explicit text assumptions |
| [Rust learning resources](https://rust-lang.org/learn/) and [Rustlings](https://github.com/rust-lang/rustlings) | Practical language exercises alongside the Book | Use selected exercises as replacements for needed language remediation, not a second complete course |
| [VisuAlgo](https://visualgo.net/) | Animated structures and algorithms with interactive learning facilities | Use visual traces and predict-before-running experiments |
| [MIT Open Learning: spacing/interleaving](https://openlearning.mit.edu/mit-faculty/research-based-learning-findings/spaced-and-interleaved-practice) | Spacing and mixing practice support retention and distinguishing problem types | Build in review and delayed transfer; exact review intervals are our adjustable heuristic |

Pages were retrieved during this review. Their publication dates vary; retrieval on this date does not mean every linked curriculum was published in 2026. Resource contents can change. No paid subscription is required by the bootstrap.

## Additions relative to the original plan

1. Matrix/grid indexing and traversal, distinct from linear algebra.
2. Monotonic stacks and explicit binary-search boundary/answer-space reasoning.
3. Basic bits/XOR plus checked arithmetic, signedness and overflow assumptions.
4. Rust UTF-8 distinctions: bytes, Unicode scalar values and grapheme clusters; ASCII restrictions must be explicit. `char` is not always a user-perceived character.
5. Early memoisation exposure before the dedicated DP block, with 1D/2D state reasoning later.
6. Small brute-force oracles and operation-count experiments; asymptotic arguments remain necessary.
7. Unlabelled transfer assessments, honest assistance logs and separate language/algorithm evidence.

These are additions to clarity and coverage within existing slots, not extra daily workload.

## What we deliberately do not add

No linear algebra track, full Rust course, large production project, advanced competitive programming or comprehensive systems-design course inside these 100 days. These would dilute the agreed objective. Balanced-tree implementation, full O(1) LRU, MST, advanced DP and range-query trees remain extension material.

## Practical risks

- Learning Rust and DSA together can slow early progress. Use pseudocode/diagrams first and bounded syntax help.
- A fixed sequence cannot ensure mastery in 100 days. Replace optional depth with remediation when evidence requires it.
- Interview language support is employer-specific; verify it for actual interviews rather than assuming Rust is accepted everywhere.
- A problem list and green CI cannot establish interview readiness. Repeated unseen problem performance is required.
