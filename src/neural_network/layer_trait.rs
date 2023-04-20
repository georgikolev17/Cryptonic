use ndarray::prelude::*;



pub trait Layer {
    type CType;

    fn forward(&mut self, input : ArrayD<Self::CType>) -> ArrayD<Self::CType> where <Self as Layer>::CType: Clone + Default;

    fn get_input_shape(&self) -> &Vec<usize>;

    fn get_output_shape(&self) -> &Vec<usize>;
}