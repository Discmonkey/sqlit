use crate::converters::{Converter, ToBool, ToDate, ToFloat, ToInt};
use crate::column;
use crate::column::Column;

/// Converts raw values Vec<String> into a column
pub fn build_column(raw_values: Vec<String>) -> column::Column {
    if let Some(converted_column) = convert_into_column(&raw_values, Box::new(ToBool{})) {
        converted_column
    } else if let Some(converted_column) = convert_into_column(&raw_values, Box::new(ToDate::new())) {
        converted_column
    } else if let Some(converted_column) = convert_into_column(&raw_values, Box::new(ToInt{})) {
        converted_column
    } else if let Some(converted_column) = convert_into_column(&raw_values, Box::new(ToFloat{})) {
        converted_column
    } else {
        column::Column::Strings(raw_values)
    }
}

fn convert_into_column<T>(raw_values: &Vec<String>, mut converter: Box<dyn Converter<T>>) -> Option<Column> {
    let mut target = vec!();

    for raw_value in raw_values {
        let value = converter.convert(raw_value.as_str())?;

        target.push(value);
    }

    Some(converter.make_column(target))
}


#[cfg(test)]
mod test {
    use crate::build_column::build_column;
    use crate::column;
    #[test]
    fn build_booleans() {
        let raw_booleans = vec!("true", "false", "false").iter().map(|v| v.to_string()).collect();

        let column = build_column(raw_booleans);

        match column {
            column::Column::Booleans(v) => {
                assert!(v[0]);
                assert!(!v[1]);
                assert!(!v[2]);
            }

            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn build_booleans_fail() {
        let raw_booleans = vec!("true", "false", "233").iter().map(|v| v.to_string()).collect();

        let column = build_column(raw_booleans);

        match column {
            column::Column::Strings(_) => {
                assert!(true)
            }

            _ => {
                assert!(false)
            }
        }
    }

}
