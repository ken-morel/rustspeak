// Include this to write output to the user
#include <math.h>
#include <stdio.h>

int main() {
  // train_size is the size of the training data
  size_t i, train_size = 1;
  printf("Welcome to linear regressor!\n");
  printf("Let's start training the model\n");
  printf("What number of graph points do you have?\n> ");
  scanf("%zd", &train_size);

  double x[train_size];
  double y[train_size];

  printf("Now, enter continuously values in the form x,y."
         " You will enter %zd of them\n",
         train_size);
  for (i = 0; i < train_size; i++) {
    printf("Enter point #%zd> ", i + 1);
    scanf("%lf,%lf", &x[i], &y[i]);
  }
  // initial gradient and intercept, you will continuously improve them
  // to make them closer to the actual curve
  double gradient = 1.0, intercept = 0.0;

  // the number of times you will improve on the model
  printf("For how many epochs should we train? default 5000\n> ");
  size_t epochs;
  scanf("%zd", &epochs);
  if (epochs == 0)
    epochs = 5000; // default value

  printf("What should be learn rate of the model? default 0.01\n> ");
  double learn_rate;
  scanf("%lf", &learn_rate);
  if (learn_rate == 0)
    learn_rate = 0.01; // default

  // the number of times to iterace before printing
  size_t log_epoch = epochs / 100;

  for (size_t epoch = 0; epoch < epochs; epoch++) {
    double sum_errors = 0;
    for (i = 0; i < train_size; i++) {
      // y = mx + c
      double y_guess = gradient * x[i] + intercept;
      // find difference with actual value
      double error = y_guess - y[i];
      if (error == 0)
        continue;
      sum_errors += fabs(error);
      // improve the gradient
      gradient -= learn_rate * x[i] * error;
      // improve the intercept
      intercept -= learn_rate * error;
    }
    if (epoch % log_epoch == 0)
      printf("epoch %zd, loss is %.4lf equation is y = %.2lfx + %.2lf\n",
             epoch + 1, sum_errors, gradient, intercept);
    if (sum_errors < 0.000001) {
      printf("Stopping to train the model because the error reduced alot");
      break;
    }
  }
  printf("The equation of the line is y = %.2lfx + %.3lf", gradient, intercept);
  return 0;
}
