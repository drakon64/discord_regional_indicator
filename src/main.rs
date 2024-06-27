fn main() {
    let string = "Scything isn't sorry";

    let mut indicators = vec![Vec::new()];
    let mut line_count: usize = 0;

    for indicator in string.chars() {
        if indicator.is_ascii_alphabetic() {
            indicators[line_count].push(format!(
                ":regional_indicator_{}: ",
                indicator.to_lowercase()
            ));
        } else if indicator.is_numeric() {
            indicators[line_count].push(format!(":{indicator}:"));
        } else if indicator.is_whitespace() && !indicator.eq(&'\n') {
            indicators[line_count].push(" ".to_string());
        } else if indicator.eq(&'\n') {
            line_count += 1;
            indicators.push(Vec::new())
        } else if indicator.eq(&'!') {
            indicators[line_count].push(":exclamation: ".to_string());
        } else if indicator.eq(&'?') {
            indicators[line_count].push(":question: ".to_string());
        }
    }

    for discord in indicators {
        println!("{}", discord.join("").trim_end());
    }
}
