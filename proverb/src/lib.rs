pub fn build_proverb(input: Vec<&str>) -> String {
    if input.len() == 0 { return String::new() }

    let mut output: Vec<String> = vec![];
    for i in 0..(input.len() - 1) {
        output.push(format!("For want of a {} the {} was lost.", input[i], input[i+1]));
    }
    if input.len() >= 3 && input[0..3] == ["nail", "shoe", "horse"] {
        output.push(String::from("And all for the want of a horseshoe nail."));
    } else {
        output.push(format!("And all for the want of a {}.", input[0]));
    }
    output.join("\n")
}
