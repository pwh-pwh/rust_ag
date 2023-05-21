pub fn encode(source: &str) -> String {
    let mut pre = '%';
    let mut count = 0;
    let mut result = String::from("");
    for (index, x) in source.chars().enumerate() {
        if index == source.len() - 1 {
            if x == pre {
                count += 1;
                result.push_str(format!("{}{}", count, pre).as_str());
            } else {
                if count == 1 {
                    result.push(pre);
                } else {
                    result.push_str(format!("{}{}", count, pre).as_str());
                }
                result.push(x);
            }
            break;
        }
        if x != pre {
            if pre != '%' {
                if count == 1 {
                    result.push(pre);
                } else {
                    result.push_str(format!("{}{}", count, pre).as_str());
                }
                pre = x;
                count = 1;
            } else {
                pre = x;
                count += 1;
            }
        } else {
            count += 1;
            pre = x;
        }
    }
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::from("");
    let mut count = 0;
    for x in source.chars() {
        if x.is_digit(10) {
            count = count * 10 + x.to_digit(10).unwrap();
        } else {
            if count == 0 {
                count = 1;
            }
            for _ in 0..count {
                result.push(x);
            }
            count = 0;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        assert_eq!(encode("zzz ZZ  zZ"), "3z 2Z2 zZ");
    }
    #[test]
    fn test_decode() {
        assert_eq!(decode("3z 2Z2 zZ"), "zzz ZZ  zZ");
    }
}
