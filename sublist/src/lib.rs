#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(needle: &[T], haystack: &[T]) -> Comparison {
    if needle.len() > haystack.len() {
        let result = sublist(haystack, needle);
        if result == Comparison::Sublist { return Comparison::Superlist }
        else { return result }
    }
    for i in 0..(haystack.len() - needle.len() + 1) {
        if haystack[i..(i+needle.len())] == needle[..] { 
            if needle.len() == haystack.len() { return Comparison::Equal }
            else { return Comparison::Sublist }
        }
    }
    Comparison::Unequal
}
