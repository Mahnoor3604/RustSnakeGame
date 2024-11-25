pub struct Score {
    score: usize,
}

impl Score {
    pub fn new() -> Self {
        Score {
            score: 0,
        }
    }

    pub fn print(&mut self) -> String {
        let score: String = self.score.to_string();
        return score;
    }

    pub fn eat(&mut self) {
        self.score += 1;
    }

    pub fn get_score(&mut self) -> usize {
        return self.score;
    }
}