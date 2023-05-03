pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() <= 1 {
        return false;
    }
    let mut bytes = code.into_bytes();
    if !bytes.iter().all(|c| *c >= '0' as u8 && *c <= '9' as u8) {
        return false;
    }
    bytes.reverse();
    let sum = bytes
        .iter_mut()
        .enumerate()
        .map(|(index, c)| {
            if index % 2 != 0 {
                let mut t = (*c - '0' as u8) as u8;
                t += t;
                if t > 9 {
                    t -= 9;
                }
                *c = ((t + '0' as u8) as u8);
                c
            } else {
                c
            }
        })
        .map(|c| (*c - '0' as u8) as u8)
        .sum::<u8>();
    sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let valid = is_valid("055 444 285");
        assert_eq!(valid, true)
    }
}
