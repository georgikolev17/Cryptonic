use std::fmt::Debug;
use std::ops::{Add, AddAssign, Deref, Mul, MulAssign};
// This import was deprecated
// use crate::cryptography::type_traits::{MyAdd, MyMul};
use crate::neural_network::layer_trait::Layer;

use ndarray::prelude::*;
use ndarray::Array;


// TODO: Add tests and examples for everything
pub struct Link(Option<usize>, Option<usize>);

pub struct Nnet<T> where T : Clone + Default + Debug + AddAssign + MulAssign + Add<i32, Output = T> + Mul<i32> + Mul<i32, Output = T> {
    // HashMap <id, (&Layer, biases)>
    pub layers : Vec<Box<dyn Layer<T>>>,
    pub are_weights_initialised : bool,
    pub are_biases_initialised : bool
}

impl<T> Nnet<T> where T : Clone + Default + Debug + AddAssign + MulAssign + Add<i32, Output = T> + Mul<i32> + Mul<i32, Output = T> {
    pub fn new() -> Nnet<T> {
        Nnet {
            layers: Vec::new(),
            are_weights_initialised: false,
            are_biases_initialised: false
        }
    }

    /// Adds new layer to the neural network
    ///
    /// # Example:
    /// ```
    /// use Cryptonic::layers::dense_layer;
    /// use Cryptonic::neural_network::nnet::Nnet;
    /// let mut nnet : Nnet<dense_layer> = Nnet::new();
    /// let l : dense_layer = dense_layer {};
    /// //println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// //println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// ```
    ///
    pub fn add_layer(&mut self, layer : Box<dyn Layer<T>>) -> usize {
        self.layers.push(layer);

        // The id of the added layer is returned so it can be used when attaching links to it
        self.layers.len()-1
    }

    /// Adds link between two layers if they exists
    ///
    /// # Example:
    ///```
    /// use Cryptonic::layers::dense_layer;
    /// use Cryptonic::neural_network::nnet::Nnet;
    /// use Cryptonic::nnet::Nnet;
    /// let mut nnet : Nnet<dense_layer> = Nnet::new();
    /// let l : dense_layer = dense_layer {};
    /// //println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// //println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// //nnet.add_link(0, 1);
    /// ```
    ///
    ///0


    pub fn forward(&mut self, input: Array<T, IxDyn>) -> Result<(Array<T, IxDyn>), &str> {
        if !self.are_biases_initialised {
            return Err("You have not initialised the biases for this neural network!");
        }
        if !self.are_weights_initialised {
            return Err("You have not initialised the weights for this neural network!");
        }
        let mut current_input = input;

        for layer in &self.layers {
            if layer.get_weights().shape() == &[0 as usize] {
                continue;
            }
            let weights = layer.get_weights();
            let bias = layer.get_bias();


            // Gets first dimension(i.e. size of previous layer)
            let number_of_input_elements = current_input.shape()[0];

            // Number of weights should be previous_layer_len * next_layer_len
            let number_of_output_elements = weights.len()/number_of_input_elements;
            let mut result = Vec::new();
            for i in 0.. number_of_output_elements {
                let mut current_res : T = T::default();
                for j in i*number_of_input_elements..(i+1)*number_of_input_elements {
                    let value = current_input[j%number_of_input_elements].clone()*weights[j];
                    println!("{:?}", current_input[j%number_of_input_elements]);
                    println!("{:?}", weights[j]);

                    current_res += value;
                }
                current_res = current_res.clone() + bias;

                result.push(current_res.clone());
            }
            current_input = Array::from_vec(result).into_dyn();
        }
        Ok(current_input)
    }
    /// Initialises the weights
    ///
    ///# Example
    /// ```
    ///
    /// ```
    pub fn initialise_weights(&mut self, weights : Vec<Array<i32, Ix1>>) ->Result<(), String> {
        if weights.len() != (self.layers.len()-1) {
            return Err(format!("{} sets of weights expected. Received {}!", self.layers.len()-1, weights.len()));
        }
        for i in 0..self.layers.len()-1 {
            // TODO: (Should work without this too) Check if current set of weights is appropriate. For example if the current layer has shape [2, 3] and the next has [3, 3], the number of weights in the current set should be equal to 2*3 * 3*3 = 54
            self.layers[i].change_weights(weights[i].clone())
        }
        self.are_weights_initialised = true;
        Ok(())
    }
    /// Initialises the biases
    pub fn initialise_biases(&mut self, biases : Vec<i32>) ->Result<(), String> {
        if biases.len() != (self.layers.len()-1) {
            return Err(format!("{} biases expected. Received {}!", self.layers.len()-1, biases.len()));
        }
        for i in 0..self.layers.len()-1 {
            self.layers[i].change_bias(biases[i])
        }
        self.are_biases_initialised = true;
        Ok(())
    }
}
