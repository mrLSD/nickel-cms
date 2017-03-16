use nickel::{
    Params,
    BodyError,
};
use std;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Validators;

#[derive(Debug)]
pub enum FormValidationErrors {
    FieldNotExist
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
    let mut validation_errors: HashMap<&str, Vec<FormValidationErrors>> = HashMap::new();
    let field_name = "done.a1";
    let get_form = form_data.get(field_name);

    match get_form {
        Some(x) => println!("{:#?}", x),
        _ => {
            let err = vec![FormValidationErrors::FieldNotExist];
            validation_errors.insert(field_name, err);
        },
    }
    let get_data: Option<String> = form_data.get(field_name)
        .or_else(|| { println!("Field not exist"); None })
        .and_then(|o| o.parse().ok());
    println!("{:#?}", get_data);

    Ok(())
}
