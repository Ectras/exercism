#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self {
            scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.iter().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    // From https://stackoverflow.com/a/57815298
    pub fn position_max_copy<T: Ord + Copy>(slice: &[T]) -> Option<usize> {
        slice.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut copy = self.scores.to_vec();
        let mut top = Vec::new();
        for i in 0..3 {
            let index = Self::position_max_copy(&copy);
            match index {
                Some(index) => {
                    top.push(copy[index]);
                    copy.swap_remove(index);
                },
                None => break,
            }
        };
        top
    }
}
