use crate::ops::MapOp;
use crate::result::{SqlResult, SqlError};
use crate::table::Column;
use crate::result::ErrorType::{ Type};
use chrono::{NaiveDateTime, Datelike};

pub struct Year{}

impl MapOp for Year {
    fn apply(&self, arguments: Vec<&Column>) -> SqlResult<Column> {
        arg_check!(1, arguments, "year", !=);

        match &arguments[0] {
            Column::Dates(d) => {
                Ok(Column::Ints(d.into_iter().map(|maybe_timestamp| {
                    maybe_timestamp.map(|timestamp| NaiveDateTime::from_timestamp(timestamp, 0).year() as i64)
                }).collect()))
            }
            _ => Err(SqlError::new("year function can only be called on date time", Type))
        }
    }
}