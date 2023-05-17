use std::ops::Index;

pub fn number(user_number: &str) -> Option<String> {
    let user_number: String = user_number.chars().filter(|c| c.is_numeric()).collect();
    match user_number.len() {
        11 if va11(&user_number) => Some(user_number[1..].to_string()),
        10 if va10(&user_number) => Some(user_number),
        7 if user_number.starts_with(num2To9) => Some(user_number),
        _ => None,
    }
}

fn va10(s: &str) -> bool {
    s.starts_with(num2To9) && s.chars().nth(3).map_or(false, num2To9)
}

fn va11(s: &str) -> bool {
    s.starts_with("1")
        && s.chars().nth(1).map_or(false, num2To9)
        && s.chars().nth(4).map_or(false, num2To9)
}

fn num2To9(c: char) -> bool {
    ('2'..='9').contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let option = number("1 (023) 456-7890");
        println!("{:#?}", option);
    }
}
