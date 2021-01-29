use std::fmt;
use std::error::Error;
use crate::result::ErrorType::Runtime;


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

    pub fn args_length_error(num_args_expected: usize,
                             num_args_received: usize, function_name: &str) -> Self {
        let message = format!("{} expects {} but received {}", function_name, num_args_expected, num_args_received);

        SqlError {
            type_: Runtime,
            message
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

