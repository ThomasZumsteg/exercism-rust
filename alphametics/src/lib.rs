use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::iter;

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    for map in Mappings::new(get_letters(puzzle), vec![0,1,2,3,4,5,6,7,8,9]) { 
        if let Some(equation) = Equation::translate(puzzle, &map) {
            println!("{:?}", equation);
            if equation.evaluate() { return Some(map) }
        }
    }
    None
}

fn get_letters(words: &str) -> Vec<char> {
    let mut letters = HashSet::new();
    for c in words.chars() {
        if c.is_alphabetic() && c.is_uppercase() { letters.insert(c); }
    }
    letters.iter().map(|&c| c).collect::<Vec<char>>()
}

struct Mappings<K, V> { 
    keys: Vec<K>,
    values: Vec<V>,
    idx: Vec<usize>,
} 

impl<K, V> Mappings<K, V> {
    fn new(keys: Vec<K>, values: Vec<V>) -> Mappings<K, V> {
        let idx = iter::repeat(0).take(keys.len()).collect();
        Mappings { keys: keys, values: values, idx: idx }
    }
}

impl<K: Eq + Hash + fmt::Display + Clone, V: Clone> Iterator for Mappings<K, V> {
    type Item = HashMap<K, V>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut values = self.values.clone();
        let mut result = HashMap::new();
        for (i, k) in self.keys.iter().enumerate() {
            if self.idx[i] >= values.len() {
                self.idx[i] = 0;
                if self.idx.len() <= i + 1 { return None }
                else { self.idx[i+1] += 1 }
            }
            result.insert(k.clone(), values.remove(self.idx[i]));
        }
        self.idx[0] += 1;
        Some(result)
    }
}

#[derive(Debug)]
struct Equation { values: Vec<String>, total: String }

impl Equation {
    fn translate(source: &str, map: &HashMap<char, u8>) -> Option<Equation> {
        let mut digits = source.to_string();
        for (k, v) in map.iter() { 
            digits = digits.replace(&k.to_string(), &v.to_string());
        }
        let sides = digits.split(" == ").collect::<Vec<&str>>();
        let values = sides[0].split(" + ");
        if values.clone().any(|v| v.starts_with("0")) || sides[1].starts_with("0") {
            return None
        } else {
            Some(Equation {
                values: values.map(|s| s.to_string()).collect::<Vec<String>>(),
                total: sides[1].to_string()
            })
        }
    }
    
    fn evaluate(&self) -> bool {
        let mut total = 0;
        for value in &self.values { total += value.parse().unwrap(); }
        total == self.total.parse().unwrap()
    }
}
