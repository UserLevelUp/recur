#!/usr/bin/env julia

"""
Red-first contract for visible validation, refinement, and exception routing.
The current parser does not yet accept this 0.3 result-variant syntax.
"""

using Test

if !isdefined(Main, :MainLang)
    include(joinpath(@__DIR__, "..", "demos", "main.lang", "main.lang.runtime.jl"))
end
using .MainLang

const FORM_VALIDATION_SOURCE = read(
    joinpath(
        @__DIR__,
        "..",
        "demos",
        "recur-language",
        "main.lang.form-validation.recur",
    ),
    String,
)

const FORM_VALIDATION_PARSE_RESULT = try
    MainLang.parse_program(FORM_VALIDATION_SOURCE)
catch error
    error
end

@testset "form validation source freezes explicit result routing" begin
    @test occursin("recur 0.3 coordination FormValidation", FORM_VALIDATION_SOURCE)
    @test occursin("Result<ValidatedFormInput, ValidationErrors>", FORM_VALIDATION_SOURCE)
    @test occursin("validate.o(b).success -> refine.i(b)", FORM_VALIDATION_SOURCE)
    @test occursin("validate.o(b).failure -> form_feedback.i(d)", FORM_VALIDATION_SOURCE)
    @test occursin("check every_failure_outcome_is_handled", FORM_VALIDATION_SOURCE)
    @test occursin("check no_exception_cycles", FORM_VALIDATION_SOURCE)
end

@testset "form validation parser and graph contracts (expected broken)" begin
    @test_broken FORM_VALIDATION_PARSE_RESULT isa MainLang.Program
end
