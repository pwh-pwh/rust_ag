use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    (0..worker_count)
        .into_par_iter()
        .map(|i| {
            let mut tallies = HashMap::new();
            input
                .join("")
                .chars()
                .skip(i)
                .step_by(worker_count)
                .filter(|c| c.is_alphabetic())
                .flat_map(|c| c.to_lowercase())
                .for_each(|c| *tallies.entry(c).or_insert(0) += 1);
            tallies
        })
        .reduce(HashMap::new, |mut result, m| {
            m.iter().for_each(|(&k, &v)| {
                *result.entry(k).or_insert(0) += v;
            });
            result
        })
}
