pub fn encode(num: u64) -> String {
    if num == 0 { return String::from("zero"); }

    let powers = vec![""," thousand"," million"," billion", 
        " trillion", " quadrillion", " quintillion"];
    let mut result: Vec<String> = vec![];

    for (i, mag) in powers.iter().enumerate() {
        let m = magnitude((num / 10u64.pow((i as u32)*3)) % 1000);
        if m != "" { result.insert(0, format!("{}{}", m, mag)) }
    }
    result.join(" ")
}

pub fn magnitude(num: u64) -> String {
    let hundereds = digits(num / 100);
    let teens = teens( num % 100 );

    let mut output = String::new();

    if hundereds != "" { output = format!("{}{} hundred ", output, hundereds) }
    if teens != "" { return format!("{}{}", output, teens) }

    let tens = tens( (num % 100)/10 );
    let ones = digits( num % 10 );
    
    if tens != "" && ones != "" { output = format!("{}{}-{}", output, tens, ones) }
    else if tens != "" { output = format!("{}{}", output, tens) }
    else { output = format!("{}{}", output, ones) }
    output.trim().to_string()
}

fn digits(num: u64) -> &'static str {
    match num {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "",
    }
}

fn teens(num: u64) -> &'static str {
    match num {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }
}

fn tens(num: u64) -> &'static str {
    match num % 10 {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    }
}
