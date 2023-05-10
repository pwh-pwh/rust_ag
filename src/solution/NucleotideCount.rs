use std::collections::HashMap;
//ACGT
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let c = nucleotide;
    if c != 'A' && c != 'C' && c != 'G' && c != 'T' {
        return Err(c);
    }
    if let Some(c) = dna
        .chars()
        .find(|&c| c != 'A' && c != 'C' && c != 'G' && c != 'T')
    {
        return Err(c);
    }
    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(c) = dna
        .chars()
        .find(|&c| c != 'A' && c != 'C' && c != 'G' && c != 'T')
    {
        return Err(c);
    }
    let mut mp = HashMap::new();
    mp.insert('A', 0);
    mp.insert('G', 0);
    mp.insert('C', 0);
    mp.insert('T', 0);
    let m = dna.chars().fold(mp, |mut m, c| {
        *m.entry(c).or_default() += 1;
        m
    });
    Ok(m)
}
