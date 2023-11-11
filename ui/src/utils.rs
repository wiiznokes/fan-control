pub trait RemoveElem<T> {
    fn remove_elem<F>(&mut self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool;
}

impl<T> RemoveElem<T> for Vec<T> {
    fn remove_elem<F>(&mut self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.iter()
            .position(predicate)
            .map(|index| self.remove(index))
    }
}
