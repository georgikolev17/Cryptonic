use Cryptonic::{matrix::Matrix, layout::Layout};

fn main() {
    println!("Hello, World!");
    let mut mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);
    println!("{:?}", mat.shape);
}