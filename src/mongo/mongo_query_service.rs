use bson::{doc, Document};
use crate::r#enum::Difficulty::Difficulty;

pub fn build_mongo_query_for_number_of_question_solved(question_type: Difficulty, uuid: String) -> Vec<Document> {
    let pipeline = vec![
        doc! {"$match":{"difficulty":question_type.to_str(),"uuid":uuid}, },
        doc! {"$group":{"_id":"null","count":{"$sum":1} } },
    ];
    pipeline
}