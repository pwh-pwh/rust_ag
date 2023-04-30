use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();
    factors.into_iter().map(|& f| {
        if f == 0 {
            return 0;
        }
        let mut r = 0;
        let mut p = 2;
        let mut t = f;
        while t<limit {
            if set.contains(&t) {

            } else {
                r+=t;
                set.insert(t);
            }
            t=p*f;
            p+=1;
        }
        r
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::solution::sumofmul::sum_of_multiples;

    #[test]
    fn test() {
        println!("{}",sum_of_multiples(1000, &[3, 5]))
    }
}