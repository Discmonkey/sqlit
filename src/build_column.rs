use crate::converters::{Converter, ToBool, ToDate, ToFloat, ToInt};
use crate::table::Column;

/// Converts raw values Vec<String> into a column
///
/// params:
///
/// raw_values -> a vector of Strings, from which we convert into columns
/// null_as_string -> the string value of null, which will be checked in each converter
pub fn build_column(raw_values: Vec<String>, null_as_string: &str) -> Column {

    if let Some(converted_column) = convert_into_column(&raw_values, Box::new(ToBool{}), null_as_string) {
        converted_column
    } else if let Some(converted_column) = convert_into_column(&raw_values, Box::new(ToDate::new()), null_as_string) {
        converted_column
    } else if let Some(converted_column) = convert_into_column(&raw_values,
                                                               Box::new(ToInt{}), null_as_string) {
        converted_column
    } else if let Some(converted_column) = convert_into_column(&raw_values,
                                                               Box::new(ToFloat{}), null_as_string) {
        converted_column
    } else {
        Column::Strings(raw_values.into_iter().map(|s| {
            if &s == null_as_string {
                None
            } else {
                Some(s)
            }
        }).collect())
    }
}

fn convert_into_column<T>(raw_values: &Vec<String>, mut converter: Box<dyn Converter<T>>,
                          null_as_string: &str) -> Option<Column> {
    let mut target = vec!();

    for raw_value in raw_values {


        let value = if raw_value == null_as_string {
            None
        } else {
            // if the converter fails on a null value then we reject the column from being this type
            Some(converter.convert(raw_value.as_str())?)
        };

        target.push(value);
    }

    Some(converter.make_column(target))
}


#[cfg(test)]
mod test {
    use crate::build_column::build_column;
    use crate::table::Column;
    #[test]
    fn build_booleans() {
        let raw_booleans = vec!("true", "false", "false").iter().map(|v| v.to_string()).collect();

        let column = build_column(raw_booleans, "nan");

        match column {
            Column::Booleans(v) => {
                assert!(v[0].unwrap());
                assert!(!v[1].unwrap());
                assert!(!v[2].unwrap());
            }

            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn build_booleans_fail() {
        let raw_booleans = vec!("true", "false", "233").iter().map(|v| v.to_string()).collect();

        let column = build_column(raw_booleans, "nan");

        match column {
            Column::Strings(_) => {
                assert!(true)
            }

            _ => {
                assert!(false)
            }
        }
    }

}
