use std::cmp::min;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        //判断是否回文数
        let mut temp = value;
        let mut reverse = 0;
        while temp != 0 {
            reverse = reverse * 10 + temp % 10;
            temp /= 10;
        }
        if reverse == value {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut maxP = Palindrome(0);
    let mut minP = Palindrome(0);
    for i in min..=max {
        for j in min..=max {
            if Palindrome::new(i * j).is_some() {
                if minP.0 == 0 {
                    minP = Palindrome(i * j);
                } else if minP.0 > i * j {
                    minP = Palindrome(i * j);
                }
                if maxP.0 == 0 {
                    maxP = Palindrome(i * j);
                } else if maxP.0 < i * j {
                    maxP = Palindrome(i * j);
                }
            }
        }
    }
    if minP.0 != 0 {
        return Some((minP, maxP));
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test() {
        let option = palindrome_products(100, 999);
        println!("{:?}", option);
        panic!("");
    }
}
