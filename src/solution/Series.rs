pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .as_bytes()
        .windows(len)
        .map(|bs| unsafe { String::from_utf8_unchecked(bs.to_vec()) })
        .collect()
}
