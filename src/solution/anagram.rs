use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = string_lowercase(word);
    let w_cs = checksum(&word);
    let wlen = word.len();
    possible_anagrams
        .iter()
        //过滤方法
        .filter(|&pa| {
            let pa = string_lowercase(pa);
            if pa.len() != wlen || w_cs != checksum(&pa) || pa == word {
                return false;
            }
            let mut len: usize = 0;
            pa.chars().all(|c| {
                len += 1;
                len > wlen
                    || word.chars().filter(|&x| x == c).count()
                        == pa.chars().filter(|&x| x == c).count()
            })
        })
        .map(|&x| x)
        .collect()
}

const CASE_MASK: u8 = 32;

#[inline(always)]
fn char_lowercase(c: char) -> char {
    if c.is_ascii_alphabetic() {
        (c as u8 | CASE_MASK) as char
    } else if c.is_uppercase() {
        c.to_lowercase().next().unwrap()
    } else {
        c
    }
}

fn string_lowercase(s: &str) -> String {
    s.chars().map(|c| char_lowercase(c)).collect()
}

#[inline(always)]
fn checksum(word: &str) -> u8 {
    word.bytes().fold(0u8, |a, b| a.overflowing_add(b).0)
}

#[cfg(test)]
mod test {
    use crate::solution::anagram;
    use std::collections::HashSet;
    use std::{assert_eq, vec};

    #[test]
    fn test_an() {
        let word = "aa";
        let inputs = ["aa"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
        let result = anagram::anagrams_for(word, inputs);
        let expected: HashSet<&str> = expected.iter().cloned().collect();

        assert_eq!(result, expected);
    }
}
