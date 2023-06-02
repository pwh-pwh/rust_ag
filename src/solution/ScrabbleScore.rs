/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut count = 0u64;
    word.chars().for_each(|c| count += get_char_value(c));
    count
}

fn get_char_value(c: char) -> u64 {
    let c = c.to_ascii_uppercase();
    let s1 = "AEIOULNRST";
    if s1.contains(c) {
        return 1;
    }
    let s2 = "DG";
    if s2.contains(c) {
        return 2;
    }
    let s3 = "BCMP";
    if s3.contains(c) {
        return 3;
    }
    let s4 = "F, H, V, W, Y";
    if s4.contains(c) {
        return 4;
    }
    if c == 'K' {
        return 5;
    }
    let s5 = "JX";
    if s5.contains(c) {
        return 8;
    }

    let s6 = "QZ";
    if s6.contains(c) {
        return 10;
    }
    return 0;
}
