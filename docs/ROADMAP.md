# 100-day roadmap

One main task per session. All tasks start not started. Later briefs and exercise code are added just in time; this is a complete sequence, not a claim that 100 lessons are already implemented.

Use the questions and evidence rules in [LEARNING.md](LEARNING.md). Substitute a smaller corrective problem when a prerequisite is weak. Unlabelled assessments must be selected by the coach without showing the solution or topic first.

## Days 1–10: Rust, arrays and foundations

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 001 | practice | Find maximum and earliest index | Derive a scan invariant; borrow a slice |
| 002 | practice | Reverse an array in place | Trace swaps and bounds before coding |
| 003 | practice | Remove duplicates from a sorted array | Derive read/write pointers |
| 004 | practice | Merge two sorted sequences | Compare input size and extra space |
| 005 | practice | Valid palindrome with an explicit ASCII contract | Distinguish bytes from Unicode scalar values |
| 006 | implementation | Build a stack wrapper over Vec | Define push/pop and empty behaviour |
| 007 | review | Re-solve maximum with a changed tie rule | Explain from memory; separate Rust and algorithm errors |
| 008 | practice | Best time to buy and sell stock | Retain only the history needed for a future decision |
| 009 | implementation | Explore Vec length/capacity and implement linear search | Predict allocation observations; count comparisons |
| 010 | assessment | Array and Rust baseline assessment | Solve an unlabelled variant and explain ownership |

## Days 11–20: Hashing, prefix sums and bits

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 011 | practice | Contains duplicate | Compare pairwise checking with set membership |
| 012 | practice | Two Sum | Derive lookup requirements and duplicate handling |
| 013 | implementation | Build a log-frequency analyser | Use HashMap entry API and borrowed vs owned keys |
| 014 | practice | Valid anagram | State text assumptions and compare frequency tables |
| 015 | practice | Group anagrams | Construct a stable grouping key |
| 016 | practice | Range sum query | Derive prefix sums and endpoint conventions |
| 017 | review | Delayed array/hash transfer | Choose a method without a topic label |
| 018 | practice | Subarray sum equals K | Explain why a sliding window is insufficient with negatives |
| 019 | implementation | Build a tiny separate-chaining hash table | Trace collisions with integer keys; document toy limits |
| 020 | practice | Single number and bit flag operations | Derive XOR cancellation; handle width and overflow assumptions |

## Days 21–30: Pointers, windows and binary search

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 021 | practice | Two Sum on sorted input | Justify which pointer can move safely |
| 022 | practice | Container with most water | Challenge the discard rule with counterexamples |
| 023 | practice | Three Sum | Combine sorting with duplicate avoidance |
| 024 | implementation | Build a fixed-window request counter | Define timestamp ordering and boundary semantics |
| 025 | practice | Longest substring without repeated characters | Define a shrinking-window invariant |
| 026 | practice | Longest repeating character replacement | Distinguish valid-window and best-length reasoning |
| 027 | review | Delayed hash/window transfer | Explain when negative numbers or text assumptions break a method |
| 028 | practice | Binary search exact match | Derive half-open boundaries and logarithmic work |
| 029 | practice | First/last position and search insert | Use lower/upper bounds without unsigned underflow |
| 030 | assessment | Mixed search/window assessment | Solve a new formulation without a pattern hint |

## Days 31–40: Stacks, queues and linked lists

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 031 | practice | Valid parentheses | Explain LIFO and early rejection |
| 032 | practice | Min Stack | Augment state while preserving operation costs |
| 033 | implementation | Build a queue wrapper over VecDeque | Contrast front removal from Vec |
| 034 | practice | Daily Temperatures | Derive a monotonic stack from unresolved requests |
| 035 | practice | Evaluate reverse Polish notation | Trace operand order and error assumptions |
| 036 | implementation | Build and reverse an owned singly linked list | Use Box and Option::take without unsafe code |
| 037 | review | Delayed stack/window transfer | Explain amortised pushes/pops from first principles |
| 038 | practice | Find middle and merge sorted linked lists | Trace ownership changes; compare arena representations |
| 039 | practice | Detect a cycle in an index-based linked structure | Derive fast/slow pointers; recognise owned-list limitations |
| 040 | practice | Reorder a linked list as bounded substeps | Split reverse merge; use an index model if Rust blocks reasoning |

## Days 41–50: Recursion, sorting and grids

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 041 | practice | Count ways to climb stairs recursively | Draw repeated subproblems and call frames |
| 042 | implementation | Implement merge sort | Derive a recurrence and count temporary storage |
| 043 | practice | Compare quicksort partitions with merge sort | Explain pivots worst cases stability and library sorting |
| 044 | practice | Search a sorted 2D matrix | Map row/column coordinates to a search space |
| 045 | practice | Spiral matrix traversal | Maintain shrinking boundaries and avoid double visits |
| 046 | practice | Flood fill with DFS | Choose visited-state semantics and termination |
| 047 | review | Delayed recursion/pointer transfer | Explain call-stack cost and base cases |
| 048 | practice | Add memoisation to stairs | Separate distinct states from total calls |
| 049 | implementation | Build a grid traversal helper and island counter | Handle neighbours and bounds explicitly |
| 050 | practice | Count paths with memoisation | Introduce a two-dimensional state before formal DP |

