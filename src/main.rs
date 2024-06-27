use std::env::args;

fn main() {
    let mut indicators = Vec::new();

    for character in args().skip(1).next().unwrap().chars() {
        if character.is_ascii_alphabetic() {
            indicators.push(format!(
                ":regional_indicator_{}: ",
                character.to_lowercase()
            ));
        } else if character.is_numeric() {
            let number;

            match character as u8 - '0' as u8 {
                0 => number = "zero",
                1 => number = "one",
                2 => number = "two",
                3 => number = "three",
                4 => number = "four",
                5 => number = "five",
                6 => number = "six",
                7 => number = "seven",
                8 => number = "eight",
                9 => number = "nine",
                _ => panic!(), // Unreachable
            }

            indicators.push(format!(":{number}: "));
        } else if character.is_whitespace() && !character.eq(&'\n') {
            indicators.push(" ".to_string());
        } else if character.eq(&'\n') {
            indicators.push('\n'.to_string())
        } else if character.eq(&'!') {
            indicators.push(":exclamation: ".to_string());
        } else if character.eq(&'?') {
            indicators.push(":question: ".to_string());
        }
    }

    println!("{}", indicators.join("").trim_end())
}
