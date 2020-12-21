use crate::column::Column;
use crate::scalar::Scalar;
use std::fmt;

pub enum Evaluated {
    Column(Column),
    Scalar(Scalar),
    Value(String),
}

#[derive(Debug)]
pub enum ErrorType {
    Syntax,
    Type,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.type_, self.message)
    }
}

pub type SqlResult<T> = std::result::Result<T, SqlError>;