## Days 51–60: Trees and traversals

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 051 | practice | Maximum depth of binary tree | Connect height to recursive work and space |
| 052 | practice | Invert a binary tree | Trace transformations using owned nodes |
| 053 | implementation | Build an arena-based BST with insert/search | Define duplicate policy and reason about deletion cases |
| 054 | practice | Validate a BST | Explain why parent-only checks fail |
| 055 | review | Delayed tree/recursion transfer | Compare recursive and explicit-stack traces |
| 056 | practice | Binary-tree level-order traversal | Derive BFS queue state |
| 057 | practice | Lowest common ancestor in a BST | State ordering and node-existence assumptions |
| 058 | practice | Kth smallest element in a BST | Derive in-order traversal and early stopping |
| 059 | practice | Diameter and balance reasoning | Return useful summaries from subproblems |
| 060 | assessment | Tree assessment and skewed-tree experiment | Contrast O(log n) balanced height with O(n) skew |

## Days 61–68: Heaps and tries

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 061 | practice | Kth largest element in a stream | Derive a bounded min-heap invariant |
| 062 | implementation | Implement binary-heap sift operations | Trace array indices and heapify reasoning |
| 063 | practice | Top K frequent events | Combine counting and priority selection |
| 064 | practice | Merge K sorted streams | Track one candidate from each stream |
| 065 | review | Delayed heap/tree transfer | Choose heap sorting or ordered traversal and justify |
| 066 | practice | Task scheduling with a priority queue | Model ordering ties and cooldown assumptions |
| 067 | implementation | Build a tiny trie | Implement insert and exact/prefix lookup |
| 068 | practice | Word dictionary with wildcard search | Combine trie traversal with recursive choices |

## Days 69–80: Graphs and dependencies

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 069 | practice | Number of islands with BFS | Model a grid as an implicit graph |
| 070 | implementation | Build a dependency graph from synthetic services | Use adjacency lists and isolated nodes |
| 071 | practice | Clone a graph using an index mapping | Separate identity from node value |
| 072 | practice | Rotting oranges | Derive multi-source BFS levels |
| 073 | review | Delayed graph/queue transfer | Recognise disconnected components and cycles |
| 074 | practice | Course schedule cycle detection | Contrast directed and undirected cycle rules |
| 075 | practice | Produce a deployment order | Derive indegree-based topological sorting |
| 076 | implementation | Build union-find | Explain path compression and union by size |
| 077 | practice | Redundant connection | Compare traversal with incremental connectivity |
| 078 | practice | Shortest path in an unweighted graph | Explain why FIFO gives shortest hop count |
| 079 | implementation | Implement Dijkstra on nonnegative weights | Use Reverse and skip stale heap entries |
| 080 | assessment | Graph assessment and negative-weight counterexample | Explain where Dijkstra's guarantee fails |

## Days 81–90: Backtracking, greedy and DP

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 081 | practice | Generate subsets | Draw a choice tree and define output-size cost |
| 082 | practice | Combination Sum | State reuse rules and derive pruning conditions |
| 083 | practice | Permutations | Manage choices and undo state correctly |
| 084 | review | Delayed recursion/tree/graph transfer | Choose a traversal from constraints alone |
| 085 | practice | Merge intervals and insert interval | Define touching endpoints and sorting assumptions |
| 086 | practice | Non-overlapping intervals | Derive a greedy choice and test counterexamples |
| 087 | review | Delayed heap/interval transfer | Explain why a plausible greedy alternative fails |
| 088 | practice | House Robber | Derive states transitions base cases and rolling memory |
| 089 | practice | Coin Change | Compare greedy failure with DP; explain update order |
| 090 | review | DP repair and 2D grid-path re-solve | Reconstruct memoisation then bottom-up reasoning |

## Days 91–100: Mixed assessment and repair

| Day | Type | Main task | Reasoning / experiment |
|---|---|---|---|
| 091 | assessment | Mock 1: unseen array/hash/search problem | Clarify derive code test explain within 45 minutes |
| 092 | review | Repair the weakest mock-1 skill | Use a smaller new variant and explain back |
| 093 | assessment | Mock 2: unseen stack/list/window problem | Record algorithmic hints separately from syntax help |
| 094 | assessment | Mock 3: unseen tree/heap problem | Derive correctness and account for recursion memory |
| 095 | review | Delayed re-solve of a recorded failure | Work from blank without the old solution |
| 096 | assessment | Mock 4: unseen graph/grid problem | Choose representation and handle disconnected input |
| 097 | assessment | Mock 5: unseen greedy/backtracking/DP problem | Derive state or choice; do not guess a label |
| 098 | review | Mixed transfer from the error log | Change a constraint and test whether the method survives |
| 099 | assessment | Mock 6: unseen representative medium | Repeat independent reasoning under time constraints |
| 100 | review | Final delayed assessment and next-course plan | Reassess one weakness and document evidence-based readiness |

## Task budget

60 new practice tasks + 15 implementation/applied tasks + 15 review tasks + 10 assessments = 100 sessions. A task can contain a closely related comparison or variant; do not turn it into multiple full daily problem sets. Days 91–100 contain six timed mocks and four targeted/delayed reviews.

These are targets, not mandatory completion counts. Record actual attempts, assistance and rescheduling honestly.
