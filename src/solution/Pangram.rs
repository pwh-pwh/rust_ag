use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let set: HashSet<char> = sentence
        .to_lowercase()
        .chars()
        .into_iter()
        .filter(|&c| c >= 'a' && c <= 'z')
        .collect();
    set.len() == 26
}

#[cfg(test)]
mod tests {
    use crate::solution::Pangram::is_pangram;

    #[test]
    fn test() {
        let s = "\"Five quacking Zephyrs jolt my wax bed.\"";
        println!("{}", is_pangram(s));
    }
}
