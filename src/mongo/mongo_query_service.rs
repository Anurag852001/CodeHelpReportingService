use bson::{doc, Document};
use crate::r#enum::question_type_enum::QuestionType;

pub fn build_mongo_query_for_number_of_question_solved(question_type: QuestionType, uuid: String) -> Vec<Document> {
    let pipeline = vec![
        doc! {"$match":{"questionType":question_type.to_str()} },
        doc! {"$group":{"_id":uuid,"count":{"$sum":1} } },
    ];
    pipeline
}