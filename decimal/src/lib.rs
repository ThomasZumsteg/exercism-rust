use std::ops::{Mul, Add, Sub};
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub struct Decimal { 
    power: usize,
    digits: Vec<u32> 
}

impl Decimal {
    pub fn try_from(chars: &str) -> Option<Decimal> {
        let mut digits = Vec::new();
        let mut power: usize = 0;
        for (i, digit) in chars.chars().rev().enumerate() {
            if digit == '.' { power = i }
            else if let Some(d) = digit.to_digit(10) { digits.insert(0, d) }
            else { return None }
        };
        println!("{:?} ^ -{:?}", digits, power);
        Some(Decimal { power: power, digits: digits })
    }
}

impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal { 
        if self.power < other.power { return other.add(self) }

        let mut digits: Vec<_> = Vec::new();
        let mut carry = 0;
        for (a, b) in self.digits.iter().zip(other.digits.iter()) {
            digits.push((a + b + carry) % 10);
            carry = (a + b + carry) / 10;
        }
        if carry != 0 { digits.push(carry) }
        Decimal{ digits: digits, power: self.power }
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal { 
        let mut digits: Vec<_> = Vec::new();
        let mut carry = 0;
        for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            digits.insert(0, (a - b - carry) % 10);
            carry = (a - b - carry) / 10;
        }
        Decimal { digits: digits, power: self.power }
    } 
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
