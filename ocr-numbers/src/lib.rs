// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[allow(unused_variables)]
pub fn convert(input: &str) -> Result<String, ()> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut result = String::new();
    let mut row = 0;
    if lines.len() % 4 != 0 { return Err(()) }
    while row < lines.len() {
        for j in 0.. {
            if j >= 1 { break; }
        }
        println!("\n{}", &lines[row..(row+3)].join("\n"));
        row += 4;
    }
    Ok(get_digit(input))
}

fn get_digit(input: &str) -> String {
    match input {
        "   \n  |\n  |\n   " => "1",
        _ => "?",
    }.to_string()
}
