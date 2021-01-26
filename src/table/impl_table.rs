use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead};
use rayon::prelude::*;
use std::path::Path;
use crate::build_column::build_column;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::{Lookup, Runtime};
use crate::table::{Table, Column, NamedColumn, TableMeta};
use crate::ingest::{SepFinder, read_line};

/// uses the filename minus the extension
fn extract_table_name(file_path: &str) -> Option<String> {
    Path::new(file_path).file_stem()?.to_str().map(|s| {
        s.to_string().replace(".", "_")
    })
}

impl Table {

    pub fn new() -> Self {
        Table {
            alias: "".to_string(),
            columns: Vec::new(),
            column_map: HashMap::new(),
            column_names: Vec::new(),
        }
    }

    pub fn alias(&self) -> String {
        self.alias.clone()
    }

    pub fn from_file(file_location: &str, separator: &Box<dyn SepFinder>) -> Result<Self, std::io::Error> {
        let f = File::open(file_location)?;

        let alias = extract_table_name(file_location)
            .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "could not parse table name from file"))?;

        let mut lines = std::io::BufReader::new(f).lines();

        let column_line = lines.next().ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "file is empty"))?;

        let column_names = parse_header_line(column_line?, separator);

        let column_map = create_column_map(&alias, &column_names);

        let mut raw_string_columns: Vec<Vec<String>> = vec![vec!(); column_names.len()];

        let mut line_counter = 0;
        for line in lines {
            let parsed = read_line(line?, separator);

            if parsed.len() < column_names.len() {
                println!("Parse Error: Line {}: {}", line_counter, parsed.join(","))
            } else {
                parsed.into_iter().enumerate().for_each(|(num, s)| {
                    raw_string_columns[num].push(s);
                });
            }

            line_counter += 1;
        }

        let columns = raw_string_columns.into_par_iter().map(
            build_column
        ).collect();

        Ok(Table {
            alias, column_map, column_names, columns
        })
    }
    /// roughly equivalent to a union operation
    pub fn from_tables(mut tables: Vec<Self>) -> SqlResult<Self> {
        let mut first = tables.pop().ok_or(SqlError::new("cannot build table from empty result set",
                                                     Runtime))?;

        for table in tables.into_iter() {
            for (num, c) in table.into_columns().into_iter().enumerate() {
                &first.columns[num].merge(c.column);
            }
        };

        Ok(first)
    }

    pub fn column(&self, table: &str, name: &str) -> Option<&Column> {
        let key = (table.to_string().to_lowercase(), name.to_string());
        let index = self.column_map.get(&key)?.clone();
        Some(&self.columns[index])
    }

    /// column search is a non fully qualified column access IE SELECT a FROM table
    /// as opposed to SELECT table.a FROM table
    pub fn column_search(&self, name: &str) -> SqlResult<&Column> {
        let mut index = 0;
        let mut found_once = false;
        for (num, column) in self.column_names.iter().enumerate() {
            if column.as_str() == name {
                if found_once {
                    return Err(SqlError::new("unqualified column name is ambiguous", Lookup))
                } else {
                    found_once = true;
                    index = num 
                }
            }
        }

        if !found_once {
            return Err(SqlError::new("column not found in table", Lookup))
        }

        Ok(&self.columns[index])
    }

    pub fn len(&self) -> usize {
        self.columns.iter().map(|c| c.len()).max().unwrap_or(0)
    }

    pub fn limit(&mut self, length: usize) {
        for i in 0..self.columns.len() {
            self.columns[i].limit(length);
        }
    }

    pub fn meta(&self) -> TableMeta {
        TableMeta {
            columns: self.columns.iter().zip( self.column_names.iter()).map(
                |(column, name)| {
                    (name.clone(), column.type_())
                }).collect(),
            length: self.len(),
            alias: self.alias.clone()
        }
    }

    pub fn push(&mut self, column: NamedColumn, table: Option<&str>) {
        let table_name = table.unwrap_or("");
        let name = column.name;
        let column = column.column;

        self.column_names.push(name.clone());
        self.columns.push(column);
        self.column_map.insert(
            (table_name.to_string(), name),
            self.columns.len() - 1
        );
    }

    pub fn into_columns(self) -> Vec<NamedColumn> {
        self.columns.into_iter().zip(self.column_names.into_iter()).map(|(column, name)| {
            NamedColumn {
                column, name
            }
        }).collect()
    }

    pub fn order_by(&mut self, order_vec: Vec<usize>) -> Self {
        let columns: Vec<Column> = self.columns.iter().map(|c| {
            c.order(&order_vec)
        }).collect();

        Self {
            columns,
            column_names: self.column_names.clone(),
            alias: self.alias.clone(),
            column_map: self.column_map.clone(),
        }
    }

    pub fn where_(&self, mask: Vec<bool>) -> Self {
        let columns: Vec<Column> = self.columns.iter().map(|c| {
            c.select(&mask)
        }).collect();

        Self {
            columns,
            column_names: self.column_names.clone(),
            alias: self.alias.clone(),
            column_map: self.column_map.clone(),
        }
    }
}

// trim string from white spaces, also replace "|' from first and last characters
fn clean(raw: &str) -> String {
    let mut s: String = raw.trim()
        .chars()
        .skip_while(|c| {
            c.eq(&'\"') || c.eq(&'\'')
        })
        .collect();

    while s.ends_with(&"\"") || s.ends_with(&"\'") {
        s.truncate(s.len() - 1)
    }

    s
}


fn parse_header_line(header_line: String, separator: &Box<dyn SepFinder>) -> Vec<String> {
    read_line(header_line, separator).into_iter().enumerate().map(|(num, s)| {
        if s.len() == 0 {
            num.to_string()
        } else {
            s.to_lowercase().replace(".", "_")
        }
    }).collect()
}

fn create_column_map(table_name: &String, column_names: &Vec<String>) -> HashMap<(String, String), usize> {
    column_names.iter().enumerate().map(|(index, name)| {
        ((table_name.clone(), name.clone()), index)
    }).collect()
}


#[cfg(test)]
mod test {


    use crate::table;
    use crate::table::impl_table::clean;
    use crate::ingest::{CsvFinder, SepFinder};

    #[test]
    fn test_string_clean() {
        time_test!();

        let something = "\"something\"";
        let something_else = "'yep'";
        let with_spaces = "    spaces man";

        assert_eq!(clean(something), "something".to_string());
        assert_eq!(clean(something_else), "yep".to_string());
        assert_eq!(clean(with_spaces), "spaces man".to_string());
    }

    #[test]
    fn build_table() {
        time_test!();

        let parsed_table = table::Table::from_file("test/data.csv", &(Box::new(CsvFinder{}) as Box<dyn SepFinder>));

        match parsed_table {
            Ok(t) => assert!(t.len() > 0),
            Err(_) => assert!(false),
        }

    }
}