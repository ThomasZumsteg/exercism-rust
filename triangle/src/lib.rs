pub struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    pub fn build(sides: [usize; 3]) -> Result<Triangle, &'static str> {
        let mut sorted_sides = sides.to_vec();
        sorted_sides.sort();    
        if sorted_sides[2] <= 0 { Err("Zero length sides are not allowed") }
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
