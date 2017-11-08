pub fn convert(input: &str) -> Result<String, ()> {
    let lines = input.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
    let mut result = String::new();
    // Make this an interator for row in lines.group_iter(grouper)
    for r in 0.. {
        let row = r * 4;
        if row + 4 > lines.len() { return Err(()) }
        // Also here for digit in row.group_iter(grouper)
        for c in 0.. {
            let col = c * 3;
            if col == lines[row].len() { break }
            if lines[row..row+4].iter().any(|r| col + 3 > r.len()) {
                return Err(())
            }
            result += get_digit(
                lines[row..row+4]
                    .iter()
                    .map(|l| l[col..col+3].to_string())
                    .collect());
        }
        if row+4 == lines.len() { break }
        result += ",";
    }
    Ok(result)
}

fn get_digit(input: Vec<String>) -> &'static str {
    match &*input.join("\n") {
        " _ \n| |\n|_|\n   " => "0",
        "   \n  |\n  |\n   " => "1",
        " _ \n _|\n|_ \n   " => "2",
        " _ \n _|\n _|\n   " => "3",
        "   \n|_|\n  |\n   " => "4",
        " _ \n|_ \n _|\n   " => "5",
        " _ \n|_ \n|_|\n   " => "6",
        " _ \n  |\n  |\n   " => "7",
        " _ \n|_|\n|_|\n   " => "8",
        " _ \n|_|\n _|\n   " => "9",
        _ => "?",
    }
}
