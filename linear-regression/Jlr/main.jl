using Jlr
println("starting timed")

@time for result in Jlr.linear_regression(_ -> 1)
    println(result)
end
