extern crate core;

#[cfg(test)]
mod test_neural_network {
    use std::ptr::null;
    use Cryptonic::neural_network::nnet::Nnet;

    #[test]
    fn test_adding_a_new_layer() {
        let nn : Nnet<i32> = Nnet::new();

    }
}

#[cfg(test)]
mod test_layers {
    use Cryptonic::neural_network::dense_layer::DenseLayer;
    use Cryptonic::neural_network::layer_trait::Layer;
    use Cryptonic::neural_network::nnet::Nnet;

    #[test]
    fn test_layer_shape() {
        let layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let input_shape : Vec<usize> = vec![3, 2];
        assert_eq!(layer.get_input_shape(), &input_shape);
    }

}

/*
#[cfg(test)]
mod test_layers
{
    use crate::dense_layer;
    #[test]
    fn test_dense_layer_weights_and_biases() {
        let dense_layer = dense_layer::DenseLayer::new(20, 10);
        assert_eq!(10, dense_layer.weights.len());
        assert_eq!(20, dense_layer.weights[0].len());
        assert_eq!(10, dense_layer.biases.len());
    }
}
*/
