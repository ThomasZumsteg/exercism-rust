pub struct Roman { digits: String }

impl Roman {
    pub fn from(mut n: usize) -> Roman {
        let mut result = String::new();
        for (value, roman) in Roman::digits() {
            println!("{}x{}={}", roman, n / value, roman.repeat(n / value));
            result = result + &roman.repeat(n / value);
            n = n % value;
        }
        Roman { digits: result }
    }

    pub fn to_string(&self) -> &String { &self.digits }

    fn digits() -> Vec<(usize, &'static str)> {
        let mut digits = vec![ 
            (    1, "I"), (  4, "IV"), (  5, "V"), (  9, "IX"),
            (   10, "X"), ( 40, "XL"), ( 50, "L"), ( 90, "XC"),
            (  100, "C"), (400, "CD"), (500, "D"), (900, "CM"),
            ( 1000, "M"), ];
        digits.reverse();
        digits
    }
}
