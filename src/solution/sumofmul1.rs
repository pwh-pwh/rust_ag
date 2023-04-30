pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|&lt|
        factors.into_iter().any(|&ft|ft!=0 && lt % ft == 0)
    ).sum()
}

#[cfg(test)]
mod tests {
    use crate::solution::sumofmul::sum_of_multiples;

    #[test]
    fn test() {
        println!("{}",sum_of_multiples(1000, &[3, 5]))
    }
}