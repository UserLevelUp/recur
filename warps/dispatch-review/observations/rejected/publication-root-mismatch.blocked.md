artifact.type = lane
publish: main.command.warp.dispatch.review.coordination.publication_root_mismatch failed acceptance projection
consumer: main.command.warp.dispatch.review.coordination corrected root binding

The companion wrote the original implementation layer with a checked reference
relative to the repository root. Standalone evidence assessment at that root
passed, but inventory/show assesses evidence at the map directory (warps), so
the live implementation gate and its dependent final gate were blocked.

The original layer bytes are preserved in publication-root-mismatch.json here;
it is an invalid publication observation, not accepted live coverage. No valid
historical completion layer was rewritten. A new root-corrected implementation
attempt binds the reviewed acceptance summary as declared evidence. Live show
then reports four of four accepted slices and declared-gates-satisfied.

The scoped 19-test dispatch evidence manifest is independently checked at the
repository root. It is supplemental evidence, not a checked inventory gate.
Full raw Cargo output separately records 287 passes and seven ignored doctests.
Preserve the distinction between these scopes. Future companion publication
should preflight the same root and evidence rules its intended reader uses.
