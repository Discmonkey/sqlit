use crate::table::{Table, Column};
use std::io::Write;
use std::fmt::Display;
use std::cmp::max;
use std::path::Path;
use std::fs::File;
use chrono::NaiveDateTime;
use crate::result::{SqlResult, SqlError};
use std::error::Error;

fn item_width(dest: &mut Vec<u8>, writable: &dyn Display) -> std::io::Result<usize> {
    dest.truncate(0);

    write!(dest, "{}", writable)?;

    Ok(dest.len())
}

macro_rules! maybe_length {
    ($item:ident, $length: ident, $dest: ident) => {
        if let Some(unwrapped) = $item {
            max(item_width(&mut $dest, unwrapped)?, $length)
        } else {
            max(4, $length)
        }
    }
}


fn find_column_width(col: &Column, name: &String) -> std::io::Result<usize> {
    let mut scratch = Vec::new();
    let name_print_width = item_width(&mut scratch, name)?;

    macro_rules! find_max {
        ($vector: ident, $scratch: ident, |$value: ident| $block:block) => {
            $vector.iter().map(|maybe_value| {
                maybe_value.as_ref().map(|$value| {
                    item_width(&mut $scratch, $block).unwrap_or(0)
                }).unwrap_or(0)
            }).max().unwrap_or(0);
        }
    }
    let max_width_column = match col {
        Column::Strings(s) => {
            find_max!(s, scratch, |string| {&string})
        },

        Column::Ints(i) => {
            find_max!(i, scratch, |int| {&int})
        },

        Column::Floats(f) => {
            find_max!(f, scratch, |float| {&float})
        },

        Column::Dates(d) => {
            find_max!(d, scratch, |timestamp| {
                &NaiveDateTime::from_timestamp(timestamp.clone(), 0).to_string()
            })
        },

        Column::Booleans(_) => {
            5
        }
    };

    // add two spaces of padding
    // may be worth making configurable
    Ok(max(name_print_width, max_width_column) + 2)

}

fn write_entry(f: &mut std::fmt::Formatter, lengths: &Vec<usize>,
               col: usize, maybe_writable: Option<&dyn Display>, scratch: &mut Vec<u8>) -> std::fmt::Result {

    let null_print = "NULL".to_string();

    let writable = if let Some(writable) = maybe_writable {
        writable as &dyn Display
    } else {
        &null_print as &dyn Display
    };

    let mut write_width = item_width(scratch, writable).map_err(|_| std::fmt::Error::default())?;

    write!(f, "{}", writable)?;

    while write_width < lengths[col] {
        write!(f, " ")?;
        write_width += 1;
    }

    Ok(())
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let column_print_widths = self.columns.iter().zip(self.column_names.iter()).map(|(c, s)| {
            find_column_width(c, s)
        }).collect::<std::io::Result<Vec<usize>>>().map_err(|_| {
            std::fmt::Error::default()
        })?;

        let max_length = self.columns.iter().map(|c| c.len()).max().unwrap_or(0);
        let mut scratch = Vec::new();

        for (num, name) in self.column_names.iter().enumerate() {
            write_entry(f, &column_print_widths, num, Some(name), &mut scratch)?;
        }

        writeln!(f)?;

        for i in 0..max_length {
            for (num, col) in self.columns.iter().enumerate() {
                let mut index = i;
                if i > col.len() {
                    index = col.len() - 1;
                }

                macro_rules! as_display {
                    ($expr: expr) => {
                       $expr.as_ref().map(|opt| opt as &dyn std::fmt::Display)
                    }
                }

                match col.as_ref() {
                    Column::Strings(s) => write_entry(f,
                                                      &column_print_widths,
                                                      num, as_display!(s[index]),
                                                      &mut scratch)?,

                    Column::Ints(i) => write_entry(f,
                                                   &column_print_widths,
                                                   num,
                                                   as_display!(i[index]),
                                                   &mut scratch)?,

                    Column::Floats(floats) => write_entry(f,
                                                          &column_print_widths,
                                                          num,
                                                          as_display!(floats[index]),
                                                          &mut scratch)?,

                    Column::Dates(d) => write_entry(f,
                                                    &column_print_widths,
                                                    num,
                                                    as_display!(d[index].map(|t| NaiveDateTime::from_timestamp(t.clone(), 0).to_string())),
                                                    &mut scratch)?,

                    Column::Booleans(b) => write_entry(f,
                                                       &column_print_widths,
                                                       num, as_display!(b[index]),
                                                       &mut scratch)?,
                }
            }

            if i != max_length - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Table {
    pub fn write_to_file(&self, filename: &str) -> SqlResult<()> {
        let path = Path::new(filename);

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = File::create(filename).map_err(|e| SqlError::io_error(&e.to_string()))?;

        let mut separator = if filename.ends_with(".tsv") {
            "\t"
        } else {
            ","
        }.to_string();

        for (i, s) in self.column_names.iter().enumerate() {
            if i > 0 {
                file.write(separator.as_bytes());
            }
            file.write(s.as_bytes());
        }

        let quote = "\"".to_string();

        for i in 0..self.len() {

            file.write("\n".as_bytes());

            for (num, column) in self.columns.iter().enumerate() {
                if num > 0 {
                    file.write(separator.as_bytes());
                }
                file.write(quote.as_bytes());
                column.as_writable(i, &mut file, "NULL");
                file.write(quote.as_bytes());

            }
        }

        Ok(())
    }
}