use log::error;
use mongodb::Collection;
use tokio::sync::mpsc::error::SendError;
use crate::handlers::code_help_reporting_handler::track_question;
use crate::r#struct::TrackQuestion::TrackQuestion;
use crate::services::tracker_service::QUESTION_TRACKER;

pub async fn track_que(track_question_obj:TrackQuestion)->bool {
    if let Some(sender) = QUESTION_TRACKER.get() {
        if let Err(e) = sender.send(track_question_obj).await {
            error!("Failed to send track_question to worker: {:?}", e);
            return false;
        }
    } else {
        error!("Track sender not initialized");
        return false;
    }
    return true;
}