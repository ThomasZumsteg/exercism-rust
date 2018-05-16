use std::collections::HashSet;

pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut products = HashSet::new();
    for i in min..(max+1) {
        for j in i..(max+1) {
            let product = i * j;
            let digits = format!("{}", product);
            if digits.chars().into_iter().eq(digits.chars().into_iter().rev()) {
                products.insert(product);
            }
        }
    }
    products.iter().map(|&p| p).collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.into_iter().fold(None, |min, &p| match min {
        None => Some(p),
        Some(q) => Some(if p < q { p } else { q })
    })
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.into_iter().fold(None, |min, &p| match min {
        None => Some(p),
        Some(q) => Some(if p < q { q } else { p })
    })
}
