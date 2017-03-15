use super::*;
use middleware::form_validator::FormValidator;

#[derive(Debug)]
pub struct Pages {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl FormValidator for Pages {
    fn fill_form(_: &Params) -> Self {
        Pages {
            id: 10,
            title: "Title".to_string(),
            body: "Body".to_string(),
            published: false,
        }
    }

}
