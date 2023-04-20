use ndarray::prelude::*;


// Dimension D Specifies the dimensionality of the Data including the layer also
// The dimension D should look like (LayerSize, Data.Shape)
pub trait Layer<D: Dimension> {
    type CType;

    fn forward(&mut self, input : Array<Self::CType, D>) -> Array<Self::CType, D> where <Self as Layer<D>>::CType: Clone + Default;

    fn get_input_shape(&self) -> &Array<usize, D>;

    fn get_output_shape(&self) -> &Array<usize, D>;
}