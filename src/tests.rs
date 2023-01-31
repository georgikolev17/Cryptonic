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

    // Write tests for everything cause it's buggy as fuck
    #[test]
    fn test_transpose(){
        let mut mat: Matrix<i32> = Matrix::new(vec![2, 3], Layout::RowMajor);
        mat.set(&vec![0,0], 6);
        mat.set(&vec![0,1], 4);
        mat.set(&vec![0,2], 24);
        mat.set(&vec![1,0], 1);
        mat.set(&vec![1,1], -9);
        mat.set(&vec![1,2], 8);

        match mat.get(&vec![0,0]) {
            Ok(val) => println!("{}", val),
            Err(err) => println!("{}", err)
        }

        mat.transpose();
        println!("{:?}", mat.strides);
        println!("{:?}", mat.shape);
/*
        match mat.get(&vec![0,0]) {
            Ok(val) => println!("{}", val),
            Err(err) => println!(err)
        }*/
    }
}