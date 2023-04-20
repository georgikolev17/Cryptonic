use ndarray::array;

fn main() {
    let arr1 = array![2, 2, 2];
    let arr2 = array![[3, 3, 3], [3, 3, 3], [3, 3, 3]];
    let result = arr2.dot(&arr1);
    println!("{}", result);
}