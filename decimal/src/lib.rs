use std::ops::{Mul, Add, Sub};
use std::cmp::{Ordering, max, min};
use std::fmt;
use std::iter;

type Digit = u8;
const BASE: Digit = 10;

#[derive(Debug, Clone)]
pub struct Decimal { 
    negative: bool,
    power: isize,
    digits: Vec<Digit> 
}

impl Decimal {
    pub fn try_from(chars: &str) -> Option<Decimal> {
        let mut number = Decimal::new();
        let mut start = 0;
        if chars.starts_with("-") {
            start = 1;
            number.negative = true;
        }
        for (i, digit) in chars[start..].chars().rev().enumerate() {
            if digit == '.' { number.power = -(i as isize) }
            else if let Some(d) = digit.to_digit(BASE as u32) { 
                number.digits.insert(0, d as Digit) 
            } else { return None }
        };
        Some(number.clean())
    }

    fn make_digits(this: &Decimal, other: &Decimal) -> Vec<(Digit, Digit)> { 
        let mut result = Vec::new();
        for i in 0..max(this.digits.len(), other.digits.len()) {
            // println!("{:?}:{:?}", this.digits.get(i), other.digits.get(i));
            result.push((
                    *this.digits.get(i).unwrap_or(&0),
                    *other.digits.get(i).unwrap_or(&0)));
        }
        result
    }

    fn make_equal_digits(this: &Decimal, other: &Decimal) -> Vec<(Digit, Digit)> {
        let mut result = Decimal::new();
        let diff = this.power - other.power;
        if diff > 0 {
            result.digits = other.digits.clone();
            result.digits.extend(iter::repeat(0).take((diff-1) as usize));
            result.power = this.power;
            Decimal::make_digits(this, &result)
        } else {
            result.digits = this.digits.clone();
            result.digits.extend(iter::repeat(0).take((1-diff) as usize));
            result.power = other.power;
            // println!("{:?} : {:?}", result.digits, other.digits);
            Decimal::make_digits(&result, other)
        }
    }

    fn abs(&self) -> Decimal {
        let mut result = self.clone();
        result.negative = false;
        result
    }

    fn clean(&self) -> Decimal {
        let mut result = self.clone();
        while let Some(&d) = result.digits.last() {
            if d != 0 { break; }
            result.power += 1;
            result.digits.pop();
        }
        while let Some(&d) = result.digits.first() {
            if d != 0 { break }
            result.digits.remove(0);
        }
        if result.digits == [] { 
            result.digits = vec![0];
            result.power = 0;
            result.negative = false;
        }
        result
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
        for (a, b) in Decimal::make_equal_digits(&self, &other) {
            result.digits.push((a + b + carry) % BASE);
            carry = (a + b + carry) / BASE;
        }
        if carry != 0 { result.digits.insert(0, carry) }
        println!("{} + {} = {}", self, other, result);
        result.clean()
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal {
        let mut result = Decimal::new();
        result.power = min(self.power, other.power);
        let mut carry = 0;
        for (a, b) in Decimal::make_equal_digits(&self, &other) {
            if a >= b + carry {
                result.digits.push(a - b - carry);
                carry = 0;
            } else {
                result.digits.push(a + BASE - b - carry);
                carry = 1;
            }
        }
        println!("{} - {} = {}", self, other, result);
        result.clean()
    } 
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Decimal) -> Decimal {
        let mut result = Decimal::new();
        result.digits = vec![0];
        result.power = self.power;
        for (p, a) in self.digits.iter().enumerate() {
            let mut step = Decimal::new();
            step.power = (p as isize) + self.power;
            let mut carry = 0;
            for b in &other.digits {
                step.digits.insert(0, (a * b + carry) % BASE);
                carry = (a * b + carry) / BASE;
            }
            if carry > 0 { step.digits.push(carry); }
            result = step.add(result);
        }
        println!("{} * {} = {}", self, other, result);
        result.clean()
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        // println!("{}:{}", self, other);
        self.clean().partial_cmp(&other.clean()) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> { 
        // println!("{}:{}", self, other);
        for &(a, b) in Decimal::make_equal_digits(self, other).iter().rev() {
            // println!("{}:{}", a, b);
            if a != b { return a.partial_cmp(&b) }
        }
        Some(Ordering::Equal)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num = self.digits.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("");
        if self.negative {
            write!(f, "-{}x10^{}", num, self.power)
        } else {
            write!(f, "{}x10^{}", num, self.power)
        }
    }
}
