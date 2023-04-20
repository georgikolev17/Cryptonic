// use std::async_iter::from_iter;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign};
// This import was deprecated
// use crate::cryptography::type_traits::{MyAdd, MyMul};
use crate::neural_network::layer_trait::Layer;
use crate::neural_network::layer_type::LayerType;

use ndarray::prelude::*;
use ndarray::{Array, Ix3, IxDynImpl};

// TODO: Add tests and examples for everything
pub struct Link(Option<usize>, Option<usize>);

pub struct Nnet<T, D:Dimension> where T : Clone + Default + AddAssign + MulAssign  + Add<i32, Output = T>{
    // HashMap <id, (&Layer, biases)>
    layers : Vec<Box<dyn Layer<D, CType=T>>>,
}

impl<T, D:Dimension> Nnet<T, D:Dimension> where T : Clone + Default + Debug + AddAssign + MulAssign + Add<i32, Output = T> + Mul<i32> + Mul<i32, Output = T>, i32: Mul<T>{
    pub fn new() -> Nnet<T> {
        Nnet {
            layers: Vec::new(),
            links: HashMap::new(),
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
    pub fn add_layer(&mut self, layer_type : LayerType<T>, biases : Vec<i32>) -> usize {
        self.layers.push(layer_type);

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

    pub fn forward(&mut self, input: ArrayD<Self::CType>) -> Result<(ArrayD<Self::CType>), &str> {
        let mut current_input = input;
        for mut layer in self.layers {
            let weights = layer.get_weights();
            let bias = layer.get_bias();
            let result = current_input * weights + bias;

            current_input = layer.forward(result);
        }
        Ok(current_input)
        // // let first_layer_id = match &self.get_first_layer_id() {
        // //     Some(a) => a,
        // //     None => return Err("You must select the first layer by adding a link from None to first layer id!")
        // // };
        // let (first_layer, ibiases) = self.layers.get_mut(&0).unwrap();
        // if input.shape() != first_layer.get_input_shape() {
        //     return Err("Invalid input shape!");
        // }
        // let mut links_left : VecDeque<(&Option<usize>, &Option<usize>, &Vec<i32>)> = VecDeque::new();
        // for ((from, to), weights) in &self.links {
        //     if !from.is_none() && &from.unwrap() == &0 {
        //         links_left.push_back((from, to, weights));
        //     }
        // }
        // let input_iterator : MatrixIter<T> = MatrixIter {
        //     mat: &input,
        //     index: vec![0; input.shape().len()],
        //     current_el: None,
        //     empty: false,
        // };
        // let mut current_input:Vec<T> = Vec::new();
        // for (el, idx) in input_iterator {
        //     current_input.push(el+ibiases[idx.iter().fold(0, |acc, &x| acc + x)]);
        // }
        // let mut current_input = Matrix::from_iter(vec![current_input.len()], current_input, Layout::RowMajor);
        //
        // while !links_left.is_empty() {
        //     let (current_from_id, current_to_id, weights)  = links_left.pop_back().unwrap();
        //     let (current_from_layer, _biases) = self.layers.get_mut(&current_from_id.unwrap()).unwrap();
        //     if current_to_id.is_none() {
        //         current_input = current_from_layer.forward(current_input);
        //         break;
        //     }
        //     let (current_to_layer, biases) = self.layers.get_mut(&current_to_id.unwrap()).unwrap();
        //     // Add next links
        //     for ((from, to), weights) in &self.links {
        //         if !from.is_none() && &from.unwrap() == &current_to_id.unwrap() {
        //             links_left.push_back((from, to, weights));
        //         }
        //     }
        //
        //     // Forward propagation (could be in separate function)
        //
        //     let mut result : Vec<T> = Vec::new();
        //     let input_iterator : MatrixIter<T> = MatrixIter {
        //         mat: &current_input,
        //         index: vec![0; current_input.shape().len()],
        //         current_el: None,
        //         empty: false,
        //     };
        //     let mut ctr: usize = 0;
        //     for (el, idx) in input_iterator {
        //         let weights: Vec<i32> = weights.clone()[ctr*current_input.clone().shape.iter().sum::<usize>()..(ctr+1)*current_input.clone().shape.iter().sum::<usize>()].to_vec();
        //         let weights_matrix : Matrix<i32> = Matrix::from_iter(vec![current_input.clone().shape.iter().sum()], weights, Layout::RowMajor);
        //         let mut current_output_el = multiply_scalar_generic(weights_matrix.clone(), el.clone());
        //
        //         let iterator : MatrixIter<T> = MatrixIter {
        //             mat: &current_output_el,
        //             index: vec![0; current_output_el.shape().len()],
        //             current_el: None,
        //             empty: false,
        //         };
        //         // let mut sum = T::default();
        //         for (el, idx) in iterator {
        //             if result.len() > idx.iter().fold(0, |acc, &x| acc + x) {
        //                 result[idx.iter().fold(0, |acc, &x| acc + x)] += el;
        //             }
        //             else {
        //                 result.push(el);
        //             }
        //         }
        //         // sum = sum + *biases.get(idx[0]).unwrap();
        //
        //         ctr+=1;
        //     }
        //     result = result.iter().zip(biases.iter()).map(|(a, b)| a.clone()+b.clone()).collect();
        //
        //     // Activation function (and other layer operations)
        //     let forwarded_input: Matrix<T> = Matrix::from_iter(vec![result.len()], result, Layout::RowMajor);
        //     current_input = current_to_layer.forward(forwarded_input);
        // }
        //
        // Ok((current_input))
    }

    fn add_first_link(&mut self, layer_id : usize, weights : Vec<i32>)  -> Result<(), &str> {
        if !self.layers.iter().any(|(x, _)| x == &layer_id) {
            return Err("The layer id you provide doesn't exist in the  current neural network");
        }
        self.links.insert((None, Some(layer_id)), weights);
        Ok(())
    }

    fn add_last_link(&mut self, layer_id : usize, weights : Vec<i32>)  -> Result<(), &str> {
        if !self.layers.iter().any(|(x, _)| x == &layer_id) {
            return Err("The layer id you provide doesn't exist in the  current neural network");
        }
        self.links.insert((Some(layer_id), None), weights);
        Ok(())
    }

    fn get_first_layer_id(&self) ->  &Option<usize> {
        for ((from, to), _weights) in &self.links {
            if from.is_none() {
                return to;
            }
        }
        &None
    }

    fn get_last_layer_id(&self) ->  &Option<usize> {
        for ((from, to), _weights) in &self.links {
            if to.is_none() {
                return from;
            }
        }
        &None
    }

    fn get_layer_by_id(&self, id:usize) -> Option<&LayerType<T>> {
        for (key, value) in &self.layers {
            if key == &id {
                return Some(&value.0);
            }
        }
        None
    }
}
