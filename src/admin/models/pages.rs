use super::*;
use middleware::form_validator::FormValidator;

pub struct Pages {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl FormValidator for Pages {
    fn fill_form(&self) -> Self {
        Pages {
            id: 10,
            title: "Title".to_string(),
            body: "Body".to_string(),
            published: false,
        }
    }

}
