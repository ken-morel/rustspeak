use ndarray::prelude::*;

 const FEATURES: usize = 2;
const MAX_EPOCHS: usize = 500_000;

fn linear_fn( arr: ndarray::ArrayView1<f64>) -> f64 {
     2.389 * arr[0] + 15.67 * arr[1] + 3.57
}
fn main() {
    const TRAIN_SIZE: usize = 3;
    let train_x = array![
        [0.1, 0.2],
        [0.2, 0.4],
        [0.6, 0.8]
    ]; // (3, 2)

    // (3, 1)
    let train_y: Array2<f64> = (&train_x).rows().into_iter().map(|row| linear_fn(row)).collect::<Array1<f64>>().insert_axis(Axis(1));

    let mut weights: Array2<f64> = (&train_x).columns().into_iter().map(|_| 1.).collect::<Array1<f64>>().insert_axis(Axis(0));
    let mut bias = 0.;

    let learn_rate = 0.1;

    let log_epochs = MAX_EPOCHS / 100;
    let mut loss: f64 = 1.;

    let mut epoch = 0;

    while loss > 1e-12 {
        epoch +=1;
                // column major, like in julia!!!
        let pred_y: Array2<f64> = (&train_x).dot(&(&weights).t()) + bias;
        

        loss = (&pred_y - &train_y).into_iter().map(|d| d.powf(2.)).sum();

        let errors:Array2<f64> = &pred_y - &train_y;

        // I hope operator precedence is like julia's
        let grad = (&train_x).t().dot(&errors).t().to_owned();
        weights = weights -( &learn_rate / TRAIN_SIZE as f64 * grad);
        bias -= (learn_rate  / TRAIN_SIZE as f64) * errors.sum();

        if epoch % log_epochs == 0 || epoch == 1 {
            print!("Epoch {epoch}, loss {loss:.4} y =");
            for x in 0..FEATURES {
                let n = x + 1;
                let w = weights[(0, x)];
                print!("{w:.3}x_{n} + ");
            }
            println!("{bias:.3}");
        }
    }
    println!("After {epoch} epochs and with a loss of {loss}");
     print!("The equation is \n> ");
            for x in 0..FEATURES {
                let n = x + 1;
                let w = weights[(0, x)];
                print!("{w:.3}x_{n} + ");
            }
            println!("{bias:.3}");


}
