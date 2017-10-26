use std::ops::{Mul, Add, Sub};
use std::cmp::{Ordering, max, min};
use std::iter;

type Digit = u8;
const Base: usize = 10;

#[derive(PartialEq, Debug)]
pub struct Decimal { 
    negative: bool,
    power: isize,
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
            if digit == '.' { number.power = -(i as isize) }
            else if let Some(d) = digit.to_digit(Base as u32) { 
                number.digits.push(d as Digit) 
            } else { return None }
        };
        Some(number)
    }

    fn zip_digits(&self, other: &Decimal) -> Vec<(Digit, Digit)> { 
        if self.power < other.power { other.zip_digits(&self) }
        else {
            let other_digits = other.digits.iter().cloned().chain(iter::repeat(0));
            self.digits.iter().cloned().zip(other_digits).collect()
        }
    }

    fn abs(&self) -> Decimal {
        Decimal { power: self.power, digits: self.digits.clone(), negative: false }
    }

    fn new() -> Decimal { Decimal { digits: Vec::new(), power: 0, negative: false } }
}


impl Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal { 
        let mut result = Decimal::new();
        result.power = min(self.power, other.power);

        if self.negative && other.negative { result.negative = true; }
        else if self.negative { return other.sub(self.abs()) }
        else if other.negative { return self.sub(other.abs()) }

        let mut carry = 0;
        for (a, b) in self.zip_digits(&other) {
            result.digits.push((a + b + carry) % (Base as u8));
            let carry = (a + b + carry) / (Base as u8);
        }
        result
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal {
        let mut result = Decimal::new();
        result.power = max(self.power, other.power);
        let mut carry = 0;
        for (a, b) in self.zip_digits(&other) {
            if a >= b + carry {
                result.digits.push(a - b - carry);
                carry = 0;
            } else {
                result.digits.push(a + 10 - b - carry);
                carry = 1;
            }
        }
        result
    } 
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Decimal) -> Decimal {
        println!("{:?} * {:?}", self, other);
        let mut result = Decimal::new();
        result.digits = vec![0];
        result.power = self.power;
        for (p, a) in self.digits.iter().enumerate() {
            let mut step = Decimal::new();
            step.power = (p as isize) + self.power;
            let mut carry = 0;
            for b in &other.digits {
                step.digits.push((a * b + carry) % 10);
                carry = (a * b + carry) / 10;
            }
            if carry > 0 { step.digits.push(carry); }
            println!("{:?}:{:?}",result, step);
            result = step.add(result);
        }
        result
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> { 
        for (a, b) in self.zip_digits(other) {
            if a != b { return a.partial_cmp(&b) }
        }
        Some(Ordering::Equal)
    }
}
