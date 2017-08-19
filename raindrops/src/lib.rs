
pub fn raindrops(num: isize) -> String {
    let mut result: String = String::from("");
    if num % 3 == 0 { result += "Pling" }
    if num % 5 == 0 { result += "Plang" }
    if num % 7 == 0 { result += "Plong" }
    if result.len() == 0 { result = format!("{}", num) }
    result
}
