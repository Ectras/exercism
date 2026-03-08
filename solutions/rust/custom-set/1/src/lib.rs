#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T: Clone + Ord> CustomSet<T> {
    /// Constructs a new set. The time complexity is *O*(*n* \* log(*n*)).
    pub fn new(input: &[T]) -> Self {
        let mut items = input.to_vec();
        items.sort_unstable();
        items.dedup();
        Self { items }
    }

    /// Checks whether an element is contained in the set. The time complexity is
    /// *O*(log(*n*)).
    pub fn contains(&self, element: &T) -> bool {
        self.items.binary_search(&element).is_ok()
    }

    /// Adds an element to the set. The time complexity is *O*(*n*).
    pub fn add(&mut self, element: T) {
        let find_result = self.items.binary_search(&element);
        if let Err(insert_pos) = find_result {
            self.items.insert(insert_pos, element);
        }
    }

    /// Checks wether `self` is a subset of `other`, i.e., whether all elements of
    /// `self` are contained in `other`. The time complexity is *O*(*n* \* log(*n*)).
    pub fn is_subset(&self, other: &Self) -> bool {
        self.difference(other).is_empty()
    }

    /// Checks wether the set is empty. The time complexity is *O*(1).
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Checks wether `self`and `other` are disjoint, i.e., whether there are no
    /// common elements in the two sets. The time complexity is *O*(*n* \* log(*n*)).
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    /// Computes the intersection between `self` and `other`, i.e., the resulting set
    /// contains all elements that are in both `self` and `other`. The time
    /// complexity is *O*(*n* \* log(*n*)).
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for item in &self.items {
            if other.contains(&item) {
                result.push(item.clone());
            }
        }
        Self { items: result }
    }

    /// Computes the set difference between `self` and `other`, i.e., the resulting
    /// set contains all elements that are in `self`, but not in `other`. The time
    /// complexity is *O*(*n* \* log(*n*)).
    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for item in &self.items {
            if !other.contains(&item) {
                result.push(item.clone());
            }
        }
        Self { items: result }
    }

    /// Computes the union of `self` and `other`, i.e., the resulting set contains
    /// all elements that are in `self` or `other` (without duplicates). The time
    /// complexity is *O*(*n*).
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut left = self.items.iter().peekable();
        let mut right = other.items.iter().peekable();
        let mut result = Vec::new();

        // Merge the two sorted vecs in linear time
        loop {
            let next_element = match (left.peek(), right.peek()) {
                (None, None) => break,
                (None, Some(_)) => right.next(),
                (Some(_), None) => left.next(),
                (Some(x), Some(y)) => {
                    if x < y {
                        left.next()
                    } else {
                        right.next()
                    }
                }
            }
            .unwrap()
            .clone();

            // Since the merged sequence is sorted, we can directly check for
            // duplicates
            let is_duplicate = result
                .last()
                .map(|last| *last == next_element)
                .unwrap_or_default();

            if !is_duplicate {
                result.push(next_element);
            }
        }

        Self { items: result }
    }
}
