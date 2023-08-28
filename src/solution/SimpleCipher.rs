use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().into_iter().all(|c| c.is_ascii_lowercase()) {
        None
    } else {
        let mut result = String::default();
        s.chars().enumerate().for_each(|(i, c)| {
            let mut t = match key.as_bytes().get(i) {
                Some(&c) => c,
                None => b'a',
            } - b'a'
                + c as u8;
            if t > b'z' {
                t -= 26;
            }
            result.push(t as char);
        });
        Some(result)
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().into_iter().all(|c| c.is_ascii_lowercase()) {
        None
    } else {
        let mut result = String::default();
        s.chars().enumerate().for_each(|(index, c)| {
            let mut t = c as u8
                - (match key.as_bytes().get(index) {
                    Some(&c) => c,
                    None => b'a',
                } - b'a');
            if t < b'a' {
                t += 26;
            }
            result.push(t as char);
        });
        Some(result)
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut r = String::default();
    (0..100).for_each(|_| {
        let mut rng = rand::thread_rng();
        let c = rng.gen_range('a'..='z');
        r.push(c);
    });
    let v = encode(&r, s).unwrap();
    (r, v)
}

#[cfg(test)]
mod tests {

    const PLAIN_TEXT: &str = "thisismysecret";

    const KEY: &str = "abcdefghij";

    use super::*;
    #[test]
    fn test() {
        // let option = decode(KEY, &encode(KEY, PLAIN_TEXT));
        // println!("{option:?}",);
        // panic!("log");
        let option = encode(KEY, PLAIN_TEXT);
        println!("{option:?}");
        panic!("log");
    }
}
