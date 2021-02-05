use linefeed;
use std::sync::Arc;

use sqlit::linefeed_io::TableCompleter;
use sqlit::parser::rdp::RecursiveDescentParser;
use sqlit::tokenizer;
use sqlit::table;
use sqlit::ops;
use sqlit::eval;

mod args;

fn main() -> std::io::Result<()> {

    // reading command line args
    let args = args::get();

    // creating our tokenizer
    let toke = tokenizer::Tokenizer::new();

    // loading tables
    let mut table_store = table::Store::from_paths(args.table_paths, &args.separator, &args.null_representation)?;

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

