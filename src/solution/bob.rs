pub fn reply(message: &str) -> &str {
    match message {
        msg if msg
            .as_bytes()
            .iter()
            .any(|&c| (c >= 'a' as u8 && c <= 'z' as u8) || (c >= 'A' as u8 && c <= 'Z' as u8))
            && msg.ends_with("?")
            && msg.to_string().eq(&msg.to_uppercase()) =>
        {
            "Calm down, I know what I'm doing!"
        }
        msg if msg.trim_end().ends_with("?") => "Sure.",
        msg if msg.trim().is_empty() => "Fine. Be that way!",
        msg if msg
            .as_bytes()
            .iter()
            .any(|&c| (c >= 'a' as u8 && c <= 'z' as u8) || (c >= 'A' as u8 && c <= 'Z' as u8))
            && msg.to_string().eq(&msg.to_uppercase()) =>
        {
            "Whoa, chill out!"
        }
        _ => "Whatever.",
    }
}

#[cfg(test)]
mod tests {

    fn A(numR: &i64) {}
    fn B(s: &str) {}

    #[test]
    fn test() {
        let a = &3;
        A(a);
        A(a);

        let s = "aa";
        let ss = s;
        B(s);
        B(s);
    }
}
