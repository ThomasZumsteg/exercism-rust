use std::iter::repeat;

pub fn get_diamond(c: char) -> Vec<String> {
    let length =  (c as u8) - b'A' + 1;
    let mut diamond = Vec::new();
    for r in 0..length {
        let mut row = repeat(' ').take(length as usize).collect::<Vec<char>>();
        row[r as usize] = (b'A' + r) as char;
        let mut reversed = row.clone();
        reversed.reverse();
        row.drain(0..1);
        reversed.extend(row);
        diamond.push(reversed.iter().collect());
    } 
    let mut bottom = diamond.clone();
    bottom.reverse();
    bottom.drain(0..1);
    diamond.extend(bottom);
    return diamond
}
