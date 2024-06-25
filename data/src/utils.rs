use std::cmp::Ordering;

use std::collections::BTreeSet;

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

pub fn has_duplicate<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Ord,
{
    let mut uniq = BTreeSet::new();
    !iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn is_sorted<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Ord,
{
    let mut iter = iter.into_iter();

    let mut prev = iter.next();

    for current in iter {
        if let Some(ref prev) = prev {
            if prev > &current {
                return false;
            }
        }
        prev.replace(current);
    }

    true
}

#[cfg(test)]
pub fn init_test_logging() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

pub trait InsertSorted<T> {
    /// Don't allow duplicate
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
