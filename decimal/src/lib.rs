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
        } else if chars.starts_with("+") { start = 1; }
        for (i, digit) in chars[start..].chars().rev().enumerate() {
            if digit == '.' { number.power = -(i as isize) }
            else if let Some(d) = digit.to_digit(BASE as u32) { 
                number.digits.push(d as Digit) 
            } else { return None }
        };
        Some(number.clean())
    }

    fn make_digits(this: &Decimal, other: &Decimal) -> Vec<(Digit, Digit)> { 
        let mut result = Vec::new();
        let (this_len, other_len) = (this.digits.len(), other.digits.len());
        for i in 0..max(this_len, other_len) {
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

            result.digits = iter::repeat(0).take(diff as usize)
                .chain(this.digits.clone()).collect();
            result.power = other.power;
            Decimal::make_digits(&result, other)
        } else {
            result.digits = iter::repeat(0).take(-diff as usize)
                .chain(other.digits.clone()).collect();
            result.power = this.power;
            Decimal::make_digits(this, &result)
        }
    }

    fn flip_sign(&self) -> Decimal {
        let mut result = self.clone();
        result.negative = !self.negative;
        result
    }

    fn clean(&self) -> Decimal {
        let mut result = self.clone();
        while let Some(&d) = result.digits.last() {
            if d != 0 { break; }
            result.digits.pop();
        }
        while let Some(&d) = result.digits.first() {
            if d != 0 { break }
            result.power += 1;
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
        else if self.negative { return other.sub(self.flip_sign()) }
        else if other.negative { return self.sub(other.flip_sign()) }

        let mut carry = 0;
        for (a, b) in Decimal::make_equal_digits(&self.clean(), &other.clean()) {
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
        if other.negative { return self.add(other.flip_sign()) }

        if self < other { return other.sub(self).flip_sign() }
        let mut result = Decimal::new();
        result.power = min(self.power, other.power);
        let mut carry = 0;
        for (a, b) in Decimal::make_equal_digits(&self.clean(), &other.clean()) {
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
        result.power = self.power + other.power;
        for (p, a) in self.clean().digits.iter().enumerate() {
            let mut step = Decimal::new();
            step.power = (p as isize) + self.power;
            let mut carry = 0;
            for b in &other.clean().digits {
                step.digits.push((a * b + carry) % BASE);
                carry = (a * b + carry) / BASE;
            }
            if carry > 0 { step.digits.push(carry); }
            result = step.clean().add(result);
        }
        result.negative = self.negative != other.negative;

        println!("{} * {} = {}", self, other, result);
        result.clean()
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        self.clean().partial_cmp(&other.clean()) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> { 
        if self.negative && !other.negative { return Some(Ordering::Less) }
        else if !self.negative && other.negative { return Some(Ordering::Greater) }

        for &(a, b) in Decimal::make_equal_digits(self, other).iter().rev() {
            if a != b { 
                if !self.negative { return a.partial_cmp(&b) }
                else { return b.partial_cmp(&a) }
            }
        }
        Some(Ordering::Equal)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num = self.digits.iter().rev()
            .map(|d| d.to_string()).collect::<Vec<String>>().join("");
        if self.negative {
            write!(f, "-{}x10^{}", num, self.power)
        } else {
            write!(f, "{}x10^{}", num, self.power)
        }
    }
}
