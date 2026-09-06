# Slice 1: Discovery implementation

Status: complete

Bare warp equals warp list; default excludes complete projections; --all includes them. Reuse bubble/ring merge, preserve invalid maps as diagnostic rows, deterministic output and existing directory exclusions. JSON and text are read-only.

Gate: verification. Retain commands and observed results before acceptance.

defines: recur.warp.discovery.slice.1 Discovery implementation


## Verified slice outcome

Implemented default/explicit inventory, --all, deterministic JSON/text and nonmutating diagnostic rows using existing bubble/ring merge. Focused Julia CLI suite: 36 passed. Cargo: 179 passed, seven ignored docs. Ring plus coordinator map is a valid pair; duplicate same-kind manifests remain errors.
