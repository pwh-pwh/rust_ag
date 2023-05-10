#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let option = dna
            .chars()
            .enumerate()
            .find(|&(index, c)| c != 'A' && c != 'C' && c != 'T' && c != 'G');
        if let Some((index, _)) = option {
            return Err(index);
        }
        Ok(Dna(dna.into()))
    }

    pub fn into_rna(self) -> Rna {
        let rnaS: String = self
            .0
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'T' => 'A',
                'G' => 'C',
                _ => 'A',
            })
            .collect();
        Rna(rnaS)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let option = rna
            .chars()
            .enumerate()
            .find(|&(index, c)| c != 'A' && c != 'C' && c != 'U' && c != 'G');
        if let Some((index, _)) = option {
            return Err(index);
        }
        Ok(Rna(rna.into()))
    }
}
