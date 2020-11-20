use crate::column;
use crate::column::{Type, Value, Column};
use regex::Regex;
use chrono::{DateTime, NaiveDateTime, NaiveDate};

trait Converter {
    fn convert(&mut self, field: &str) -> Option<column::Value>;
    fn column_type(&self) ->  column::Type;
}

pub struct ToString {}

impl Converter for ToString {
    fn convert(& mut self, field: &str) -> Option<column::Value> {
        return Some(column::Value::String(field.to_owned()))
    }

    fn column_type(&self) -> Type {
        return Type::String;
    }
}

pub struct ToInt {}
pub struct ToFloat {}
pub struct ToBool {}

macro_rules! make_default_converter {
    ($name: tt, $type: tt, $enum: tt) => {
        impl Converter for $name {
            fn convert(&mut self, field: &str) -> Option<Value> {
                match field.parse::<$type>() {
                    Err(e) => {
                        println!("{}", e);

                        None
                    },
                    Ok(i) => Some(Value::$enum(i))
                }
            }

            fn column_type(&self) -> Type {
                return column::Type::$enum;
            }
        }
    }
}

make_default_converter!(ToInt, i64, Int);
make_default_converter!(ToFloat, f64, Float);
make_default_converter!(ToBool, bool, Boolean);

pub struct ToDate {
    valid_format: String,
    valid_format_found: bool,
}

impl ToDate {
    pub fn new() -> Self {
        return ToDate{
            valid_format: "".to_owned(),
            valid_format_found: false,
        }
    }
}

impl Converter for ToDate {
    fn convert(&mut self, field: &str) -> Option<Value> {
        if self.valid_format_found {
            match NaiveDate::parse_from_str(field, self.valid_format.as_str()) {
                Err(_) => None,
                Ok(d) => Some(Value::Date(d.and_hms(0, 0, 0).timestamp()))
            }
        } else {
            match DateTime::parse_from_rfc2822(field) {
                Err(_) => (),
                Ok(d) => return Some(Value::Date(d.timestamp())),
            };

            match DateTime::parse_from_rfc3339(field) {
                Err(_) => (),
                Ok(d) => return Some(Value::Date(d.timestamp())),
            }

            // we should only hit this part once
            let formats = vec!("%y-%m-%d", "%Y-%m-%d", "%m/%d/%Y");

            for format in formats {
                match NaiveDate::parse_from_str(field, format) {
                    Err(_) => (),
                    Ok(d) => {
                        self.valid_format = format.to_owned();
                        self.valid_format_found = true;

                        return Some(Value::Date(d.and_hms(0, 0, 0).timestamp()));
                    }
                }
            }

            None
        }
    }

    fn column_type(&self) -> Type {
        return column::Type::Date;
    }
}

#[cfg(test)]
mod test {
    use crate::converters::{ToFloat, ToInt, Converter, ToBool, ToDate};
    use crate::column::Value;

    #[test]
    fn convert_int() {
        let source = "12";

        let mut float_converter = ToFloat{};
        let mut int_converter = ToInt{};

        match int_converter.convert(source) {
            None => assert!(false),
            Some(Value::Int(i)) => assert_eq!(i, 12),
            _ => assert!(false),
        }

        match float_converter.convert(source) {
            None => assert!(false),
            Some(Value::Float(f)) => assert_eq!(f, 12.0),
            _ => assert!(false),
        }
    }

    #[test]
    fn convert_float() {
        let source = "12.56";

        let mut float_converter = ToFloat{};
        let mut int_converter = ToInt{};

        match int_converter.convert(source) {
            None => assert!(true),
            Some(Value::Int(i)) => assert!(false),
            _ => assert!(false),
        }

        match float_converter.convert(source) {
            None => assert!(false),
            Some(Value::Float(f)) => assert_eq!(f, 12.56),
            _ => assert!(false),
        }
    }

    #[test]
    fn convert_bool() {
        let true_source = "true";
        let false_source = "false";
        let true_as_int = "1";
        let false_as_int = "0";
        let false_as_non_one = "2";

        let mut c = ToBool{};

        match c.convert(true_source) {
            None => assert!(false),
            Some(Value::Boolean(b)) => assert!(b),
            _ => assert!(false),
        }

        match c.convert(false_source) {
            None => assert!(false),
            Some(Value::Boolean(b)) => assert!(!b),
            _ => assert!(false),
        }

        match c.convert(true_as_int) {
            None => assert!(true),
            Some(Value::Boolean(b)) => assert!(false),
            _ => assert!(false),
        }

        match c.convert(false_as_int) {
            None => assert!(true),
            Some(Value::Boolean(b)) => assert!(false),
            _ => assert!(false),
        }

        match c.convert(false_as_non_one) {
            None => assert!(true),
            Some(Value::Boolean(b)) => assert!(false),
            _ => assert!(false),
        }

    }

    #[test]
    fn convert_date() {
        let date = "2020-12-08";

        let mut converter = ToDate::new();

        match converter.convert(date) {
            Some(Value::Date(d)) => assert!(true),
            _ => assert!(false),
        }
    }
}