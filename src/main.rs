use Cryptonic::layout::Layout;
use Cryptonic::matrix::{add, concat, Matrix, MatrixIter, subtract};
fn main() {
    let mut mat1 = Matrix::from_iter(vec![2, 3], 0.., Layout::RowMajor);
    let mut mat2 = Matrix::from_iter(vec![2, 3], 1.., Layout::RowMajor);

    let (mat_concat, mat1, mat2) = subtract(mat1, mat2).unwrap();
    println!("{mat_concat:?}");
    // let mut matrix_iter = MatrixIter {
    //     mat: &mat_concat,
    //     index: vec![0; mat_concat.shape().len()],
    //     current_el: None,
    //     empty: false,
    // };
    // for (element, idx) in matrix_iter {
    //     println!("{idx:?} -> {element}")
    // }
}
