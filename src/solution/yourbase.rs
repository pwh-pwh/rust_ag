#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    for &num in number {
        if num >= from_base {
            return Err(Error::InvalidDigit(num));
        }
    }
    if number.is_empty() {
        return Ok(vec![0]);
    }
    let mut n = 0;
    number
        .into_iter()
        .rev()
        .enumerate()
        .for_each(|(index, &num)| {
            n += from_base.pow(index as u32) * num;
        });
    Ok(d2n(n, to_base))
}

fn d2n(mut num: u32, tob: u32) -> Vec<u32> {
    let mut result = vec![];
    while num > 0 {
        let rm = num % tob;
        result.push(rm);
        num /= tob;
    }
    if result.is_empty() {
        result.push(0);
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        _ = convert(&[1, 1, 2, 0], 3, 2);
    }
}
