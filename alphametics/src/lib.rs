use std::collections::HashMap;
use std::iter;

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let equation = Equation::parse(puzzle).unwrap();
    for e in equation { 
        println!("{:?}", e);
        if e.evaluate() { return Some(e.get_map()) } }
    None
}

#[derive(Debug, Clone)]
struct Equation { 
    left: Vec<String>,
    right: String,
    letters: Vec<(char, u8)>,
    idx: usize,
}

impl Equation {
    fn parse(source: &str) -> Option<Equation> {
        let sides = source.split(" == ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Some( Equation{
            left: sides[0].split(" + ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            right: sides[1].clone(),
            letters: Equation::letters(source).iter()
                .zip(iter::repeat(0))
                .map(|(&c, i)| (c, i as u8))
                .collect(),
            idx: 0,
        })
    }   

    fn get_map(&self) -> HashMap<char, u8> { 
        let mut map = HashMap::new();
        for &(c, i) in &self.letters { map.insert(c, i); }
        map
    }

    fn letters(sentence: &str) -> Vec<char> {
        let mut result = Vec::new();
        for c in sentence.chars() {
            if c.is_alphabetic() && !result.contains(&c) { result.push(c) }
        }
        result
    }
    fn evaluate(&self) -> bool{ 
        false
    }
}

impl Iterator for Equation {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let mut new = self.clone();
        while new.letters[new.idx].1 >= 9 {
            new.letters[new.idx].1 = 0;
            new.idx += 1;
            if new.idx > new.letters.len() { return None }
        }
        unimplemented!()
    }
}
