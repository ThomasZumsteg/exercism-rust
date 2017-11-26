pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq)]
pub struct Forth {
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: Vec::new() }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        for token in input.split(|c:char| c.is_whitespace() || c.is_control()) {
            try!(match token {
                "+" => self.add(),
                "-" => self.subtract(),
                "/" => self.divide(),
                "*" => self.multiply(),
                t if t.to_lowercase() == "dup" => self.dup(),
                t if t.to_lowercase() == "drop" => self.drop(),
                t if t.to_lowercase() == "swap" => self.swap(),
                t if t.to_lowercase() == "over" => self.over(),
                t => self.to_value(t),
            });
        }
        Ok(())
    }

    fn add(&mut self) -> ForthResult {
        let a = self.stack.pop();
        let b = self.stack.pop();
        if a.is_some() && b.is_some() { 
            self.stack.push(b.unwrap() + a.unwrap());
            Ok(())
        } else { Err(Error::StackUnderflow) }
    }

    fn subtract(&mut self) -> ForthResult {
        let a = self.stack.pop();
        let b = self.stack.pop();
        if a.is_some() && b.is_some() { 
            self.stack.push(b.unwrap() - a.unwrap());
            Ok(())
        } else { Err(Error::StackUnderflow) }
    }

    fn divide(&mut self) -> ForthResult {
        let a = self.stack.pop();
        let b = self.stack.pop();
        if a.is_none() || b.is_none() { Err(Error::StackUnderflow) }
        else if a.unwrap() == 0 { Err(Error::DivisionByZero) }
        else { 
            self.stack.push(b.unwrap() / a.unwrap());
            Ok(())
        }
    }

    fn multiply(&mut self) -> ForthResult {
        let a = self.stack.pop();
        let b = self.stack.pop();
        if a.is_some() && b.is_some() { 
            self.stack.push(b.unwrap() * a.unwrap());
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

    fn to_value(&mut self, value: &str) -> ForthResult {
        if let Ok(v) = value.parse() { 
            self.stack.push(v);
            Ok(())
        } else { Err(Error::UnknownWord) }
    }
}

