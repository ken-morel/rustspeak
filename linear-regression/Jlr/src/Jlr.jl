module Jlr

using FunctionWrappers: FunctionWrapper

mean(vals::AbstractVector{T}) where {T} = T(sum(vals) / length(vals))

const LinearFunction{T, N} = FunctionWrapper{T, NTuple{N, T}}


linear_regression(fn::Function; kw...) = linear_regression(
    LinearFunction{
        Float64,
        length(first(methods(fn)).sig.parameters) - 1,
    }(fn); kw...
)

const LinearRegressionResult{T, N} = Tuple{NTuple{N, T}, T}

function linear_regression(
        fn::LinearFunction{T, N};
        train_size = 1000,
        epochs = 50_000,
        precision::Int = 10
    )::Channel{LinearRegressionResult} where {N, T}
    return Channel{LinearRegressionResult}() do channel
        Threads.@threads for learn_rate in 10.0 .^ (-2:0)
            println("Starting with learn rate $learn_rate")
            splat_fn = Base.splat(fn)
            train_x = rand(T, train_size, N)
            train_y = splat_fn.(eachrow(train_x))

            weights = rand(T, N)
            bias = rand(T)
            last_loss = Inf
            oepoch = 0

            for epoch in 0:epochs
                oepoch = epoch

                # TODO: pass descent algorithm as argument to allow other descents

                pred_y = (train_x * weights) .+ bias

                loss = mean((pred_y .- train_y) .^ 2)

                errors = pred_y .- train_y


                weights .-= learn_rate * (2 / train_size) * (train_x' * errors)
                bias -= learn_rate * (2 / train_size) * sum(errors)

                # last_loss ≈ loss && break
                last_loss = loss
            end
            isnan(bias) || any(isnan.(weights)) || push!(channel, (tuple(round.(weights; digits = precision)...), round(bias, digits = precision)))
        end
    end
end
precompile(linear_regression, Tuple{})

# fn = linear_regression((vals) -> sum(vals .* 2) - 1)

end # module Jlr
