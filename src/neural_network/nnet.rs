// use std::async_iter::from_iter;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign};
// This import was deprecated
// use crate::cryptography::type_traits::{MyAdd, MyMul};
use crate::neural_network::layer_trait::Layer;

use ndarray::prelude::*;
use ndarray::{Array, Ix3, IxDynImpl};


// TODO: Add tests and examples for everything
pub struct Link(Option<usize>, Option<usize>);

pub struct Nnet<T> where T : Clone + Default + AddAssign + MulAssign  + Add<i32, Output = T>, i16 : Mul<T> + Add<T> {
    // HashMap <id, (&Layer, biases)>
    layers : Vec<Box<dyn Layer<T>>>,
}

impl<T> Nnet<T> where T : Clone + Default + Debug + AddAssign + MulAssign + Add<i32, Output = T> + Mul<i32> + Mul<i32, Output = T>, i32: Mul<T>, i16 : Mul<T> + Add<T>{
    pub fn new() -> Nnet<T> {
        Nnet {
            layers: Vec::new()
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

    pub fn forward(&mut self, input: ArrayD<T>) -> Result<(ArrayD<T>), &str> {
        let mut current_input = input;
        for mut layer in self.layers {
            let weights = layer.get_weights();
            let bias = layer.get_bias();
            let result = current_input.;

            current_input = layer.forward(result);
        }
        Ok(current_input)
    }
}
