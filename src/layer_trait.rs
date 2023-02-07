use crate::matrix::Matrix;

pub trait Layer {
    type CType;

    fn forward(&mut self, inputs: Matrix<Self::CType>) -> Matrix<Self::CType> where <Self as Layer>::CType: Clone + Default;
}