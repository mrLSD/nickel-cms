// FormValidator.validators
// - set validators rules
use nickel::{
    Params,
};
use std;
use std::collections::HashMap;

macro_rules! parse_fn {
    ( $vtype:ty ) => {{
        fn parse(x: &str) -> Option<i32> {
            if x.parse::<$vtype>().is_ok() { Some(1) } else { None }
        }
        parse
    }};
}

macro_rules! parse_field {
    ( $field:expr, $vtype:ty ) => {{
        $field.parse::<$vtype>()
    }};
}

macro_rules! validators {
    ( $([$($field:expr),*]: $vtype:ty => $($func:ident$args:tt),*; )+ ) => {{
        let mut rules: ValidatorParams = Vec::new();
        fn type_ref_parser(vtype: i32){

        }//--
        $({
            let rule = ValidatorParam {
                fields: vec![$($field),*],
                validators: vec![$($func),*],
                type_ref: stringify!($vtype),
                type_ref_fn: parse_fn!(i32),
                validators_arg: validators! ( @validator_func $($args),* ),
            };
            rules.push(rule);
        })+
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
    pub type_ref_fn: fn(&str) -> Option<i32>,
    //--
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
        Validators::new(Self::validators()).create_validator(&form_data);
        Ok(Self::fill_form(&form_data)?)
    }
    // Validation rules and validators
    fn validators<'a>() -> ValidatorParams<'a>;
    // fill form from form_data to struct
    // when invoked method all data already correct and validated
    fn fill_form(form_data: &Params)
            -> Result<Self, FormValidationErrors>
            where Self: std::marker::Sized;
}

impl<'a> Validators<'a> {
    fn new(validators: ValidatorParams<'a>) -> Validators<'a> {
        Validators { validators: validators }
    }

    fn create_validator(&self, form_data: &Params) -> Result<(), FormValidationErrors> {
        let mut validation_errors: FormValidationErrors = HashMap::new();
        let mut validation_warnings: FormValidationErrors = HashMap::new();

        for rules in &self.validators {
            println!("Type ref: {:#?}", rules.type_ref);
            for (key, ref validator) in rules.validators.iter().enumerate() {
                let args = rules.validators_arg[key].to_owned();
                for field_name in &rules.fields {
                    println!("Fields: {:#?}", field_name);
                    println!("Get: {:#?}", form_data.all(field_name));
                    println!("Args: {:#?}\n\t", args);

                    // Fetch fields withour any results
                    // We iterate over all fields with `field_name`
                    // Try parse with specific type
                    // We gathering all possible errors, without skiping any posible error
                    form_data.all(field_name)
                        // If field not exist - add error
                        .or_else(|| {
                            add_error(&mut validation_warnings, field_name, FormValidationError::FieldNotExist);
                            None
                        })
                        .and_then(|v| {
                            // Iterate over fields
                            Some(v.iter()
                                .map(|x| {
                                    // Type inference via Parse
                                    (rules.type_ref_fn)(x).ok_or(0).ok()
                                        .and_then(|v| {
                                            println!("Parsed: OK");
                                            validator(args.to_owned());
                                            Some(v)
                                        })
                                        // If parse Error - add error
                                        .or_else(|| {
                                            add_error(&mut validation_errors, field_name, FormValidationError::ParseError);
                                            None
                                        });
                                    // Return same field
                                    x
                                }).cloned().collect::<String>())
                        });
                }
            }
        }

        println!("Warnings: {:#?}", validation_warnings);
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
}

fn add_error<'a>(validation_errors: &mut FormValidationErrors<'a>, field: &'a str, error: FormValidationError) {
    if validation_errors.contains_key(&field) {
        let mut err = validation_errors.get_mut(&field).unwrap();
        if !err.contains(&error) {
            err.push(error);
        }
    } else {
        validation_errors.insert(&field, vec!(error));
    }
}

pub fn required(_: ValidatorFuncArgs) {
    println!("required");
    // required template
}

pub fn max(_: ValidatorFuncArgs) {
    println!("max");
    // max template
}
