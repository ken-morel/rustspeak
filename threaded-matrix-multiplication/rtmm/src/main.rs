use ndarray::prelude::*;
use ndarray::{Array, Array2};
use rayon::prelude::*;

// generate a matrix and pass ownership to the caller
fn main() {
    let vec1 = Array::<f64, _>::zeros(1000 .f());
    let vec2 = Array::<f64, _>::linspace(1., 1000., 1000);

    let vec3 = vec2.map(| x| *x + 1.);
    let vec4 = &vec1 + &vec3;

    println!("vec4: {:?}: ", vec4);
    

    let mut matrices1: [Array2<f64>;100] = std::array::from_fn(|_| Array2::ones((100, 100).f()));
    let mut inputtext = String::new();
    _ = std::io::stdin().read_line(&mut inputtext); // guess inputtext will still be null
    let mut matrices2: [Array2<f64>;100] = std::array::from_fn(|_| Array2::from_elem((100, 100).f(),match  inputtext.trim().parse::<i32>() {
        Ok(n) => n as f64,
        Err(_) => 0.
    }) );
    let mut matrices_results: [Array2<f64>;100] = std::array::from_fn(|_| Array2::zeros((100, 100).f())); // well, don't know how to get it pre-allocated and unitnitialized
    // well let me still seperate the steps
    let mut indices: [usize; 100] = std::array::from_fn(|i| i as usize); 
    for i in 0..100 {
        indices[i] = i
    }

    println!("results are: {:?}: ", matrices_results)
}
