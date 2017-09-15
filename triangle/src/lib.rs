extern crate num;

use num::Num;

pub struct Triangle<N>
    where N: Num + PartialOrd + Copy, {
    a: N,
    b: N,
    c: N,
}

impl<N> Triangle<N>
    where N: Num + PartialOrd + Copy, {
    pub fn build(sides: [N; 3]) -> Result<Triangle<N>, &'static str> {
        let mut sorted_sides = sides.to_vec();
        sorted_sides
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        if sorted_sides[2] <= N::zero() { Err("Zero length sides are not allowed") }
        else if sorted_sides[2] > sorted_sides[1] + sorted_sides[0] {
            Err("Not a valid triangle")
        } else { 
            Ok(Triangle { a: sorted_sides[2], b: sorted_sides[1], c: sorted_sides[0]  })
        }
    }

    pub fn is_equilateral(&self) -> bool { self.a == self.b && self.b == self.c }

    pub fn is_scalene(&self) -> bool { !self.is_equilateral() && !self.is_isosceles() }

    pub fn is_isosceles(&self) -> bool { self.a == self.b || self.b == self.c }
}
