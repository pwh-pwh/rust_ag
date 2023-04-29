pub fn nth(n: u32) -> u32 {
    let mut primes = vec![];
    (2..)
        .filter(|c|{
            if !primes.iter().any(|p|c%p==0) {
                primes.push(*c);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}