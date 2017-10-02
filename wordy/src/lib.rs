pub struct WordProblem { phrase: String }

// start ::= "What is";
// end ::= "?";
// operator ::= "plus" | "minus" | "multiplied by" | "divided by"
// number ::= digit+ | "-" digit+
// operation ::= operator number
// question ::= start number operation+ end


impl WordProblem {

    pub fn new(phrase: &str) -> WordProblem { 
        WordProblem { phrase: phrase.to_string() }
    }

    pub fn answer(&self) -> Result<isize, &'static str> {
        let mut equation = self.phrase.clone();
        let start = equation.find("What is ");
        let end = equation.find("?");
        if start.is_none() || start.unwrap() != 0 || 
            end.is_none() || end.unwrap() != equation.len() - 1 {
            return Err("Nope");
        }

        equation = equation.replace("multiplied by", "multiplied_by");
        equation = equation.replace("divided by", "divided_by");
        let mut tokens = equation[8..(equation.len() - 1)].split(" ")
            .collect::<Vec<&str>>();
        if tokens.len() < 3 || tokens.len() % 2 != 1 {
            return Err("Wrong number of tokens");
        }

        let mut result = tokens.remove(0).parse::<isize>().unwrap();
        while tokens.len() >= 2 {
            let operator = tokens.remove(0);
            let value = tokens.remove(0);
            result = match operator {
                "plus" => result + value.parse::<isize>().unwrap(),
                "minus" => result - value.parse::<isize>().unwrap(),
                "multiplied_by" => result * value.parse::<isize>().unwrap(),
                "divided_by" => result / value.parse::<isize>().unwrap(),
                _ => return Err("Nope")
            };
        }
        Ok(result)
    }
}
