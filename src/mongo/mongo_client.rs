use std::ptr::null;
use std::sync::Arc;
use log::info;
use mongodb::Client;
use crate::utils;

#[derive(Clone)]
pub struct MongoClient{
    client: Arc<mongodb::Client>
}

impl MongoClient {
    pub async fn new() -> mongodb::error::Result<Client> {
        let client = mongodb::Client::with_uri_str(utils::constants::MONGO_URI.clone()).await;
        client // Return the result directly
    }
}
