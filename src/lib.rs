extern crate core;

mod matrix;
mod errors;
mod layout;

#[cfg(test)]
mod test_matrix_instantiation {
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
}
