struct HelloApp
    prefix::String
end
HelloApp() = HelloApp("Hello")
# consumes: demo.holdem.hello name and optional prefix
# produces: demo.holdem.message bundled class result
function greet(app::HelloApp, name::AbstractString)
    isempty(strip(name)) && throw(ArgumentError("name must not be blank"))
    (message="$(app.prefix), $(strip(name))!",)
end
present(bundle) = bundle.message * "\n"
# produces: demo.holdem.call caller delegates to the class method
run_hello(name="World"; prefix="Hello") = present(greet(HelloApp(prefix), name))
