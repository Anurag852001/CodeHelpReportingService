
#[derive(Debug)]
pub enum Difficulty {
    Easy,Medium,Hard
}

impl Difficulty {
    pub fn to_str(&self) ->&str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard"
        }
    }

    pub fn from_str(difficulty: &str) -> Difficulty {
        match difficulty {
            "Easy" => Difficulty::Easy,
            "Medium" => Difficulty::Medium,
            "Hard" => Difficulty::Hard,
            _ => Difficulty::Easy
        }
    }
}
