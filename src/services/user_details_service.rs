use crate::r#struct::UserDetails::UserDetails;

pub fn get_user_details() ->UserDetails{
    UserDetails {
        name: "John Doe".to_string(),
        age: 30,
        email: "johndoe@example.com".to_string(),
    }
}