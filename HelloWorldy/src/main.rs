use burn::data::dataloader::batcher;
use burn::module::Module;
use burn::nn::{Linear, Loss, MSELoss, Optimizer, SGD};
use burn::tensor::{Data, Tensor};
use rand::Rng;
use textplots::{Chart, Plot, Shape};

// Define the linear regression model
#[derive(Module, Debug)]
struct LinearRegression {
    layer: Linear<f32>,
}

impl LinearRegression {
    fn new() -> Self {
        Self {
            layer: Linear::new(1, 1),
        }
    }

    fn forward(&self, input: Tensor<f32>) -> Tensor<f32> {
        self.layer.forward(input)
    }
}

// Generate synthetic data: y = 2x + 1 with noise
fn generate_data(n: usize) -> (Vec<f32>, Vec<f32>) {
    let mut rng = rand::thread_rng();
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();

    for _ in 0..n {
        let x: f32 = rng.gen_range(-10.0..10.0);
        let noise: f32 = rng.gen_range(-1.0..1.0); // Small noise
        let y = 2.0 * x + 1.0 + noise;
        x_data.push(x);
        y_data.push(y);
    }

    (x_data, y_data)
}

fn main() {
    let (x_train, y_train) = generate_data(100);

    // Convert to tensors
    let x_tensor = Tensor::from_data(Data::new(x_train.clone(), vec![100, 1]));
    let y_tensor = Tensor::from_data(Data::new(y_train.clone(), vec![100, 1]));

    // Initialize model
    let model = LinearRegression::new();

    // Define optimizer
    let mut optimizer = SGD::new(model.clone(), 0.01); // Learning rate 0.01

    // Training loop
    let epochs = 100;
    for epoch in 0..epochs {
        // Forward pass
        let predictions = model.forward(x_tensor.clone());

        // Compute loss
        let loss = MSELoss::compute(&predictions, &y_tensor);

        // Backpropagation
        optimizer.step(loss);

        // Print loss every 10 epochs
        if epoch % 10 == 0 {
            println!("Epoch {}: Loss = {:?}", epoch, loss);
        }
    }

    // Evaluate model on new data
    let (x_test, y_test) = generate_data(20);
    let x_test_tensor = Tensor::from_data(Data::new(x_test.clone(), vec![20, 1]));
    let predictions = model.forward(x_test_tensor);

    // Convert predictions back to Rust vectors
    let pred_values: Vec<f32> = predictions.into_data().value;

    // Plot results
    println!("Plotting results:");
    Chart::new(80, 25, -10.0, 10.0)
        .lineplot(&Shape::Points(&x_test, &y_test))
        .lineplot(&Shape::Points(&x_test, &pred_values))
        .display();
}
