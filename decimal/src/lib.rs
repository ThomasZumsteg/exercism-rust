use std::ops::{Mul, Add, Sub};
use std::cmp::{Ordering, max};
use std::iter;

type Digit = u8;
const Base: usize = 10;

#[derive(PartialEq, Debug)]
pub struct Decimal { 
    negative: bool,
    power: usize,
    digits: Vec<Digit> 
}

impl Decimal {
    pub fn try_from(chars: &str) -> Option<Decimal> {
        let mut number = Decimal { digits: Vec::new(), negative: false, power: 0};
        let mut start = 0;
        if chars.starts_with("-") {
            start = 1;
            number.negative = true;
        }
        for (i, digit) in chars[start..].chars().rev().enumerate() {
            if digit == '.' { number.power = i }
            else if let Some(d) = digit.to_digit(Base as u32) { 
                number.digits.insert(0, d as Digit) 
            } else { return None }
        };
        Some(number)
    }

    fn zip_digits(&self, other: &Decimal) -> Vec<(Digit, Digit)> { 
        if self.power > other.power { other.zip_digits(&self) }
        else {
            let other_digits = other.digits.iter().cloned().chain(iter::repeat(0));
            self.digits.iter().cloned().zip(other_digits).collect()
        }
    }
}


impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal { 
        let mut result = Decimal { 
            digits: Vec::new(),
            power: max(self.power, other.power),
            negative: false
        };
        let mut carry = 0;
        for (&a, &b) in self.digits.iter().zip(other.digits.iter()).rev() {
            result.digits.push((a + b + carry) % (Base as u8));
            let carry = (a + b + carry) / (Base as u8);
        }
        result
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal { unimplemented!() } 
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Decimal) -> Decimal { unimplemented!() }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> { 
        for (a, b) in self.zip_digits(other) {
            if a != b { return a.partial_cmp(&b) }
        }
        Some(Ordering::Equal)
    }
}
