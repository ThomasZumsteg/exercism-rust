pub fn count(lines: &Vec<&str>) -> usize {
    let mut rects = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            rects += count_rects(i,j,&lines);
        }
    }
    rects
}

fn count_rects(i: usize, j: usize, lines: &Vec<&str>) -> usize {
    if &lines[i][j..j+1] != "+" { return 0 }
    let mut count = 0;
    for m in (i+1)..lines.len() {
        for n in (j+1)..lines[m].len() {
            println!("({},{}),({},{})", i, j, m, n);
            if is_rect(i,j,m,n,&lines) {
                count += 1;
            }
        }
    }
    count
}

fn is_rect(i: usize, j: usize, m: usize, n: usize, lines: &Vec<&str>) -> bool {
    if &lines[i][j..j+1] != "+" || &lines[i][n..n+1] != "+" ||
       &lines[m][j..j+1] != "+" || &lines[m][n..n+1] != "+" {
       return false
    }
    for v in i..m {
        if !(&lines[v][j..j+1] == "|" || &lines[v][j..j+1] == "+") ||
           !(&lines[v][n..n+1] == "|" || &lines[v][n..n+1] == "+") {
               return false;
       }
    }
    for h in j..n {
        if !(&lines[i][h..h+1] == "-" || &lines[i][h..h+1] == "+") ||
           !(&lines[m][h..h+1] == "-" || &lines[m][h..h+1] == "+") {
               return false;
       }
    }
    return true
}
