#[cfg(test)]
mod test_matrix_instantiation {
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
}