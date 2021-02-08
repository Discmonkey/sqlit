use crate::table::TableMeta;
use linefeed::{Completer, Prompter, Completion, Terminal};

pub struct TableCompleter {
    tables: Vec<TableMeta>
}

impl<Term: Terminal> Completer<Term> for TableCompleter {
    fn complete(&self, _word: &str, _reader: &Prompter<Term>,
                _start: usize, _end: usize) -> Option<Vec<Completion>> {
        let mut completions = Vec::new();

        self.tables.iter().for_each(|metadata| {
            if metadata.alias.starts_with(_word) {
                completions.push(Completion::simple(metadata.alias.clone()))
            }

            metadata.columns.iter().for_each(|(_, c, _)| {
                if c.starts_with(_word) {
                    completions.push(Completion::simple(c.clone()))
                }
            })
        });

        if completions.len() > 0 {
            Some(completions)
        } else {
            None
        }
    }
}

impl TableCompleter {
    pub fn new(tables: Vec<TableMeta>) -> Self {
        Self {
            tables
        }
    }
}