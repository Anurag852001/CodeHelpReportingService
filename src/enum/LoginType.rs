use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub enum LoginType{
    ADMIN,USER
}