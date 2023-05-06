pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_ascii_uppercase())
                    .filter(|c| c.is_ascii_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let mut a = 1;
        let mut b = 2;
        let f1 = || a + b;
        println!("{}", f1());
        b += 1;
        // println!("{}", f1());
    }
}
