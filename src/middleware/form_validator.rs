use nickel::{
    Params
};
use std;

#[derive(Debug)]
pub struct Validators;

#[derive(Debug)]
pub struct FormValidationErrors;

pub trait FormValidator {
    fn validate(form_data: &Params)
            -> Result<Self, FormValidationErrors>
            where Self: std::marker::Sized {
        let _validators = Self::validators();
        Ok(Self::fill_form(form_data)?)
    }
    fn validators() -> Validators;
    fn fill_form(form_data: &Params)
            -> Result<Self, FormValidationErrors>
            where Self: std::marker::Sized;
}