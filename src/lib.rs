#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        /* unimplemented!("Construct a HighScores struct, given the scores: {scores:?}") */
       HighScores {
           scores: scores.to_vec(),
       }
        }

    pub fn scores(&self) -> &[u32] {
        &self.scores


    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().copied()


    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorting_sores = self.scores.clone();
        sorting_sores.sort_unstable_by(|a, b| b.cmp(a));
        sorting_sores.into_iter().take(3).collect()
    }
}
