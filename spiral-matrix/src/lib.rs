pub fn spiral_matrix(size: usize) -> Vec<Vec<usize>> {
    if size == 0 { return vec![] }
    if size == 1 { return vec![vec![1]] }

    let mut matrix = vec![vec![0; size]; size];
    let (mut row, mut col): (i8, i8) = (0, 0);
    let (mut dr, mut dc): (i8, i8) = (0, 1);

    let mut i = 0;
    while matrix[row as usize][col as usize] == 0 {
        i += 1;
        matrix[row as usize][col as usize] = i;
        if !(0 <= row + dr && row + dr < (size as i8)) || 
           !(0 <= col + dc && col + dc < (size as i8)) ||
           matrix[(row + dr) as usize][(col + dc) as usize] != 0 {
            let temp = dc;
            dc = -dr;
            dr = temp;
        }
        row += dr;
        col += dc;
    }
    matrix
}
