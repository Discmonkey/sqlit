use crate::column::Column;
use crate::scalar::Scalar;

pub enum Evaluated {
    Column(Column),
    Scalar(Scalar),
    Value(String),
}

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

pub type SqlResult<T> = std::result::Result<T, SqlError>;

