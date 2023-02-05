use Cryptonic::layout::Layout;
use Cryptonic::matrix::{concat, Matrix};
fn main() {
    let mut mat1 = Matrix::from_iter(vec![2, 3], 0.., Layout::RowMajor);
    let mut mat2 = Matrix::from_iter(vec![2, 3], 1.., Layout::RowMajor);

    let (mat_concat, mat1, mat2) = concat(mat1, mat2, 0).unwrap();
    println!("{:?}", mat_concat);
}
