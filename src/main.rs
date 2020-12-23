mod schema;
mod column;
mod io;
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



fn main() {
    let io = io::UserIO::new();
    let toke = tokenizer::Tokenizer::new();
    let parser = parser::rdp::RecursiveDescentParser{};

    loop {
        io.greet();

        let line = io.read_line();

        if line.is_none() { break; }

        let l = line.unwrap();

        let tokens = toke.tokenize(l);

        match tokens {
            Err(e) => io.write_line(&e.to_string()),
            Ok(mut tokens) => {
                let parse_result = parser.parse(&mut tokens);

                match parse_result {
                    Err(e) => println!("{}", e),
                    Ok(parsed) => print!("{}", parsed)
                }
            }
        }
    }

    print!("exiting...");
}

#[cfg(test)]
#[macro_use]
extern crate time_test;
