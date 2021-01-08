use crate::table::{Table, Column};
use std::io::Write;
use crate::parser::ParserNodeType::Columns;
use std::fmt::Display;
use std::cmp::max;
use chrono::NaiveDateTime;
use rayon::prelude::*;

fn item_width(dest: &mut Vec<u8>, writable: &dyn Display) -> usize {
    dest.truncate(0);

    write!(dest, "{}", writable);

    dest.len()
}

fn find_column_width(col: &Column, name: &String) -> std::io::Result<usize> {
    let mut dest = Vec::new();
    let mut max_length = item_width(&mut dest, name);

    match col {
        Column::Strings(s) => {
            s.iter().for_each(|string| {
                max_length = max(item_width(&mut dest, string), max_length);
            });
        },

        Column::Ints(i) => {
            i.iter().for_each(|int| {
                max_length = max(item_width(&mut dest, int), max_length);
            });
        },

        Column::Floats(f) => {
            f.iter().for_each(|float| {
                max_length = max(item_width(&mut dest, float), max_length);
            });
        },

        Column::Dates(d) => {
            d.iter().for_each(|timestamp| {
                max_length = max(item_width(&mut dest, &NaiveDateTime::from_timestamp(timestamp.clone(), 0).to_string()), max_length);
            });
        },

        Column::Booleans(_) => {
            // max(len("false"), len("true")) == 5
            max_length = max(max_length, 5);
        }
    }

    // add two spaces of padding
    // may be worth making configurable
    Ok(max_length + 2)

}

fn write_entry(f: &mut std::fmt::Formatter, lengths: &Vec<usize>,
               col: usize, writable: &dyn Display, scratch: &mut Vec<u8>) -> std::fmt::Result {
    let mut write_width = item_width(scratch, writable);

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
        }).collect::<std::io::Result<Vec<usize>>>().map_err(|e| {
            std::fmt::Error::default()
        })?;

        let max_length = self.columns.iter().map(|c| c.len()).max().unwrap_or(0);
        let mut scratch = Vec::new();

        for (num, name) in self.column_names.iter().enumerate() {
            write_entry(f, &column_print_widths, num, name, &mut scratch);
        }

        writeln!(f)?;

        for i in 0..max_length {
            for (num, col) in self.columns.iter().enumerate() {
                let mut index = i;
                if i > col.len() {
                    index = col.len() - 1;
                }

                match col {
                    Column::Strings(s) => write_entry(f, &column_print_widths, num, &s[index], &mut scratch)?,
                    Column::Ints(i) => write_entry(f, &column_print_widths, num, &i[index], &mut scratch)?,
                    Column::Floats(floats) => write_entry(f, &column_print_widths, num, &floats[index], &mut scratch)?,
                    Column::Dates(d) => write_entry(f,  &column_print_widths, num, &NaiveDateTime::from_timestamp(d[index].clone(), 0).to_string(), &mut scratch)?,
                    Column::Booleans(b) => write_entry(f, &column_print_widths, num, &b[index], &mut scratch)?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}