pub fn sum_of_multiples(end: usize, multiples: &Vec<usize>) -> usize {
    let mut total = 0;
    for i in 1..end {
        if multiples.iter().any( |&x| i % x == 0 ) { total += i }
    }
    total
}
