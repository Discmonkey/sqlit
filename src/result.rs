use crate::column::Column;
use std::fmt;

pub enum Evaluated {
    Column(Column),
    Value(String),
}

#[derive(Debug)]
pub enum ErrorType {
    Syntax,
    Type,
    Lookup,
}

pub struct SqlError {
    type_: ErrorType,
    message: String,
}

impl SqlError {
    pub fn new(message: &str, error_type: ErrorType) -> Self {
        SqlError {
            type_: error_type, message: message.to_string()
        }
    }
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} Error: {}", self.type_, self.message)
    }
}

pub type SqlResult<T> = std::result::Result<T, SqlError>;

