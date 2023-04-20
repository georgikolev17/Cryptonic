extern crate core;

#[cfg(test)]
mod test_layers {
    use ndarray::array;
    use Cryptonic::neural_network::dense_layer::DenseLayer;
    use Cryptonic::neural_network::layer_trait::Layer;
    use Cryptonic::neural_network::nnet::Nnet;

    #[test]
    fn test_layer_shape() {
        let layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let input_shape : Vec<usize> = vec![3, 2];
        assert_eq!(layer.get_input_shape(), &input_shape);
    }
    #[test]
    fn test_change_bias() {
        let mut layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        layer.change_bias(20);
        assert_eq!(layer.get_bias(), 20);
    }
    #[test]
    fn test_change_weights() {
        let mut layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        layer.change_weights(array![10, 10, 10]);
        assert_eq!(layer.get_weights(), array![10, 10, 10]);
    }
}

#[cfg(test)]
mod test_neural_network {
    use std::ptr::null;
    use Cryptonic::neural_network::dense_layer::DenseLayer;
    use Cryptonic::neural_network::nnet::Nnet;

    #[test]
    fn test_adding_a_new_layer() {
        let mut nn : Nnet<i32> = Nnet::new();
        let mut layer : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        nn.add_layer(Box::new(layer));
        assert_eq!(nn.layers.len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_should_panic_if_number_of_weights_is_inappropriate() {

        let mut layer1 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);
        let mut layer2 : DenseLayer<i32> = DenseLayer::new(vec![3, 2]);

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
