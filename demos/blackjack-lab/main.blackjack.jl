module BlackjackLab
using Random
for stage in 1:parse(Int,get(ENV,"BLACKJACK_STAGE","6"))
    include("main.blackjack.$(lpad(stage,2,'0')).jl")
end
end
