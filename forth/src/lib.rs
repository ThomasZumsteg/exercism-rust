use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl  Forth {
    pub fn new() -> Forth {
        Forth { 
            stack: Vec::new(),
            words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> { self.stack.clone() }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let lower = input.to_lowercase();
        let mut tokens: Vec<String> = lower
            .split(|c:char| c.is_whitespace() || c.is_control())
            .map(|s| s.to_string())
            .collect();
        while !tokens.is_empty() {
            try!(match tokens.remove(0).as_ref() {
                t if self.words.contains_key(t) => {
                    for d in self.words.get(t).unwrap() {
                        tokens.insert(0, d.to_string());
                    };
                    Ok(())
                },
                ":" => self.add_def(&mut tokens),
                "+" => self.add(),
                "-" => self.subtract(),
                "/" => self.divide(),
                "*" => self.multiply(),
                "dup" => self.dup(),
                "drop" => self.drop(),
                "swap" => self.swap(),
                "over" => self.over(),
                t => self.to_value(t),
            });
        }
        Ok(())
    }

    fn add(&mut self) -> ForthResult {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(b + a);
            Ok(())
        } else { Err(Error::StackUnderflow) }
    }

    fn subtract(&mut self) -> ForthResult {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(b - a);
            Ok(())
        } else { Err(Error::StackUnderflow) }
    }

    fn divide(&mut self) -> ForthResult {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            if a == 0 { Err(Error::DivisionByZero) }
            else {
                self.stack.push(b / a);
                Ok(())
            }
        } else { Err(Error::StackUnderflow) }
    }

    fn multiply(&mut self) -> ForthResult {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(b * a);
            Ok(())
        } else { Err(Error::StackUnderflow) }
    }

    fn dup(&mut self) -> ForthResult {
        if let Some(v) = self.stack.pop() {
            self.stack.push(v);
            self.stack.push(v);
            Ok(())
        } else { Err(Error::StackUnderflow) }
    }

    fn drop(&mut self) -> ForthResult {
        if let Some(_) = self.stack.pop() { Ok(()) }
        else { Err(Error::StackUnderflow) }
    }

    fn swap(&mut self) -> ForthResult {
        let a = self.stack.pop();
        let b = self.stack.pop();
        if a.is_some() && b.is_some() { 
            self.stack.push(a.unwrap());
            self.stack.push(b.unwrap());
            Ok(())
        } else { Err(Error::StackUnderflow) }
    }

    fn over(&mut self) -> ForthResult {
        let a = self.stack.pop();
        let b = self.stack.pop();
        if a.is_some() && b.is_some() { 
            self.stack.push(b.unwrap());
            self.stack.push(a.unwrap());
            self.stack.push(b.unwrap());
            Ok(())
        } else { Err(Error::StackUnderflow) }
    }

    fn add_def(&mut self, tokens: &mut Vec<String>) -> ForthResult {
        if tokens.is_empty() { return Err(Error::InvalidWord) }
        let mut def = Vec::new();
        let word = tokens.remove(0);
        if word.parse::<Value>().is_ok() { return Err(Error::InvalidWord) }
        while !tokens.is_empty() {
            let token = tokens.remove(0);
            if token == ";" { 
                self.words.insert(word.to_string(), def);
                return Ok(())
            }
            def.insert(0, token.to_string());
        }
        Err(Error::InvalidWord)
    }

    fn to_value(&mut self, value: &str) -> ForthResult {
        if let Ok(v) = value.parse() { 
            self.stack.push(v);
            Ok(())
        } else { Err(Error::UnknownWord) }
    }
}

