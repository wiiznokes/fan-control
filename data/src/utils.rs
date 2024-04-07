use std::cmp::Ordering;

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

#[cfg(test)]
pub fn init_test_logging() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

pub trait InsertSorted<T> {
    fn insert_sorted<F>(&mut self, predicate: F, element: T) -> Option<T>
    where
        F: Fn(&T) -> Ordering;
}

impl<T> InsertSorted<T> for Vec<T> {
    fn insert_sorted<F>(&mut self, predicate: F, element: T) -> Option<T>
    where
        F: Fn(&T) -> Ordering,
    {
        match self.binary_search_by(predicate) {
            Ok(index) => {
                let removed = std::mem::replace(&mut self[index], element);
                Some(removed)
            }
            Err(index) => {
                self.insert(index, element);
                None
            }
        }
    }
}
