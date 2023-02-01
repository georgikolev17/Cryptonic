use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MatrixError {
    InvalidParams,
    SliceError,
    ViewError,
    BroadcastError,
    OpError,
    DimError,
    MatmulShapeError,
    ShapeError,
    OutOfBounds,
    ReshapeError
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatrixError::InvalidParams => write!(f, "Invalid parameters"),
            MatrixError::SliceError => write!(f, "Invalid slice for Matrix"),
            MatrixError::ViewError => write!(f, "Invalid view shape for Matrix"),
            MatrixError::BroadcastError => write!(f, "Shapes are not broadcastable"),
            MatrixError::OpError => write!(f, "Matrix cannot be operated on"),
            MatrixError::DimError => write!(f, "Matrix cannot be operated on over the given dimension"),
            MatrixError::MatmulShapeError => write!(
                f,
                "Matrix must have at least two dimensions and have same shape in all dims except the last dimension"
            ),
            MatrixError::ShapeError => write!(f, "Matrix must have the same shape in all dims except the last dimension"),
            MatrixError::OutOfBounds => write!(f, "Indices are out of bounds for the matrix"),
            MatrixError::ReshapeError => write!(f, "Matrix cannot be reshaped into given shape")
        }
    }
}

// This is important for other errors to wrap this one.
impl error::Error for MatrixError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}