use std::{cmp::Eq, collections::HashSet, hash::Hash};

trait IterExt<T> {
    fn collect_to_set(&mut self) -> HashSet<T>;
}

impl<T, I> IterExt<T> for I
where
    I: Iterator<Item = T>,
    T: Eq + Hash + Copy,
{
    fn collect_to_set(&mut self) -> HashSet<T> {
        self.collect()
    }
}

pub fn to_set<T: Eq + Hash + Copy>(iterable: impl IntoIterator<Item = T>) -> HashSet<T> {
    iterable.into_iter().collect_to_set()
}
