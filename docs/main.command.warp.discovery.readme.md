# Warp discovery

Slice 0 records current behavior; Slice 1 implements inventory; Final verifies the result.

State: completed, three slices accepted. Verification commands, results, source
fingerprints and limitations are retained in main.command.warp.discovery.slice-final.complete.md.

Default: recur warp, equivalent to recur warp list, lists remaining declared bubbles and rings below the selected root. recur warp list --all includes complete entries. Existing merge projection determines completion, not current filenames. Invalid or ambiguous manifests remain visible as errors. No writes, scheduler, implicit private-directory scan, or repair of stale capsules is in scope.

Map: main.command.warp.discovery.warp-map.json. Evidence references are declared local verification records, not machine-checked producer receipts.
