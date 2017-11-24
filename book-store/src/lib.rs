use std::collections::HashSet;

pub fn lowest_price(books: &[usize]) -> f64 {
    combinations(books.to_vec())
        .iter()
        .map(price).min_by(float_min)
        .unwrap_or(0.0)
}

fn float_min(a: &f64, b: &f64) -> std::cmp::Ordering {
    if let Some(c) = a.partial_cmp(b) { c }
    else { panic!() }
}

fn combinations<T>(items: Vec<T>) -> Vec<Vec<HashSet<T>>> 
    where
        T: Eq + std::hash::Hash + Clone + std::fmt::Debug
{
    let mut results: Vec<Vec<HashSet<T>>> = vec![vec![]];
    for item in &items {
        let mut next_results = Vec::new();
        for result in &results {
            for (s, set) in result.iter().enumerate() {
                if !set.contains(item) {
                    let mut iteration = result.clone();
                    iteration[s].insert(item.clone());
                    next_results.push(iteration)
                }
            }
            let mut iteration = result.clone();
            let mut new_set = HashSet::new();
            new_set.insert(item.clone());
            iteration.push(new_set);
            next_results.push(iteration);
        }
        results = next_results;
    }
    
    results
}

fn price(sets: &Vec<HashSet<usize>>) -> f64 {
    let mut total = 0.0;
    for set in sets {
        total += match set.len() {
            0 => 0.0,
            1 => 8.0,
            2 => 8.0 * 2.0 * 0.95,
            3 => 8.0 * 3.0 * 0.90,
            4 => 8.0 * 4.0 * 0.80,
            5 => 8.0 * 5.0 * 0.75,
            _ => panic!(),
        }
    }
    total
}

