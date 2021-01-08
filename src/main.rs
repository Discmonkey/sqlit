mod args;
mod ast;
mod build_column;
mod converters;
mod eval;
mod ops;
mod parser;
mod result;
mod table;
mod tokenizer;

use linefeed;
use std::io;
use crate::parser::rdp::RecursiveDescentParser;
use crate::result::SqlResult;

fn main() -> std::io::Result<()> {

    // reading command line args
    let args = args::get();

    // setting up io interface
    let mut io = linefeed::Interface::new("sqlit")?;
    io.set_prompt("sqlit> ");

    // creating our tokenizer
    let toke = tokenizer::Tokenizer::new();

    // loading tables
    let mut table_store = table::Store::from_paths(args.table_paths)?;

    // get ops
    let mut ops = ops::OpContext::new();

    // loop
    while let linefeed::ReadResult::Input(input) = io.read_line()? {
        if input.trim().len() == 0 {
            continue;
        } else {
            io.add_history(input.trim().to_string());
        }

        let mut tokens = toke.tokenize(input);
        let mut parser = RecursiveDescentParser::new(tokens);
        let parse_result = parser.parse();

        match parse_result {
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
