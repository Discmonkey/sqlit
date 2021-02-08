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
use std::rc::Rc;

/// uses the filename minus the extension
fn extract_table_name(file_path: &str) -> Option<String> {
    Path::new(file_path).file_stem()?.to_str().map(|s| {
        s.to_string().replace(".", "_")
    })
}

impl Table {

    /// Reads file into table
    pub fn from_file(file_location: &str, separator: &Box<dyn SepFinder>, null: &str) -> Result<Self, std::io::Error> {
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

        let columns: Vec<Column> = raw_string_columns.into_par_iter().map(|s| {
            build_column(s, null)
        }).collect();

        Ok(Table {
            alias, column_map, column_names, columns: columns.into_iter().map(|c| Rc::new(c)).collect()
        })
    }

    /// Union all table and return the result
    pub fn from_tables(mut tables: Vec<Self>) -> SqlResult<Self> {

        if tables.len() > 0 {
            let first = Ok(tables.pop().unwrap());

            tables.into_iter().fold(first, |acc, next| {
                match acc {
                    Err(e) => Err(e),
                    Ok(c) => c.merge(&next)
                }
            })

        } else {
            Ok(Self::new())
        }
    }

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

    pub fn column(&self, table: &str, name: &str) -> Option<Rc<Column>> {
        let key = (table.to_string().to_lowercase(), name.to_string());
        let index = self.column_map.get(&key)?.clone();

        Some(self.columns[index].clone())
    }

    /// Non fully-qualified column access IE SELECT a FROM table
    /// as opposed to SELECT table.a FROM table
    /// throws an error if the column is not found or if the column name is ambiguous
    pub fn column_search(&self, name: &str) -> SqlResult<Rc<Column>> {
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
            Err(SqlError::look_up_error(name, "table"))
        } else {
            Ok(self.columns[index].clone())
        }
    }

    pub fn into_columns(self) -> Vec<NamedColumn> {
        self.columns.into_iter().zip(self.column_names.into_iter()).map(|(column, name)| {
            NamedColumn {
                column, name
            }
        }).collect()
    }

    pub fn len(&self) -> usize {
        self.columns.iter().map(|c| c.len()).max().unwrap_or(0)
    }

    pub fn limit(&mut self, length: usize) -> Self {
        Self {
            alias: self.alias.clone(),
            column_names: self.column_names.clone(),
            columns: self.columns.iter().map(|c| {
                Rc::new(c.limit(length))
            }).collect(),
            column_map: self.column_map.clone()
        }
    }

    pub fn merge(&self, other: &Self) -> SqlResult<Self> {
        let columns = self.columns.iter().zip(other.columns.iter()).map(|(c0, c1)| {
            c0.merge(c1).map(|c| Rc::new(c))
        }).collect::<SqlResult<Vec<Rc<Column>>>>()?;

        let column_names = self.column_names.clone();
        let column_map = self.column_map.clone();

        Ok(Self {
            columns, alias: self.alias.clone(), column_names, column_map
        })
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
        let table_name = table.unwrap_or(&self.alias);
        let name = column.name;
        let column = column.column;

        self.column_names.push(name.clone());
        self.columns.push(column);
        self.column_map.insert(
            (table_name.to_string(), name),
            self.columns.len() - 1
        );
    }

    pub fn order_by(&self, order_vec: Vec<usize>) -> Self {
        let columns: Vec<Rc<Column>> = self.columns.iter().map(|c| {
            Rc::new(c.order(&order_vec))
        }).collect();

        Self {
            columns,
            column_names: self.column_names.clone(),
            alias: self.alias.clone(),
            column_map: self.column_map.clone(),
        }
    }

    pub fn with_new_alias(&self, alias: String) -> Self {

        let mut empty = Self::new();

        self.columns.iter().zip(self.column_names.iter()).for_each(|(column, name)| {
            empty.push(NamedColumn {
                column: column.clone(), name: name.clone()
            }, Some(alias.as_str()))
        });

        empty
    }

    pub fn where_(&self, mask: &Vec<Option<bool>>) -> Self {
        let columns: Vec<Rc<Column>> = self.columns.iter().map(|c| {
            Rc::new(c.select(&mask))
        }).collect();

        Self {
            columns,
            column_names: self.column_names.clone(),
            alias: self.alias.clone(),
            column_map: self.column_map.clone(),
        }
    }
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
    use crate::ingest::{CsvFinder, SepFinder};


    #[test]
    fn build_table() {
        time_test!();

        let parsed_table = table::Table::from_file("tests/data/music.csv", &(Box::new(CsvFinder{}) as Box<dyn SepFinder>), "null");

        match parsed_table {
            Ok(t) => assert!(t.len() > 0),
            Err(_) => assert!(false),
        }

    }
}