use nickel::{
    Params,
    BodyError,
};
use std;
use std::collections::HashMap;

macro_rules! validators {
    ( $([$($field:expr),*]: $vtype:ty => $($func:ident$args:tt),*; )+ ) => {
        $(
            let v = vec![$($field),*];
            let y = vec![$($func$args),*];
            println!("Fn: {:#?}: {:#?}", v, y)
        )+
    };
}

#[derive(Debug)]
pub struct Validators;

pub type FormValidationErrors<'a> = HashMap<&'a str, Vec<FormValidationError>>;

#[derive(Debug, Clone)]
pub enum FormValidationError {
    FieldNotExist,
    ParseError
}

pub trait FormValidator {
    fn validate(form_data: &Params)
            -> Result<Self, FormValidationErrors>
            where Self: std::marker::Sized {
        let _validators = Self::validators();
        create_validator(&form_data);
        Ok(Self::fill_form(&form_data)?)
    }
    fn validators() -> Validators;
    fn fill_form(form_data: &Params)
            -> Result<Self, FormValidationErrors>
            where Self: std::marker::Sized;
}

fn create_validator(form_data: &Params) -> Result<(), FormValidationErrors> {
    let mut validation_errors: FormValidationErrors = HashMap::new();
    let field_name = "done.a";

    // Fetch fields withour any results
    // We iterate over all fields with `field_name`
    // Try parse with specific type
    // We gathering all possible errors, without skiping any posible error
    form_data.all(field_name)
        // If field not exist - add error
        .or_else(|| add_error(&mut validation_errors, field_name, FormValidationError::FieldNotExist) )
        .and_then(|v| {
            // Iterate over fields
            v.iter()
                .map(|x| {
                    // Type inference via Parse
                    x.parse::<i32>().ok()
                        // If parse Error - add error
                        .or_else(|| add_error(&mut validation_errors, field_name, FormValidationError::ParseError));
                    // Return same field
                    x
                }).cloned().collect::<String>();
            // Guaranty unwrap result
            Some(1)
        }).unwrap();
    println!("Errors: {:#?}", validation_errors);

    validators! (
        ["tst1", "tst2"]: i32 => required(), max(20);
        ["tst3", "tst4"]: i32 => max(41);
    );
    if validation_errors.is_empty() { Ok(()) } else { Err(validation_errors) }
}

fn add_error<'a, T>(validation_errors: &mut FormValidationErrors<'a>, field: &'a str, error: FormValidationError) -> Option<T> {
    if validation_errors.contains_key(&field) {
        let mut err = validation_errors.get_mut(&field).unwrap();
        err.push(error);
    } else {
        validation_errors.insert(&field, vec!(error));
    }
    None
}

fn required() {
    // required template
}

fn max(value: i32) {
    // max template
}
