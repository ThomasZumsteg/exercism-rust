pub trait Luhn { 
    fn valid_luhn(&self) -> bool {
        if let Some(digits) = self.digits() {
            println!("{:?}", digits);
            let mut total = 0;
            for (i, d) in digits.iter().rev().enumerate() {
                total += d * (((i as u32) % 2) + 1);
                if d > &4u32 && i % 2 == 1 { total -= 9 }
            }
            digits.len() > 1 && total % 10 == 0
        } else { false }
    }

    fn digits(&self) -> Option<Vec<u32>>;
}


impl Luhn for String { 
    fn digits(&self) -> Option<Vec<u32>> { 
        let mut digits: Vec<u32> = Vec::new();
        for c in self.chars() {
            if c == ' ' { continue }
            else if let Some(d) = c.to_digit(10) { digits.push(d); }
            else { return None }
        };
        Some(digits)
    }
}
impl Luhn for &'static str { fn digits(&self) -> Option<Vec<u32>> { self.to_string().digits() } }
impl Luhn for u8 { fn digits(&self) -> Option<Vec<u32>> { self.to_string().digits() } }
impl Luhn for u16 { fn digits(&self) -> Option<Vec<u32>> { self.to_string().digits() } }
impl Luhn for u32 { fn digits(&self) -> Option<Vec<u32>> { self.to_string().digits() } }
impl Luhn for u64 { fn digits(&self) -> Option<Vec<u32>> { self.to_string().digits() } }
impl Luhn for usize { fn digits(&self) -> Option<Vec<u32>> { self.to_string().digits() } }
