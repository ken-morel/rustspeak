use ndarray::prelude::*;

fn main() {
    let train_x = array![
        [0.1, 0.2, 0.3],
        [0.2, 0.4, 0.6],
        [0.6, 0.8, 1.2]
    ];
    let train_y = array![
        1., 2., 3.
    ];
    println!("Hello, world!");
}
