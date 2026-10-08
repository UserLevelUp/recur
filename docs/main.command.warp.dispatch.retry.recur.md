artifact.type = lane
consumes: main.recur.reveal.type.lane retry/intervention work-context instructions
defines: main.command.warp.dispatch.retry companion-owned bounded retry instructions
defines: main.command.warp.dispatch.retry.provider_blocked stop for unsupported client, model or provider access
defines: main.command.warp.dispatch.retry.auth_required stop for human authorization
defines: main.command.warp.dispatch.retry.transient wait and retry within the configured budget
defines: main.command.warp.dispatch.retry.execution_error inspect and retry with bounded backoff
goals.now = Keep capable agents working without spending retries or intelligence on unavailable provider access
pull.first = recur warp dispatch WARP -d ROOT --json; inspect the latest failure object and retained output
pull.then = follow failure.trace_id and failure.action; inspect the live host assignment and retry policy
do.not.disturb = do not bypass authentication, guess provider access, change acceptance gates or silently choose a different host

# Simple coordinator instructions

`provider_blocked`: pause the slice. For example, UNSUPPORTED_CLIENT requires
review of client/provider access, rather than repeated sign-in or more intelligence.
The coordinator can propose a configured available host. Change the assignment
only within the authorized task; verify that host before explicitly reopening.

`auth_required`: pause the slice and request human authorization through the
provider's supported flow. A marker is an observed diagnostic, not proof that a
login window alone fixes access. Verify access before resuming. Never automate
credentials, account creation, billing consent or authentication bypass.

`transient`: wait until failure.retry_at_unix, then let recur-watch dispatch retry.
The default delay is 5, 10, 20, 40, then 60 seconds, capped at 60 and still bounded
by max_attempts. Timeouts also use this policy. Watch cycles can bound the wait;
they do not terminate workers. No attempt is consumed merely by waiting.

`execution_error`: retain and inspect the actual failure; unknown execution
errors use the same bounded backoff. Failed configured test assertions retain
their separate intelligence-feedback policy. Provider/runtime errors never count
as test failures or justify raising intelligence.

After human/configuration intervention, preview and then explicitly reopen the
latest exact attempt, within the remaining attempt budget:

```powershell
recur-warp recover WARP --slice SLICE --attempt N --reason "access verified" -d ROOT
recur-warp recover WARP --slice SLICE --attempt N --reason "access verified" -d ROOT --confirm
recur-watch dispatch WARP -d ROOT --confirm
```

Recovery preserves the diagnostic and original outcome. It does not terminate a
live worker or reset the attempt budget. Core queries never sign in, start jobs,
set host policy or accept results.

# Recorded lineage and limitations

Every new execution-failure record has failure.category, trace_id, action,
retryable and nullable retry_at_unix. Its attempt_trace_id stays under the Warp,
slice and attempt hierarchy, using underscore-safe trace tokens. Existing private
attempts and accepted historical layers are not rewritten. Older attempts without
failure metadata retain their previous bounded behavior until explicitly assessed.

Classification uses configurable case-insensitive diagnostic markers from bounded
failed-host log tails, with provider-blocked precedence over auth/transient. This
is advisory parsing, not an authenticated provider protocol. A successful agent
quoting an error code is not classified; successful-host output is also excluded
when diagnosing a verification execution error. Inspect raw output when uncertain.
No automatic cross-provider fallback or model permission changes are introduced.

On Windows, atomic record replacement also retries temporary filesystem access
denials for a bounded interval while keeping the same complete staged bytes.
An unresolved terminal publication error is retained beside the attempt in
`*.worker.error.txt`; inspect it before treating a retained running state as live.

consumer: main.command.warp.dispatch.config initialized retry policy
consumer: main.command.warp.dispatch asynchronous attempt records
