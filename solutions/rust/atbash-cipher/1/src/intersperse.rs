use std::iter::Peekable;

pub struct IntersperseNIterator<T>
where
    T: Iterator,
    T::Item: Clone,
{
    iter: Peekable<T>,
    group_size: usize,
    separator: T::Item,
    current_group: usize,
}

impl<T> IntersperseNIterator<T>
where
    T: Iterator,
    T::Item: Clone,
{
    fn new(iter: T, group_size: usize, separator: T::Item) -> Self {
        assert!(group_size >= 1);
        Self {
            iter: iter.peekable(),
            group_size,
            separator,
            current_group: group_size,
        }
    }
}

impl<T> Iterator for IntersperseNIterator<T>
where
    T: Iterator,
    T::Item: Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Return early if there's no next element
        self.iter.peek()?;

        // See if we have to return a separator or the next item
        if self.current_group == 0 {
            self.current_group = self.group_size;
            Some(self.separator.clone())
        } else {
            self.current_group -= 1;
            Some(self.iter.next().unwrap())
        }
    }
}

pub trait IntersperseN: Iterator {
    /// An iterator adaptor to insert a particular value after `group_size` elements
    /// of the adapted iterator if there are more elements in the adapted iterator.
    /// Iterator element type is `Self::Item`.
    fn intersperse_n(self, group_size: usize, separator: Self::Item) -> IntersperseNIterator<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        IntersperseNIterator::new(self, group_size, separator)
    }
}

impl<T> IntersperseN for T where T: Iterator + ?Sized {}