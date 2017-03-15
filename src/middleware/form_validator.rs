use nickel::{
    Params
};

#[derive(Debug)]
pub struct FormValidationErrors;

pub trait FormValidator {
    fn validate(form_data: &Params) -> Result<(), FormValidationErrors> {
        let _ = form_data;
        Ok(())
    }
    fn fill_form(form_data: &Params) -> Self;
}