# Full-suite runtime faults remain unresolved

defines: demo.blackjack.web.regression full-run qualification
consumer: demo.blackjack.web.verification successful bounded demo evidence

2026-10-02, Windows Juliaup 1.12.7:

1. `--startup-file=no -O0 -C generic`: compiler EXCEPTION_ACCESS_VIOLATION in
   `walk_binding_partition`, while loading Hold'em stage 04. Exit 1.
2. `--startup-file=no --compile=min -O0 -C generic`: 27,974 passed, 73 broken,
   one ReadOnlyMemoryError in Sudoku generation, exit 1. All four website test
   groups passed in this run.
3. Unchanged standalone `julia-tests/runtests.demo.sudoku.teaching.jl` with the
   ordinary flags: 139 passed, exit 0.

This resembles the earlier Windows Julia compiler/runtime instability, but the
underlying cause has not been established here. Do not claim a clean full-suite
run or modify unrelated game algorithms solely to suppress these runtime faults.
The local website's 8,702 assertions and browser acceptance are independently
green. Logs are preserved in `observations`; investigate the Julia/runtime
boundary separately if a completely green monolithic run is needed for release.
