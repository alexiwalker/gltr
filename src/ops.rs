pub trait VecExt<T> {
    fn push_if_no_match<F>(&mut self, item: T, predicate: F)
    where
        F: Fn(&T) -> bool;
}
   // Implement the trait for Vec<T>
impl<T> VecExt<T> for Vec<T> {
    fn push_if_no_match<F>(&mut self, item: T, predicate: F)
    where
        F: Fn(&T) -> bool,
    {
        if !self.iter().any(predicate) {
            self.push(item);
        }
    }
}