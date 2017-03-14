use nickel::{
    Params
};

pub struct FormValidationErrors;

pub trait FormValidator {
    fn validate(form_data: &Params) -> Result<(), FormValidationErrors> {
        let _ = form_data;
        Ok(())
    }
    fn fill_form(&self) -> Self;
}