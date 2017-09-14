#[derive(Debug)]
#[derive(PartialEq)]
pub struct RibonucleicAcid {
    rna: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct DeoxyribonucleicAcid {
    dna: String
}

impl RibonucleicAcid {
    pub fn new(rna: &str) -> RibonucleicAcid {
        RibonucleicAcid { rna: rna.to_string() }
    }

    pub fn to_dna(&self) -> Result<DeoxyribonucleicAcid, &str> {
        let mut result = String::new();
        for c in self.rna.chars() {
            match c {
                'C' => result.push('G'),
                'G' => result.push('C'),
                'U' => result.push('A'),
                'A' => result.push('T'),
                _ => return Err("Not a valud nucleiotide"),
            }
        }
        Ok(DeoxyribonucleicAcid::new(&result))
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(dna: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { dna: dna.to_string() }
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, &str> {
        let mut result = String::new();
        for c in self.dna.chars() {
            match c {
                'C' => result.push('G'),
                'G' => result.push('C'),
                'T' => result.push('A'),
                'A' => result.push('U'),
                _ => return Err("Not a valud nucleiotide"),
            }
        }
        Ok(RibonucleicAcid::new(&result))
    }
}
