struct Append<I, J> {
    a: I,
    b: J
}

impl<I, J> Iterator for Append<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        self.a.next().or_else(|| self.b.next())
    }
}

/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    Append {
        a,
        b,
    }
}

struct Concat<I>
where
    I: Iterator
{
    nested_iter: I,
    current: Option<I::Item>,
}

impl<I> Concat<I>
where
    I: Iterator
{
    fn new(nested_iter: I) -> Self {
        Self {
            nested_iter,
            current: None,
        }
    }
}

impl<I> Iterator for Concat<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    type Item = <I::Item as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = &mut self.current {
                if let Some(value) = current.next() {
                    return Some(value);
                }
            }
            self.current = Some(self.nested_iter.next()?);
        }
    }
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    Concat::new(nested_iter)
}

struct Filter<I, F> {
    iter: I,
    predicate: F,
}

impl<I, F> Iterator for Filter<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            if (self.predicate)(&x) {
                return Some(x);
            }
        }
        None
    }
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    Filter {
        iter,
        predicate,
    }
}

pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut count = 0;
    while let Some(_x) = iter.next() {
        count += 1;
    }
    count
}

struct Map<I, F> {
    iter: I,
    function: F
}

impl<I, F, U> Iterator for Map<I, F>
where
    I: Iterator,
    F: Fn(I::Item) -> U
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&self.function)
    }
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    Map {
        iter,
        function
    }
}

pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut value = initial;
    while let Some(v) = iter.next() {
        value = function(value, v);
    }
    value
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut value = initial;
    while let Some(v) = iter.next_back() {
        value = function(value, v);
    }
    value
}

struct Reverse<I> {
    iter: I,
}

impl<I> Iterator for Reverse<I>
where
    I: DoubleEndedIterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    Reverse {
        iter
    }
}
