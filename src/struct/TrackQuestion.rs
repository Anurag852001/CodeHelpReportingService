use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct TrackQuestion {
    question_no:i32,
    difficulty:String,
    uuid:String
}

impl TrackQuestion {
    pub fn new(question_no:i32, difficulty:String, uuid:String) -> TrackQuestion {
        TrackQuestion{question_no, difficulty, uuid}
    }
}