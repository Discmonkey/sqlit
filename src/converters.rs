use crate::column;
use crate::column::{Type, Value, Column};
use regex::Regex;
use chrono::{DateTime, NaiveDateTime, NaiveDate};

fn result_to_option<T, E>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(v) => Some(v),
        Err(_) => None
    }
}

pub trait Converter<T> {
    fn convert(&mut self, field: &str) -> Option<T>;
    fn make_column(&self, values: Vec<T>) ->  column::Column;
}

pub struct ToString {}

impl Converter<String> for ToString {
    fn convert(& mut self, field: &str) -> Option<String> {
        Some(field.to_owned())
    }

    fn make_column(&self, values: Vec<String>) -> Column {
        column::Column::Strings(values)
    }
}

pub struct ToInt {}
pub struct ToFloat {}
pub struct ToBool {}

macro_rules! make_default_converter {
    ($name: tt, $type: tt, $enum: tt) => {
        impl Converter<$type> for $name {
            fn convert(&mut self, field: &str) -> Option<$type> {
                result_to_option(field.parse::<$type>())
            }

            fn make_column(&self, values: Vec<$type>) -> Column {
                column::Column::$enum(values)
            }
        }
    }
}

make_default_converter!(ToInt, i64, Ints);
make_default_converter!(ToFloat, f64, Floats);
make_default_converter!(ToBool, bool, Booleans);

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

impl Converter<column::DateTime> for ToDate {
    fn convert(&mut self, field: &str) -> Option<column::DateTime> {
        if self.valid_format_found {
            result_to_option(NaiveDate::parse_from_str(field, self.valid_format
                .as_str()).map(|v| {v.and_hms(0, 0, 0).timestamp()}))
        } else {
            match DateTime::parse_from_rfc2822(field) {
                Err(_) => (),
                Ok(d) => return Some(d.timestamp()),
            };

            match DateTime::parse_from_rfc3339(field) {
                Err(_) => (),
                Ok(d) => return Some(d.timestamp()),
            }

            // we should only hit this part once
            let formats = vec!("%y-%m-%d", "%Y-%m-%d", "%m/%d/%Y");

            for format in formats {
                match NaiveDate::parse_from_str(field, format) {
                    Err(_) => (),
                    Ok(d) => {
                        self.valid_format = format.to_owned();
                        self.valid_format_found = true;

                        return Some(d.and_hms(0, 0, 0).timestamp());
                    }
                }
            }

            None
        }
    }

    fn make_column(&self, values: Vec<i64>) -> Column {
        column::Column::Dates(values)
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
            Some(i) => assert_eq!(i, 12),
            _ => assert!(false),
        }

        match float_converter.convert(source) {
            None => assert!(false),
            Some(f) => assert_eq!(f, 12.0),
        }
    }

    #[test]
    fn convert_float() {
        let source = "12.56";

        let mut float_converter = ToFloat{};
        let mut int_converter = ToInt{};

        match int_converter.convert(source) {
            None => assert!(true),
            Some(i) => assert!(false),
        }

        match float_converter.convert(source) {
            None => assert!(false),
            Some(f) => assert_eq!(f, 12.56),
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
            Some(b) => assert!(b),
        }

        match c.convert(false_source) {
            None => assert!(false),
            Some(b) => assert!(!b),
        }

        match c.convert(true_as_int) {
            None => assert!(true),
            Some(b) => assert!(false),
        }

        match c.convert(false_as_int) {
            None => assert!(true),
            Some(b) => assert!(false),
        }

        match c.convert(false_as_non_one) {
            None => assert!(true),
            Some(b) => assert!(false),
        }

    }

    #[test]
    fn convert_date() {
        let date = "2020-12-08";

        let mut converter = ToDate::new();

        match converter.convert(date) {
            Some(d) => assert!(true),
            _ => assert!(false),
        }
    }
}