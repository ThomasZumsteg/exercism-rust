use std::collections::LinkedList;

pub struct Brackets { expression: String }

static BRACKET_PAIRS: &[(char, char)] = &[('(', ')'), ('[', ']'), ('{', '}')];

impl Brackets {

    pub fn from(expression: &str) -> Brackets {
        Brackets { expression: expression.to_string() }
    }

    pub fn are_balanced(&self) -> bool {
        let mut result: LinkedList<char> = LinkedList::new();
        for character in self.expression.chars() {
            if let Some(b) = Brackets::get_closing(character) { result.push_front(b) } 
            else if result.front() == Some(&character) { result.pop_front(); }
            else if Brackets::is_closing(character) { return false }
        }
        result.is_empty()
    }

    fn is_closing(bracket: char) -> bool {
        BRACKET_PAIRS.iter().any(|&(_, close)| close == bracket)
    }

    fn get_closing(bracket: char) -> Option<char> {
        for &(open, close) in BRACKET_PAIRS {
            if open == bracket { return Some(close) }
        }
        None
    }
}
