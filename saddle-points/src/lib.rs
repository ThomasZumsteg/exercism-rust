pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();
    for (r, row) in input.iter().enumerate() {
        for (c, &num) in row.iter().enumerate() {
            let h_slice = row.iter().map(|&d| d);
            let v_slice = input.iter().map(|row| row[c]);

            println!("{:?}, {:?}", &h_slice, &v_slice);
            if num == v_slice.min().unwrap() && num == h_slice.max().unwrap() {
                saddle_points.push((r, c));
            }
        }
    }
    saddle_points
}
