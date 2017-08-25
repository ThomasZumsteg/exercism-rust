extern crate pythagorean_triplet;

use pythagorean_triplet::find_in_range;

fn main() {
    let result = find_in_range(1, 1000);
    match result.len() {
        0 => println!("No matches found"),
        1 => println!("Found {} {} {}", result[0].0, result[0].1, result[0].2),
        _ => println!("Found many matches"),
    }
}
