use std::ops::{Mul, Add, Sub};
use std::cmp::Ordering;

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
        let mut digits: Vec<Digit> = Vec::new();
        let mut power: usize = 0;
        for (i, digit) in chars.chars().rev().enumerate() {
            if digit == '.' { power = i }
            else if let Some(d) = digit.to_digit(Base as u32) { 
                digits.insert(0, d as Digit) 
            } else { return None }
        };
        Some(Decimal { negative: false, power: power, digits: digits })
    }

    fn digits(&self, power: usize) -> Option<Vec<Digit>> { None }
}


impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal { 
        let mut digits: Vec<Digit> = Vec::new();
        let mut carry = 0;
        for (&a, &b) in self.digits.iter().zip(other.digits.iter()).rev() {
            digits.push((a + b + carry) % (Base as u8));
            let carry = (a + b + carry) / (Base as u8);
        }
        Decimal{ negative: false, digits: digits, power: self.power }
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
        if self.power != other.power { 
            return self.power.partial_cmp(&other.power)
        } else if self.digits.len() != other.digits.len() {
            return self.digits.len().partial_cmp(&other.digits.len())
        }
        for (a, b) in self.digits.iter().zip(other.digits.iter()) {
            if a != b { return a.partial_cmp(b) }
        }
        Some(Ordering::Equal)
    }
}
