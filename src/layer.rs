use crate::layer_trait::Layer;
use crate::matrix::Matrix;

pub struct layer {

}
impl Layer for layer {
    type CType = ();

    fn forward(&mut self, inputs: Matrix<Self::CType>) -> Matrix<Self::CType> where <Self as Layer>::CType: Clone + Default {
        todo!()
    }
}
