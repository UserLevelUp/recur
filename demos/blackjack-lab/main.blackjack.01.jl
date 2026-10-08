# produces: demo.blackjack.hello optional name to greeting
function hello(name::AbstractString="World")
    isempty(strip(name)) && throw(ArgumentError("name must not be blank"))
    "Hello, $(strip(name))!"
end
