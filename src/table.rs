use std::collections::HashMap;
use crate::column;


pub struct Table {
    columns: HashMap<String, usize>,
    values: Vec<column::Column>,
    rows: usize,
}


impl Table {

    // pub fn from_file(file_location: &str) -> Result<Self, std::io::Error> {
    //     let f = File::open(file_location)?;
    //
    //     let mut lines = std::io::BufReader::new(f).lines();
    //     let maybe_column_line = lines.next();
    //
    //     if let None = maybe_column_line {
    //         return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "file is empty"));
    //     }
    //
    //     let header_line = maybe_column_line.unwrap()?;
    //
    //     let columns = parse_header(header_line);
    // }


    pub fn rows(&self) -> usize {
        self.rows
    }

}

fn parse_header(header_line: String) -> Vec<String> {
    header_line.split(",").map(|s| { s.to_string().trim().to_string() }).collect()
}
