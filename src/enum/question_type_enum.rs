
pub enum QuestionType {
    Easy,Medium,Hard
}

impl QuestionType {
    pub fn to_str(&self) ->&str {
        match self {
            QuestionType::Easy => "Easy",
            QuestionType::Medium => "Medium",
            QuestionType::Hard => "Hard"
        }
    }

    pub fn from_str(question_type: &str) ->QuestionType {
        match question_type {
            "Easy" => QuestionType::Easy,
            "Medium" => QuestionType::Medium,
            "Hard" => QuestionType::Hard,
            _ => QuestionType::Easy
        }
    }
}