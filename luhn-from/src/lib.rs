pub struct Luhn { digits: Option<Vec<u32>> }

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.digits.is_none() { return false }
        let digits = self.digits.clone().unwrap();
        let mut total = 0;
        for (i, d) in digits.iter().rev().enumerate() {
            total += d * (((i as u32) % 2) + 1);
            if d > &4u32 && i % 2 == 1 { total -= 9 }
        }
        digits.len() > 1 && total % 10 == 0
    }
}

impl From<String> for Luhn {
    fn from(characters: String) -> Luhn {
        let mut digits: Vec<u32> = Vec::new();
        for c in characters.chars() {
            if c == ' ' { continue }
            else if let Some(d) = c.to_digit(10) { digits.push(d); }
            else { return Luhn { digits: None }; }
        };
        Luhn { digits: Some(digits) }
    }
}

impl From<&'static str> for Luhn { 
    fn from(characters: &str) -> Luhn { Luhn::from(characters.to_string()) }
}

impl From<u8> for Luhn { 
    fn from(code: u8) -> Luhn { Luhn::from(code.to_string()) }
}

impl From<u16> for Luhn { 
    fn from(code: u16) -> Luhn { Luhn::from(code.to_string()) }
}

impl From<u32> for Luhn { 
    fn from(code: u32) -> Luhn { Luhn::from(code.to_string()) }
}

impl From<u64> for Luhn { 
    fn from(code: u64) -> Luhn { Luhn::from(code.to_string()) }
}

impl From<usize> for Luhn { 
    fn from(code: usize) -> Luhn { Luhn::from(code.to_string()) }
}
