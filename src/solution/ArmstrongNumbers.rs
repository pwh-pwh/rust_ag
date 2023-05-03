pub fn is_armstrong_number(num: u32) -> bool {
    let numStr = num.to_string();
    let numLen = numStr.len() as u32;
    if numLen >= 10 {
        return false;
    }
    let sum: u32 = numStr
        .into_bytes()
        .into_iter()
        .map(|c| (c - '0' as u8) as u32)
        .map(|c| c.pow(numLen))
        .sum();
    sum == num
}

#[cfg(test)]
mod tests {}
