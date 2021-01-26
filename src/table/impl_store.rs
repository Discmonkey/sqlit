use crate::table::{Store, Table};
use crate::result::{SqlResult, SqlError};
use std::collections::HashMap;
use crate::result::ErrorType::{Lookup};
use std::io;
use crate::ingest::SepFinder;

impl Store {

    pub fn from_paths(csv_paths: Vec<String>, separator: &Box<dyn SepFinder>) -> io::Result<Self> {

        csv_paths.into_iter().map(|path| {
            Table::from_file(path.as_str(), separator).map(|t| {
                (t.alias(), t)
            })
        }).collect::<std::io::Result<HashMap<String, Table>>>().map(|tables| Self {tables})
    }

    pub fn get(&self, alias: &str) -> SqlResult<&Table> {
        self.tables.get(alias).ok_or(
            SqlError::new(format!("alias {} not found in store", alias).as_str(), Lookup))
    }

    pub fn list(&self) -> Vec<&Table> {
        self.tables.values().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::table::Store;
    use crate::result::SqlResult;
    use crate::ingest::{CsvFinder, SepFinder};

    #[test]
    fn test_get() -> std::io::Result<()>{
        let mut s = Store::from_paths(vec!["test/nba.games.stats.csv".to_string()], &(Box::new(CsvFinder{}) as Box<dyn SepFinder>))?;


        match s.get("nba_games_stats") {
            Ok(t) => assert!(true),
            Err(e) => assert!(false),
        };

        Ok(())
    }
}