pub fn square(s: u32) -> u64 {
    if s>64 || s==0 {
        panic!("Square must be between 1 and 64");
    }
    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    18_446_744_073_709_551_615
}

#[cfg(test)]
mod tests {
    use crate::solution::grains::square;

    #[test]
    fn test() {
        assert_eq!(4,square(3));
    }
}