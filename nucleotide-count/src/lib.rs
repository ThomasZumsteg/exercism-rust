use std::collections::HashMap;
use std::iter::FromIterator;
use std::collections::hash_map::Entry;

pub fn count(nuc: char, dna: &str) -> Result<usize, &str> {
    match try!(nucleotide_counts(dna)).get(&nuc) {
        None => Err("Not a valid nucliotide to search for"),
        Some(v) => Ok(*v)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, &str> {
    let mut result = HashMap::from_iter(vec![('G', 0), ('T', 0), ('A', 0), ('C', 0)]);
    for c in dna.chars() {
        match result.entry(c) { 
            Entry::Vacant(_)=> return Err("Invalid Nucleotide in DNA"), 
            Entry::Occupied(o) => *o.into_mut() += 1,
        }
    }
    Ok(result)
}
