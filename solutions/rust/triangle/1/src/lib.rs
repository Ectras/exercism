use std::ops::{Add, Neg};

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Add<Output = T> + Neg<Output = T> + PartialOrd + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;
        if a + b >= c && b + c >= a && a + c >= b && a > -a && b > -b && c > -c {
            Some(Triangle { a, b, c })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.a != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.b == self.c
    }
}
