use std::collections::HashMap;
use crate::column;
use std::fs::File;
use std::io::{BufRead};
use rayon::prelude::*;
use crate::build_column::build_column;

pub struct Table {
    column_list: Vec<String>,
    column_map: HashMap<String, usize>,
    columns: Vec<column::Column>,
    num_rows: usize,
}

// trim string from white spaces, also replace "|' from first and last characters
fn clean(raw: &str) -> String {
    let mut s: String = raw.trim()
        .chars()
        .skip_while(|c| {
            c.eq(&'\"') || c.eq(&'\'')
        })
        .collect();

    if s.ends_with(&"\"") || s.ends_with(&"\'") {
        s.truncate(s.len() - 1)
    }

    s
}

fn parse_line(header_line: String) -> Vec<String> {
    header_line.split(",").map(clean).collect()
}

impl Table {

    pub fn from_file(file_location: &str) -> Result<Self, std::io::Error> {
        let f = File::open(file_location)?;

        let mut lines = std::io::BufReader::new(f).lines();
        let maybe_column_line = lines.next();

        if let None = maybe_column_line {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "file is empty"));
        }

        let header_line = maybe_column_line.unwrap()?;

        let column_list: Vec<String> = parse_line(header_line).into_iter().enumerate().map(|(num, s)| {
            if s.len() == 0 {
                num.to_string()
            } else {
                s
            }
        }).collect();

        let column_map: HashMap<String, usize> = column_list.iter().enumerate().map(|(index, name)| {
            (name.clone(), index)
        }).collect();

        let mut raw_string_columns: Vec<Vec<String>> = column_list.iter().map(|_| Vec::new()).collect();

        let mut num_rows = 0;
        for line in lines {
            num_rows += 1;
            parse_line(line?).into_iter().enumerate().for_each(|(num, s)| {
                raw_string_columns[num].push(s);
            });
        }

        let columns = raw_string_columns.into_par_iter().map(
            build_column
        ).collect();


        Ok(Table {
            column_map, column_list, columns, num_rows
        })


    }


    pub fn len(&self) -> usize {
        self.num_rows
    }

}


#[cfg(test)]
mod test {


    use crate::table;
    use crate::table::clean;

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

        let parsed_table = table::Table::from_file("test/nba.games.stats.csv");

        match parsed_table {
            Ok(t) => assert!(t.len() > 0),
            Err(_) => assert!(false),
        }

    }
}