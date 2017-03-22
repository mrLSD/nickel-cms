use nickel::{
    Params,
};
use std;
use std::collections::HashMap;

macro_rules! parse_field {
    ( $field:expr, $vtype:ty ) => {{
        $field.parse::<$vtype>()
    }}
}

macro_rules! validators {
    ( $([$($field:expr),*]: $vtype:ty => $($func:ident$args:tt),*; )+ ) => {{
        let mut rules: ValidatorParams = Vec::new();
        $(
            let rule = ValidatorParam {
                fields: vec![$($field),*],
                validators: vec![$($func),*],
                type_ref: stringify!($vtype),
                validators_arg: validators! ( @validator_func $($args),* ),
            };
            rules.push(rule);
        )+
        rules
    }};
    ( @validator_func $($args:tt),* ) => {{
        vec!( $( validators! ( @validator_func_args $args ) ),* )
    }};
    ( @validator_func_args () ) => {{
        ValidatorFuncArgs::NoParams
    }};
    ( @validator_func_args ($val:expr) ) => {{
        ValidatorFuncArgs::OneParam($val.to_string())
    }};
    ( @validator_func_args ($val1:expr, $val2:expr) ) => {{
        ValidatorFuncArgs::TwoParam($val1.to_string(), $val2.to_string())
    }};
}

pub struct Validators<'a> {
    validators: ValidatorParams<'a>,
}

#[derive(Debug, Clone)]
pub enum ValidatorFuncArgs {
    NoParams,
    OneParam(String),
    TwoParam(String, String),
}

pub type ValidatorParams<'a> = Vec<ValidatorParam<'a>>;

pub struct ValidatorParam<'a> {
    pub fields: Vec<&'a str>,
    pub validators: Vec<fn(ValidatorFuncArgs)>,
    pub validators_arg: Vec<ValidatorFuncArgs>,
    pub type_ref: &'a str,
}

pub type FormValidationErrors<'a> = HashMap<&'a str, Vec<FormValidationError>>;

#[derive(Debug, Clone, PartialEq)]
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
    fn validators<'a>() -> ValidatorParams<'a>;
    fn fill_form(form_data: &Params)
            -> Result<Self, FormValidationErrors>
            where Self: std::marker::Sized;
}

impl<'a> Validators<'a> {
    fn new(validators: ValidatorParams) {
//--
    }
}

fn create_validator(form_data: &Params) -> Result<(), FormValidationErrors> {
    let mut validation_errors: FormValidationErrors = HashMap::new();

    let tst = validators! (
        ["tst1", "tst2"]: i32 => required(), max(1);
        ["tst3", "tst4"]: i32 => max(1, "2+");
    );

    for validator in tst {
        for (key, ref val) in validator.validators.iter().enumerate() {
            let args = validator.validators_arg[key].to_owned();
            for field_name in &validator.fields {
                println!("Fields: {:#?}", field_name);
                println!("Args: {:#?}", args);

                // Fetch fields withour any results
                // We iterate over all fields with `field_name`
                // Try parse with specific type
                // We gathering all possible errors, without skiping any posible error
                form_data.all(field_name)
                    // If field not exist - add error
                    .or_else(|| add_error(&mut validation_errors, field_name, FormValidationError::FieldNotExist))
                    .and_then(|v| {
                        // Iterate over fields
                        Some(v.iter()
                            .map(|x| {
                                // Type inference via Parse
                                parse_field!(x, i32).ok()
                                    // If parse Error - add error
                                    .or_else(|| add_error(&mut validation_errors, field_name, FormValidationError::ParseError));
                                // Return same field
                                x
                            }).cloned().collect::<String>())
                    });
            }
        }
    }

    println!("Errors: {:#?}", validation_errors);

    /*
    println!("Validators: {:#?}", tst.len());
    for vl in tst {
        println!("Fields: {:#?}", vl.fields);
        println!("Validators count: {:#?}", vl.validators.len());
        println!("Args count: {:#?}", vl.validators_arg.len());
        for (key, ref val) in vl.validators.iter().enumerate() {
            let args = vl.validators_arg[key].to_owned();
            val(args);
            //println!("Index: {:#?}", (*val)(vl.validators_arg[key]));
            println!("Field: {:#?}", vl.fields[key]);
            println!("Index: {:#?}", vl.validators_arg[key]);
            //val(vl.validators_arg[key]);
        }
    }*/
    if validation_errors.is_empty() { Ok(()) } else { Err(validation_errors) }
}

fn add_error<'a, T>(validation_errors: &mut FormValidationErrors<'a>, field: &'a str, error: FormValidationError) -> Option<T> {
    if validation_errors.contains_key(&field) {
        let mut err = validation_errors.get_mut(&field).unwrap();
        if !err.contains(&error) {
            err.push(error);
        }
    } else {
        validation_errors.insert(&field, vec!(error));
    }
    None
}

pub fn required(_: ValidatorFuncArgs) {
    println!("required");
    // required template
}

pub fn max(_: ValidatorFuncArgs) {
    println!("max");
    // max template
}
