#!/usr/bin/env julia

"""
Red-first contract for explicit async awaits and bounded retry routing.
The current parser does not yet accept this 0.3 grammar.
"""

using Test

if !isdefined(Main, :MainLang)
    include(joinpath(@__DIR__, "..", "demos", "main.lang", "main.lang.runtime.jl"))
end
using .MainLang

const RETRY_AWAIT_SOURCE = read(
    joinpath(
        @__DIR__,
        "..",
        "demos",
        "recur-language",
        "main.lang.retry-await.recur",
    ),
    String,
)

const RETRY_AWAIT_PARSE_RESULT = try
    MainLang.parse_program(RETRY_AWAIT_SOURCE)
catch error
    error
end

@testset "retry await source freezes bounded asynchronous control flow" begin
    @test occursin("recur 0.3 coordination RetryAwait", RETRY_AWAIT_SOURCE)
    @test occursin(
        "await all [validate.o(b).success, policy.o(c)] -> persist.i(d)",
        RETRY_AWAIT_SOURCE,
    )
    @test occursin("retry bounded max_attempts 3", RETRY_AWAIT_SOURCE)
    @test occursin("retry.o(g).exhausted -> failure_report.i(h)", RETRY_AWAIT_SOURCE)
    @test occursin("check every_await_has_producer", RETRY_AWAIT_SOURCE)
    @test occursin("check retries_are_bounded", RETRY_AWAIT_SOURCE)
    @test occursin("check every_retry_exhaustion_is_handled", RETRY_AWAIT_SOURCE)
    @test occursin("check no_retry_await_deadlocks", RETRY_AWAIT_SOURCE)
end

@testset "retry await parser contract (expected broken)" begin
    @test_broken RETRY_AWAIT_PARSE_RESULT isa MainLang.Program
end
