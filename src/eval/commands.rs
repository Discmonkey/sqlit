use std::collections::HashMap;
use crate::table::{Table, Store};
use std::fmt::Formatter;

pub (super) struct PrintableColumns {
    names: Vec<String>
}

pub (super) struct PrintableTables {
    tables: HashMap<String, PrintableColumns>
}

pub (super) struct PrintableTableNames {
    tables: Vec<String>
}

impl PrintableColumns {
    fn from(table: &Table) -> Self {
        Self {
            names: table.column_names()
        }
    }
}

impl PrintableTables {
    fn from(store: &Store) -> Self {
        Self {
            tables: store.tables().into_iter().map(|table_name| {
                (table_name, PrintableColumns::from(store.get(table_name.as_str()).unwrap()))
            }).collect()
        }
    }
}

impl PrintableTableNames {
    fn from(store: &Store) -> Self {
        Self {
            tables: store.tables()
        }
    }
}

impl std::fmt::Display for PrintableColumns {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for (num, col) in self.names.iter().enumerate() {
            if num > 0 {
                write!(f, " ")?;
            }

            write!(f, "{}", col)
        }

        Ok(())
    }
}

impl std::fmt::Display for PrintableTables {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for (num, (table_name, columns)) in self.tables.iter().enumerate() {
            if num > 0 {
                writeln!(f)?;
            }

            write!(f, "{}: {}", table_name, columns);
        }

        Ok(())
    }
}

impl std::fmt::Display for PrintableTableNames {

    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for (num, col) in self.tables.iter().enumerate() {
            if num > 0 {
                write!(f, " ")?;
            }

            write!(f, "{}", col)
        }

        Ok(())
    }
}