#!/usr/bin/env julia

include("main.lang.runtime.jl")
using .MainLang

exit(MainLang.main())
