/// The Sequential model is a linear stack of layers.

use crate::layer::Layer;
use crate::activation;
use crate::tensor::Tensor;

pub struct Sequential {
    layers: Vec<Layer>,
}

impl Sequential {
    /// Create a new empty Sequential model.
    pub fn new() -> Sequential {
        Sequential { layers: Vec::new::<>() }
    }

    /// Add a layer to the model
    pub fn add(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    /// Use this function to train the model on x_train with target y_train.
    /// Set `verbose` to true to see debugging and training information.
    pub fn fit(&mut self, x_train: Tensor, y_train: Tensor, epochs: u32, verbose: bool) {
        let alpha = 0.002;

        // Initialize weights with random values
        let mut weights: Vec<Tensor> = Vec::new();
        for i in 0..self.layers.len() {
            let unit = self.layers[i].unit;
            let input_size = if i == 0 {x_train.shape[1]} else {self.layers[i-1].unit};
            weights.push(Tensor::random(vec![unit, input_size], 0));
        }

        for iteration in 0..epochs {
            let mut error = 0.0;

            // iterate through samples
            for i in 0..x_train.shape[0] {
                // SGD implementation below

                // Forward propagation
                let mut outputs: Vec<Tensor> = Vec::new();
                // ouput of the first layer is the training sample...
                outputs.push(x_train.get_row(i).get_transpose());
                for w in &weights {
                    let output =  (w * &outputs.last().unwrap()).map(activation::relu);
                    outputs.push(output);
                }
                
                // Compute error
                error += (outputs.last().unwrap() - &y_train.get_row(i))[0].powi(2);

                // Compute backward pass
                let mut gradients: Vec<Tensor> = Vec::new();
                
                // First gradient (delta L)
                let gradient = outputs.last().unwrap() - &y_train.get_row(i);
                gradients.push(gradient.mult_el(&(weights.last().unwrap() * &outputs[outputs.len() - 2]).map(activation::relu2deriv)));

                // Other gradients (delta i)
                for (i, w) in weights.iter().skip(1).rev().enumerate() {
                    let left_gradient = &w.get_transpose() * &gradients.last().unwrap();
                    let right_gradient = (&weights[weights.len() - 2 - i] * &outputs[outputs.len() - 3 -i]).map(activation::relu2deriv);
                    let gradient = left_gradient.mult_el(&right_gradient);
                    gradients.push(gradient);
                }

                // Weight update
                for (i, w) in weights.iter_mut().enumerate() {
                    *w -= alpha * (&gradients[gradients.len() - 1 - i] * &outputs[i].get_transpose());
                }
                
            }
            
            if verbose {
                if iteration % 10 == 9 {
                    println!("Error: {}", error);
                }
            }
        }
    }
}

