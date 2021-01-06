use crate::table::{Store, Table};
use crate::result::{SqlResult, SqlError};
use std::collections::HashMap;
use crate::result::ErrorType::{Lookup, Runtime};
use std::error::Error;

impl Store {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new()
        }
    }

    pub fn from_paths(csv_paths: Vec<String>) -> SqlResult<Self> {

        csv_paths.into_iter().map(|path| {
            Table::from_file(path.as_str()).map(|t| {
                (path, t)
            })
        }).collect::<std::io::Result<HashMap<String, Table>>>()
            .map(|tables| Self {tables})
            .map_err(|e| { SqlError::new(e.to_string().as_str(), Runtime)
        })

    }

    pub fn get(&self, alias: &str) -> SqlResult<&Table> {
        self.tables.get(alias).ok_or(
            SqlError::new(format!("alias {} not found in store", alias).as_str(), Lookup))
    }
}