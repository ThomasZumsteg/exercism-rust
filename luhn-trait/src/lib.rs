pub trait Luhn { fn valid_luhn(&self) -> bool; }

impl<T> Luhn for T where T: ToString { 
    fn valid_luhn(&self) -> bool { 
        let (mut total, mut i) = (0, 0);
        for c in self.to_string().chars().rev() {
            if let Some(d) = c.to_digit(10) { 
                total += d * (((i as u32) % 2) + 1);
                if d > 4 && i % 2 == 1 { total -= 9 }
                i += 1;
            } else if c != ' ' { return false }
        }
        i > 1 && total % 10 == 0
    }
}
