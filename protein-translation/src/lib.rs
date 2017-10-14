use std::collections::HashMap;

type Pairs = Vec<(&'static str, &'static str)>;
pub struct Protines { pairs: HashMap<&'static str, &'static str>, }

pub fn parse(pairs: Pairs) -> Protines { 
    let mut proteins = Protines { pairs: HashMap::new() };
    for (coden, name) in pairs { proteins.pairs.insert(coden, name); }
    proteins
}

impl Protines {
    pub fn name_for(&self, coden: &str) -> Result<&'static str, &'static str> {
        if let Some(result) = self.pairs.get(coden) { Ok(result) }
        else { Err("Not a valid coden") }
    }

    pub fn of_rna(&self, rna: &str) -> Result<Vec<&'static str>, &'static str> {
        let mut result = Vec::new();
        let mut i = 0;
        for j in 0..(rna.len() + 1) {
            if let Ok(name) = self.name_for(&rna[i..j]) {
                if name == "stop codon" { return Ok(result) }
                result.push(name);
                i = j;
            }
        }
        if i == rna.len() { Ok(result) }
        else { Err("Nope") }
    }
}

