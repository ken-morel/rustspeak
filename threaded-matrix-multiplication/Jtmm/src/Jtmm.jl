module Jtmm

gen_matrix() = rand(Int8, 100, 100) .|> Int

Base.@ccallable function julia_main(::Vector{String})::Int
    matrices = (;
        a = [gen_matrix() for _ in 1:100],
        b = [gen_matrix() for _ in 1:100],
    )
    result = Matrix{Int}[]

    @time Threads.@threads for (a, b) in @time zip(matrices...) |> collect
        push!(result, a * b .- a)
    end
    res
    println(summary(result))
    println(result[begin:5])

    tensor = (;
        a = rand(Int8, 100, 100, 100) .|> Int, # all random
        b = Array{Int, 3}(undef, (100, 100, 100)), # uninitialized
        c = repeat(gen_matrix(), 1, 1, 100), # the same matrix all way
    )
    for r in result
        tensor.a .+= r
    end
    tensor.a .*= tensor.c
    tensor.b .= tensor.c .* tensor.a
    for (label, tense) in pairs(tensor)
        println(summary(tensor[label]) * " ", label |> string |> uppercase, ":")
        println(tensor[label][begin:5])
    end
    0
end


(@main)(a) = julia_main(a)

end # module Jtmm
