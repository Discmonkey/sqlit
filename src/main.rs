mod args;
mod build_column;
mod converters;
mod eval;
mod ops;
mod parser;
mod result;
mod table;
mod tokenizer;
mod linefeed_io;
mod ingest;

use linefeed;
use crate::parser::rdp::RecursiveDescentParser;
use std::sync::Arc;
use crate::linefeed_io::TableCompleter;
use crate::ingest::CsvFinder;

fn main() -> std::io::Result<()> {

    // reading command line args
    let args = args::get();

    // creating our tokenizer
    let toke = tokenizer::Tokenizer::new();

    // loading tables
    let mut table_store = table::Store::from_paths(args.table_paths, &args.separator)?;

    // setting up io interface
    let io = linefeed::Interface::new("sqlit")?;

    io.set_completer(
        Arc::new(
            TableCompleter::new(table_store.list().iter().map(|t| t.meta()).collect())));

    io.set_prompt("sqlit> ")?;

    // get ops
    let mut ops = ops::OpContext::new();

    // loop
    while let linefeed::ReadResult::Input(input) = io.read_line()? {
        if input.trim().len() == 0 {
            continue;
        } else {
            io.add_history(input.trim().to_string());
        }

        let tokens = toke.tokenize(input);
        let mut parser = RecursiveDescentParser::new(tokens);

        match parser.parse() {
            Err(e) => println!("{}", e),
            Ok(parsed) => {
                match eval::eval(parsed, &mut ops, &mut table_store) {
                    Err(e) => println!("{}", e),
                    Ok(evaluated) => println!("{}", evaluated)
                }
            }
        }
    }

    println!("exiting...");

    Ok(())
}


#[cfg(test)]
#[macro_use]
extern crate time_test;
