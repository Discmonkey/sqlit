use crate::ops::MapOp;
use crate::result::{SqlResult, SqlError};
use crate::table::Column;
use crate::result::ErrorType::{Syntax, Type};
use chrono::{NaiveDateTime, Datelike};

pub struct Year{}

impl MapOp for Year {
    fn apply(&self, mut arguments: Vec<Column>) -> SqlResult<Column> {
        if arguments.len() != 1 {
            Err(SqlError::new("year method takes exactly one argument", Syntax))
        } else {
            match arguments.pop().unwrap() {
                Column::Dates(d) => {
                    Ok(Column::Ints(d.into_iter().map(|timestamp| {
                        NaiveDateTime::from_timestamp(timestamp, 0).year() as i64
                    }).collect()))
                }
                _ => Err(SqlError::new("year function can only be called on date time", Type))
            }
        }
    }
}