pub fn square_of_sum(n: u32) -> u32 {
    (1..n + 1).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n + 1).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::solution::defSquares::difference;

    #[test]
    fn test() {
        assert_eq!(3025, square_of_sum(10));
        assert_eq!(385, sum_of_squares(10));
        assert_eq!(2640, difference(10));
    }
}
