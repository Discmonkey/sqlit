mod schema;
mod column;
mod io;

fn main() {
    let io = io::UserIO::new();

    loop {
        io.greet();
        let line = io.read_line();

        match line {
            Some(l) => io.write_line(&l),
            None => {
                break;
            }
        }
    }

    print!("exiting...");
}
