
use std::collections::HashMap;
use std::error::Error;
use crate::neural_network::layer_trait::Layer;
use crate::neural_network::layer_type::LayerType;
use crate::tensor_library::matrix::Matrix;

pub struct Nnet<T> where T : Clone + Default {
    // HashMap <id, (&Layer, weights, biases)>
    layers : HashMap<usize, (LayerType<T>, Vec<i32>, Vec<i32>)>,
    // HashMap <from_layer, to_layer>
    nodes : HashMap<Option<usize>, Option<usize>>,
    data_type: M
}

impl<T, M> Nnet<T, M> where T : Layer + 'static, &'static T : Layer, M : Clone + Default {
    pub fn new() -> Nnet<T, M> {
        Nnet {
            layers: HashMap::new(),
            nodes: HashMap::new(),
            data_type: M,
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
    /// println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// ```
    ///
    pub fn add_layer(&mut self, layer : T, weights : Vec<i32>, biases : Vec<i32>) -> usize {
        self.layers.insert(self.layers.len(), (layer, weights, biases));

        // The id of the added layer is returned so it can be used when attaching nodes to it
        self.layers.len()-1
    }

    /// Adds node between two layers if they exists
    ///
    /// # Example:
    ///```
    /// use Cryptonic::layers::dense_layer;
    /// use Cryptonic::nnet::Nnet;
    /// let mut nnet : Nnet<dense_layer> = Nnet::new();
    /// let l : dense_layer = dense_layer {};
    /// println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// println!("{}", nnet.add_layer(&l, vec![1;5]));
    /// nnet.add_node(0, 1);
    /// ```
    ///
    pub fn add_node(&mut self, from_layer_id : Option<usize>, to_layer_id : Option<usize>) -> Result<(), &str> {
        if from_layer_id == None && to_layer_id != None {
            return self.add_first_node(to_layer_id.unwrap());
        }
        if from_layer_id != None && to_layer_id == None {
            return self.add_last_node(from_layer_id.unwrap());
        }
        if from_layer_id == None && to_layer_id == None {
            return  Err("Both layers are None, expected at least one of them to be Some<usize>");
        }
        if !self.layers.iter().any(|(x, _)| x == &from_layer_id.unwrap()) || !self.layers.iter().any(|(x, _)| x == &to_layer_id.unwrap()) {
            return Err("One of the layer ids you provide doesn't exist in the  current neural network");
        }
        self.nodes.insert(from_layer_id, to_layer_id);
        Ok(())
    }

    pub fn forward(&self, input: Matrix<M>) -> Result<(), &str>{
        let first_layer_id = match self.get_first_layer_id() {
            Some(a) => a,
            None => return Err("You must select the first layer by adding a node from None to first layer id!")
        };
        let (first_layer, weights, biases) = self.layers.get(&first_layer_id).unwrap();
        if input.shape() != first_layer.get_input_shape() {
            return Err("invalid input shape");
        }
        Ok(())
    }

    fn add_first_node(&mut self, layer_id : usize)  -> Result<(), &str> {
        if !self.layers.iter().any(|(x, _)| x == &layer_id) {
            return Err("The layer id you provide doesn't exist in the  current neural network");
        }
        self.nodes.insert(None, Some(layer_id));
        Ok(())
    }

    fn add_last_node(&mut self, layer_id : usize)  -> Result<(), &str> {
        if !self.layers.iter().any(|(x, _)| x == &layer_id) {
            return Err("The layer id you provide doesn't exist in the  current neural network");
        }
        self.nodes.insert(Some(layer_id), None);
        Ok(())
    }

    fn get_first_layer_id(&self) ->  Option<usize> {
        for (from, to) in self.nodes {
            if from == None {
                return to;
            }
        }
        None
    }

    fn get_last_layer_id(&self) ->  Option<usize> {
        for (from, to) in self.nodes {
            if to == None {
                return from;
            }
        }
        None
    }
}
