use bson::{doc};
use mongodb::Client;
use mongodb::options::ClientOptions;

use tokio;
use futures::stream::TryStreamExt;


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
