use std::fmt;
use std::error::Error;
use crate::result::ErrorType::{Runtime, Lookup, IO};


#[derive(Debug, Clone)]
pub enum ErrorType {
    Lookup,
    Runtime,
    IO,
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
        let message = format!("{} expects {} argument(s) but received {} argument(s)", function_name, num_args_expected, num_args_received);

        SqlError {
            type_: Runtime,
            message
        }
    }

    pub fn look_up_error(key: &str, store: &str) -> Self {
        let message = format!("{} not found in {}", key, store);

        SqlError {
            type_: Lookup,
            message
        }
    }

    pub fn io_error(message: &str) -> Self {
        SqlError {
            type_: IO,
            message: message.to_string()
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

