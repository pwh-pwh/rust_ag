pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut cand = 2..;
    while n>1 {
        let i = cand.next().unwrap();
        while n%i==0 {
            result.push(i);
            n/=i;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        println!("{:?}",factors(93_819_012_551));
    }
}