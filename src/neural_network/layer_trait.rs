use ndarray::prelude::*;



pub trait Layer<T> {

    fn forward(&mut self, input : ArrayD<T>) -> ArrayD<T>;

    fn get_input_shape(&self) -> &Vec<usize>;

    fn get_output_shape(&self) -> &Vec<usize>;

    fn get_weights(&self) -> &Array<i16, Ix1>;

    fn get_bias(&self) -> i16;

    // In the process of training the model, the weights and the bias will be changed multiple times
    fn change_weights(&mut self, new_weights:Array<i16, Ix1>);
    fn change_bias(&mut self, new_bias:i16);
}