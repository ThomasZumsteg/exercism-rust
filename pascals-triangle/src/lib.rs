pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = PascalsTriangle { rows: Vec::new() };
        for i in 0..row_count {
            let mut r = Vec::new();
            for j in 0..(i+1) { r.push(value(i, j)) }
            triangle.rows.push(r);
        }
        triangle
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.to_vec()
    }
}

fn value(row: u32, col: u32) -> u32 {
    mult(col, row) / mult(1, row - col)
}

fn mult(start: u32, stop: u32) -> u32 {
    (start..stop).fold(1, |total, n| total * (n+1))
}
