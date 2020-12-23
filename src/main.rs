mod schema;
mod column;
mod ast;
mod table;
mod tokenizer;
mod result;
mod select;
mod scalar;
mod ops;
mod converters;
mod build_column;
mod parser;

use linefeed;
use std::io;

fn main() -> std::io::Result<()> {

    let mut io = linefeed::Interface::new("sqlit")?;
    io.set_prompt("sqlit> ");
    io.set_completer(std::sync::Arc::new(linefeed::complete::PathCompleter));

    let toke = tokenizer::Tokenizer::new();
    let parser = parser::rdp::RecursiveDescentParser{};

    while let linefeed::ReadResult::Input(input) = io.read_line()? {
        if input.len() == 0 {
            continue
        } else {
            io.add_history(input.trim().to_string());
        }

        let tokens = toke.tokenize(input);

        match tokens {
            Err(e) => println!("{}", e),
            Ok(mut tokens) => {
                let parse_result = parser.parse(&mut tokens);

                match parse_result {
                    Err(e) => println!("{}", e),
                    Ok(parsed) => print!("{}", parsed)
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
