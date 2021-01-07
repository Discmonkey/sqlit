use crate::table::Column;
use std::fmt;
use std::error::Error;

pub enum Evaluated {
    Column(Column),
    Value(String),
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Lookup,
    Runtime,
    Syntax,
    Type,
}

#[derive(Debug, Clone)]
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

impl Error for SqlError{}

pub type SqlResult<T> = std::result::Result<T, SqlError>;

