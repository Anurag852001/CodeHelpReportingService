use serde::Serialize;

#[derive(Serialize)]
pub struct UserDetails{
    pub(crate) name: String,
    pub(crate) age: u8,
    pub(crate) email: String,
}

impl UserDetails {
    pub fn new(name: String, age: u8, email: String){
       UserDetails{name, age, email};
    }
}