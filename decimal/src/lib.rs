use std::ops::{Mul, Add, Sub};
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub struct Decimal { 
    power: Option<usize>,
    digits: Vec<u32> 
}

impl Decimal {
    pub fn try_from(chars: &str) -> Option<Decimal> {
        let mut digits = Vec::new();
        let mut power: Option<usize> = None;
        for (i, digit) in chars.chars().rev().enumerate() {
            if digit == '.' { power = Some(i) }
            else if let Some(d) = digit.to_digit(10) { digits.insert(0, d) }
            else { return None }
        };
        println!("{:?} ^ -{:?}", digits, power);
        Some(Decimal { power: power, digits: digits })
    }
}

impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal { unimplemented!() }
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
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> { unimplemented!() } 
}
