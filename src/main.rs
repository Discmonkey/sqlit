mod schema;
mod column;
mod io;
mod ast;
mod table;
mod tokenizer;
mod result;
mod parser;
mod select;
mod scalar;
mod ops;
mod converters;
mod column_builder;

fn main() {
    let io = io::UserIO::new();
    let toke = tokenizer::Tokenizer::new();
    loop {
        io.greet();

        let line = io.read_line();

        if line.is_none() { break; }

        let l = line.unwrap();

        let tokens = toke.tokenize(l);

        match tokens {
            Err(e) => io.write_line(&e.to_string()),
            Ok(tokens) => {
                tokens.iter().for_each(|t| {
                    io.write(&format!("{} ", t));
                });
                println!();
            }
        }
    }

    print!("exiting...");
}
