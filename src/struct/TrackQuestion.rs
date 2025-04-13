use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct TrackQuestion {
    question_no:i32,
    uuid:String,
    solved:bool,
    runtime:i64,
    language:String,
}

impl TrackQuestion {
    pub fn new(question_no:i32, uuid:String, solved:bool,runtime:i64,language:String) -> TrackQuestion {
        TrackQuestion{question_no, uuid, solved,runtime, language}
    }
}