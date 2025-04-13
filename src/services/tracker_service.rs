use log::info;
use crate::r#struct::TrackQuestion::TrackQuestion;
use once_cell::sync::OnceCell; // ✅ this one is thread-safe

use tokio::sync::mpsc::{channel, Sender, Receiver};

pub static QUESTION_TRACKER: OnceCell<Sender<TrackQuestion>> = OnceCell::new();

pub async fn init_question_tracker() {
    info!("Intializing QuestionTracker");
    let (question_sender, question_receiver): (Sender<TrackQuestion>, Receiver<TrackQuestion>) = channel(1000);
    QUESTION_TRACKER.set(question_sender).unwrap();

    tokio::spawn(async move { track_question(question_receiver).await });
    info!("Intialized QuestionTracker");
}

pub async fn track_question(mut question_receiver: Receiver<TrackQuestion>) {
    info!("Track question started");
    let client = match crate::mongo::mongo_client::MongoClient::new().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Error occured while initializing mongo client : {:?}", e);
            return;
        }
    };
    let database = client.database("codehelp");
    let collection = database.collection::<TrackQuestion>("codehelp");
 474364
    while let Some(question) = question_receiver.recv().await {
        let retry = 3;
        let mut attempt = 0;

        loop {
            match collection.insert_one(&question).await {
                Ok(_) => break,
                Err(e) => {
                    log::error!("Error occured while inserting question: {:?}, attempt :{:?}", e,attempt);
                    attempt += 1;
                }
            }
            if attempt >= retry {
                log::error!(
                    "Error occured while inserting question in attempts : {:?}",
                    attempt
                );
                break;
            }
        }
    }
}
