artifact.type = warp
warp.id = main.command.warp.dispatch.review
warp.root = warps
goals.now = Preserve actual CLI-agent reviews, failed provider outcomes and coordinator decisions inside this Warp bubble
pull.first = recur warp show main.command.warp.dispatch.review -d . --json
pull.then = read warps/dispatch-review/observations and the coordination lane capsule
do.not.disturb = observations and produced attempts do not automatically accept gates; Gemini unavailability is not a Gemini code review
consumer: main.command.warp.dispatch.review.coordination reviewed handoffs

Requests and responses retain per-slice, per-attempt hierarchical trace IDs and
Eventness suffixes. Full worker records stay private under .recur/dispatch;
the shared observations directory preserves useful cross talk and provider reviews.
Claim/finish timestamps have whole-second precision. Observer timestamps describe
capture, including historical capture, and must not replace actual elapsed time.
