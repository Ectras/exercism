type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(node) => {
                let mut counter = 1;
                let mut node = node;
                while let Some(next) = &node.next {
                    counter += 1;
                    node = next;
                }
                counter
            }
        }
    }

    pub fn push(&mut self, element: T) {
        let new = Box::new(Node {
            element,
            next: self.head.take(),
        });

        self.head = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.element)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        match &self.head {
            None => Self::new(),
            Some(node) => {
                let mut out = SimpleLinkedList::new();
                let mut node = node;
                out.push(node.element.clone());
                while let Some(next) = &node.next {
                    out.push(next.element.clone());
                    node = next;
                }
                out
            }
        }
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut out = SimpleLinkedList::new();
        for item in iter {
            out.push(item);
        }
        out
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut out = Vec::new();
        while let Some(item) = linked_list.pop() {
            out.push(item);
        }
        out.reverse();
        out
    }
}
