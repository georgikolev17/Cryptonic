// We assume T::default() is the 0 of the type T
pub fn relu_usize<T>(_in: T) -> T where T: PartialOrd + Default{
    if _in < T::default() {
        return T::default();
    }
    _in
}