use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    input.into_iter().for_each(|&s| {
        s.to_lowercase().chars().for_each(|c| {
            if !c.is_ascii_digit() && c.is_alphabetic() {
                let v = *map.entry(c).or_default() + 1;
                map.insert(c, v);
            }
        });
    });
    map
}
