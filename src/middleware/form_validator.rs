use nickel::{
    Params
};
use std;

#[derive(Debug)]
pub struct FormValidationErrors;

pub trait FormValidator {
    fn validate(form_data: &Params)
            -> Result<Self, FormValidationErrors>
            where Self: std::marker::Sized {
        let _ = form_data;
        Ok(Self::fill_form(form_data))
    }
    fn fill_form(form_data: &Params) -> Self;
}