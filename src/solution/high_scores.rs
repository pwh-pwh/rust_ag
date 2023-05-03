#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|v| *v)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().map(|v| *v)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone();
        scores.sort();
        scores.into_iter().rev().take(3).collect()
    }
}
