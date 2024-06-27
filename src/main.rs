fn main() {
    let string = "Scything isn't sorry";

    let mut indicators = Vec::new();

    for indicator in string.chars() {
        if indicator.is_ascii_alphabetic() {
            indicators.push(format!(
                ":regional_indicator_{}: ",
                indicator.to_lowercase()
            ));
        } else if indicator.is_numeric() {
            indicators.push(format!(":{indicator}:"));
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
}
