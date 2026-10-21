# Curriculum

## Contract

100 sessions at 60–75 minutes each (about 100–125 hours). One main coding task each day; review, implementation and assessment count. Calendar dates are assigned when sessions happen. A missed day does not require two problems the next day.

Outcomes: derive solutions, understand internals, implement in Rust, justify correctness and complexity, test edge cases, and transfer ideas to unfamiliar problems. This is an initial foundation and interview-preparation course, not a guarantee of a particular hiring outcome.

## Core scope and sequence

| Days | Core scope | Practical connection | Mathematics and Rust |
|---|---|---|---|
| 1–10 | Baseline; arrays/slices; strings; scans; simple in-place changes | Latency samples; event identifiers | Bounds, counts, invariants; moves/borrows, Option, UTF-8, signed vs unsigned |
| 11–20 | Hash maps/sets; frequency tables; prefix sums; bit basics | Event deduplication and log summaries | Sets/functions; expected vs worst-case costs; XOR; overflow and checked arithmetic |
| 21–30 | Two pointers; fixed/variable windows; binary search and boundaries | Request windows and sorted timestamp searches | Monotone conditions, logarithms; half-open ranges and usize underflow |
| 31–40 | Stacks; queues/deques; singly linked lists; monotonic stack | Nested configuration; queues; next threshold crossing | LIFO/FIFO invariants and amortisation; VecDeque, Box, Option::take |
| 41–50 | Recursion; merge sort; quicksort reasoning; matrices/grids; early memoisation | Merge sorted logs; traverse a grid | Induction intuition, recurrences and call-stack space; indices vs references |
| 51–60 | Binary trees/BSTs; DFS/BFS; iterative traversal; balance and deletion reasoning | Hierarchical configuration and ordered lookup | Height vs node count; Box and arena/index representation |
| 61–68 | Heaps; top-K; priority queues; tries | Priority jobs, error rankings and prefix lookup | Heap invariant; BinaryHeap, Reverse, ordering; sparse trie children |
| 69–80 | Adjacency lists; graph/grid BFS/DFS; cycles; topological ordering; union-find; Dijkstra | Deployment dependencies, connected services, routing | O(V+E), connectivity; nonnegative weights for Dijkstra; stale heap entries |
| 81–90 | Backtracking; interval/greedy methods; foundational 1D and 2D DP | Explore choices, schedule jobs, count paths | Search trees; counterexamples for greedy; states/transitions/base cases and update order |
| 91–100 | Mixed unlabelled problems; targeted repair; mocks | Independent problem selection and communication | Explain trade-offs, test systematically, assess transfer |

These are learning windows, not deadlines to advance without understanding. Use review slots to repair prerequisites. Reduce optional depth before skipping core reasoning.

## Cross-cutting foundations

- Mathematics: logic, sets, powers/logarithms, counting/sums, invariants, proof by counterexample and induction intuition, simple recurrences and basic probability. Introduce notation after a concrete derivation.
- Complexity: input size, time vs extra space, worst/expected/amortised costs; recursion stack, cloning, sorting and collection conversion count too.
- Rust: functions, loops, structs/enums, match, Option/Result, ownership/borrowing, slices, iterators, collections, basic generics/traits and tests. Learn explicit loops first when useful; compare iterator forms later.
- Internals: stack/heap, contiguous storage, length/capacity, allocation, moves/copies, hash collisions/load, cache locality at a conceptual level.
- Correctness: contracts, invariants, empty/singleton inputs, duplicates, ordering assumptions, integer boundaries, Unicode assumptions, graph cycles/disconnected nodes.
- Experiments: predict results before running; count operations, test scaling and compare with a brute-force oracle on small generated inputs. Treat timing as noisy evidence, not proof of asymptotic complexity.
- Interview practice: clarify, derive brute force, improve, state invariant, code, test, explain trade-offs. Hide topic labels in mixed assessments.

## Practical implementation exercises

Selected exercises are embedded in daily slots: frequency analyser, sliding-window request counter, stack/queue wrapper, owned singly linked list, merge sort, grid flood fill, heap operations, tiny trie, dependency graph and union-find. Implement a small toy hash table to explore collisions; production-grade hashing is outside scope.

Use std collections for most problems. Explain a dynamic array's growth without building an unsafe allocator. Demonstrate LRU cache composition with diagrams; an O(1) implementation in safe Rust is optional extension work. No separate large capstone on top of daily work.

## Depth boundaries

Core: basic bits, matrix traversal, monotonic stack, binary-search-on-answer introduction, BST balance awareness, introductory DP in one and two dimensions.

Optional after secure prerequisites: monotonic deque, advanced binary-search-on-answer, minimum spanning trees, balanced-tree rotations, quickselect, full LRU implementation, more difficult DP.

Deferred: segment/Fenwick trees, suffix structures, network flow, computational geometry, advanced competitive programming, formal proof-heavy analysis, unsafe allocators, async/concurrency, full Rust mastery and system-design interview preparation.

Linear algebra, probability/statistics for ML and calculus belong in a separate AI track. Matrix DSA here means indexing and traversal, not matrix algebra.

## Assessment

See [LEARNING.md](LEARNING.md) for evidence and review policy. The final target is several independent medium-level mock problems in roughly 35–45 minutes each, including reasoning and tests. Difficulty varies by problem; this is a course target, not an employer standard. Confirm permitted interview languages with a recruiter for each actual interview; retain algorithm pseudocode fluency if Rust is unavailable.
