use bson::{doc, Document};
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;

use tokio;
use futures::stream::TryStreamExt;
use futures::StreamExt;
use log::info;
use crate::mongo::mongo_client;
use crate::mongo::mongo_client::MongoClient;
use crate::mongo::mongo_query_service::build_mongo_query_for_number_of_question_solved;
use crate::r#enum::question_type_enum::QuestionType;

pub async fn test_mongo() -> mongodb::error::Result<()> {
    println!("test_mongo");

    // Connect to MongoDB
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let new_doc = doc! {
    "name": "Anurag",
    "age": 30,
    "city": "Wonderland"
};
    // Select the database
    let database = client.database("codehelp");

    // Ping the database
    let ping_result = database.run_command(doc! {"ping": 1}).await?;
    println!("Ping result: {:?}", ping_result);

    // Access a specific collection
    let collection = database.collection::<bson::Document>("codehelp");
    collection.insert_one(new_doc).await?;
    // Define an aggregation pipeline
    let pipeline = vec![
        doc! { "$group": { "_id": null, "sum": { "$sum": 1 } } }
    ];

    // Execute aggregation
    let mut cursor = collection.aggregate(pipeline).await?;

    // Process aggregation results
    while let Some(doc) = cursor.try_next().await? {
        println!("Document: {:?}", doc);
    }

    println!("Collection ready to use!");

    Ok(())
}

pub async fn calculate_question_solved(uuid:String,question_type: QuestionType)-> i32 {
    let pipeline = build_mongo_query_for_number_of_question_solved(question_type, uuid);
    info!("{:?}", pipeline);
    let mongo_client = MongoClient::new().await;
    let db = mongo_client.unwrap().database("codehelp");
    let collection:Collection<Document> = db.collection("codehelp");
    collection.aggregate(pipeline).await.unwrap().count().await as i32
}