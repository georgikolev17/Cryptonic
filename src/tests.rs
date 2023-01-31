#[cfg(test)]
mod test_matrix_instantiation {
    use crate::errors::MatrixError;
    use crate::matrix;
    use crate::layout::Layout;
    use crate::matrix::Matrix;

    #[test]
    fn test_row_major_gen() {
        let mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
        assert_eq!(mat.strides, vec![4, 1]);
    }

    #[test]
    fn test_col_major_gen() {
        let mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::ColumnMajor);
        assert_eq!(mat.strides, vec![1, 3]);
    }

    #[test]
    fn test_bound_check() {
        let mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::ColumnMajor);
        assert!(!mat.check_bounds(&vec![3, 4]));
        assert!(mat.check_bounds(&vec![2, 3]));
    }

    #[test]
    fn test_broadcasting(){
        match matrix::broadcast(&vec![256, 256, 3], Layout::RowMajor, &vec![3], Layout::ColumnMajor) {
            Ok((v1, v2, v3)) => {
                println!("{:?}", v1);
                println!("{:?}", v2);
                println!("{:?}", v3);
            },
            Err(E) => panic!("{}", E)
        }
    }

    // Write tests for everything cause it's buggy as fuck
//     #[test]
//     fn test_transpose(){
//         let mut mat: Matrix<i32> = Matrix::new(vec![2, 3], Layout::RowMajor);
//         mat.set(&vec![0,0], 6);
//         mat.set(&vec![0,1], 4);
//         mat.set(&vec![0,2], 24);
//         mat.set(&vec![1,0], 1);
//         mat.set(&vec![1,1], -9);
//         mat.set(&vec![1,2], 8);
//
//         match mat.get(&vec![0,0]) {
//             Ok(val) => println!("{}", val),
//             Err(err) => println!("{}", err)
//         }
//
//         mat.transpose();
//         println!("{:?}", mat.strides);
//         println!("{:?}", mat.shape);
// /*
//         match mat.get(&vec![0,0]) {
//             Ok(val) => println!("{}", val),
//             Err(err) => println!(err)
//         }*/
//     }

    #[test]
    fn test_get_physical_idx() {
        let mat: Matrix<i32> = Matrix::new(vec![4, 3, 7], Layout::RowMajor);

        let (x, y, z) = (4, 0, 0);
        // Expect error  to be thrown if index out of bounds
        let expected_error = Err(MatrixError::OutOfBounds);
        assert_eq!(expected_error, mat.get_physical_idx(&vec![x, y, z]));

        // The matrix [4, 3, 7] has stride (21, 7, 1)
        // Expect physical id of elemnt [1, 2, 3] to equal 1*21 + 2*7 + 3*1
        //
        let (x, y, z) = (1, 2, 3);
        assert_eq!(Ok(1*21 + 2*7 + 3*1), mat.get_physical_idx(&vec![x, y, z]));
    }
}

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