use std::error::Error;
use crate::matrix::Matrix;
use crate::layer_trait::Layer;

pub struct Nnet<'a, T> where T : Layer {
    // Vec <id, Layer, weights>
    layers : Vec<(usize, &'a T, Vec<i32>)>,
    // Vec <from_layer, to_layer>
    nodes : Vec<(usize, usize)>,
}

impl<'a, T> Nnet<'a, T> where T : Layer {
    pub fn new() -> Nnet<'a, T> {
        Nnet {
            layers: vec![],
            nodes: vec![],
        }
    }

    /// Adds new layer to the neural network
    ///
    /// # Example:
    /// ```
    /// use Cryptonic::layer::layer;
    /// use Cryptonic::nnet::Nnet;
    /// let mut nnet : Nnet<layer> = Nnet::new();
    /// let l : layer = layer {};
    /// println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// ```
    ///
    pub fn add_layer(&mut self, layer : &'a T, weights : Vec<i32>) -> usize {
        self.layers.push((self.layers.len(), layer, weights));
        // The id of the added layer is returned so it can be used when attaching nodes to it
        self.layers.len()-1
    }

    /// Adds node between two layers if they exists
    ///
    /// # Example:
    ///```
    /// use Cryptonic::layer::layer;
    /// use Cryptonic::nnet::Nnet;
    /// let mut nnet : Nnet<layer> = Nnet::new();
    /// let l : layer = layer {};
    /// println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// nnet.add_node(0, 1);
    /// ```
    ///
    pub fn add_node(&mut self, from_layer_id : usize, to_layer_id : usize) -> Result<(), &str> {
        if !self.layers.iter().any(|&(x, _, _)| x == from_layer_id) || !self.layers.iter().any(|&(x, _, _)| x == to_layer_id) {
            return Err("One of the layers you provide doesn't exist in the  current neural network");
        }
        self.nodes.push((from_layer_id, to_layer_id));
        Ok(())
    }

    pub fn forward() {

    }
}
