use std::env::args;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = args().skip(1).next();

    if args.is_some() {
        let binding = args.unwrap();

        let mut indicators = Vec::new();

        for indicator in binding.chars() {
            if indicator.is_ascii_alphabetic() {
                indicators.push(format!(
                    ":regional_indicator_{}: ",
                    indicator.to_lowercase()
                ));
            } else if indicator.is_numeric() {
                let number;

                match indicator as u8 - '0' as u8 {
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
                    _ => panic!() // Unreachable
                }

                indicators.push(format!(":{number}: "));
            } else if indicator.is_whitespace() && !indicator.eq(&'\n') {
                indicators.push(" ".to_string());
            } else if indicator.eq(&'\n') {
                indicators.push('\n'.to_string())
            } else if indicator.eq(&'!') {
                indicators.push(":exclamation: ".to_string());
            } else if indicator.eq(&'?') {
                indicators.push(":question: ".to_string());
            }
        }

        println!("{}", indicators.join("").trim_end());

        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
