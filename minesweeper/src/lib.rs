pub fn annotate(board: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (i, row) in board.iter().enumerate() {
        result.push(String::new());
        for (j, c) in row.chars().enumerate() {
            println!("counting: [{}, {}]",i,j);
            match count_mines(board, i, j) {
                Some(0) => result[i].push(' '), 
                Some(n) => result[i] += &format!("{}", n),
                None => result[i].push(c),
            }
        }
    }
    result
}

fn count_mines(board: &[&str], i: usize, j: usize) -> Option<u8> {
    let mut count = 0;
    if &board[i][j..(j+1)] != " " { return None }
    for x in 0..3 {
        for y in 0..3 {
            if i+1 < x || i+1-x >= board.len() || 
               j+1 < y || j+1-y >= board[i].len() || 
               (x == 1 && y == 1)
               { 
                   continue;
               }
            else if &board[i+1-x][(j+1-y)..(j+2-y)] == "*" { count += 1 }
        }
    }
    Some(count)
}
