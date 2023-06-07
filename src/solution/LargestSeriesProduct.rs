#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        Err(Error::SpanTooLong)
    } else {
        let chs = string_digits
            .chars()
            .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
            .collect::<Result<Vec<u32>, Error>>()?;
        chs.windows(span)
            .map(|v| v.iter().map(|&n| n as u64).product())
            .max()
            .ok_or(Error::SpanTooLong)
    }
}
