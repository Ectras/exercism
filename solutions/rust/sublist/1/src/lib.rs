#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

impl Comparison {
    fn flip(&self) -> Comparison {
        match self {
            Comparison::Sublist => Comparison::Superlist,
            Comparison::Superlist => Comparison::Sublist,
            _ => *self,
        }
    }
}

fn compare_same<T: PartialEq>(x: &[T], y: &[T]) -> Comparison {
    if x == y {
        Comparison::Equal
    } else {
        Comparison::Unequal
    }
}

fn check_sublist<T: PartialEq>(shorter_list: &[T], longer_list: &[T]) -> Comparison {
    'outer: for i in 0..=longer_list.len() - shorter_list.len() {
        for j in 0..shorter_list.len() {
            if shorter_list[j] != longer_list[i + j] {
                continue 'outer;
            }
        }
        return Comparison::Sublist;
    }
    Comparison::Unequal
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        ([], []) => Comparison::Equal,
        ([], _) => Comparison::Sublist,
        (_, []) => Comparison::Superlist,
        (x, y) if x.len() == y.len() => compare_same(x, y),
        (x, y) if x.len() > y.len() => check_sublist(y, x).flip(),
        (x, y) if x.len() < y.len() => check_sublist(x, y),
        _ => unreachable!(),
    }
}
