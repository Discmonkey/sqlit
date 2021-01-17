use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub table_paths: Vec<String>,
    pub parse_columns: bool,
    pub separator: String,
}

pub fn get() -> Config {
    let matches = App::new("SQL-it")
        .version("0.1")
        .author("max grinchenko <frismo98@gmail.com>")
        .about("sql-ize your csvs from the command line")
        .arg(Arg::new("tables")
            .required(true)
            .about("tables that will be parsed at startup")
            .index(1)
            .multiple(true))
        .arg(Arg::new("column_help")
            .about("enter column names manually at startup")
            .short('c')
            .long("columns"))
        .arg(Arg::new("tsv")
            .about("separator file uses between columns")
            .short('t')
            .long("tsv")).get_matches();

    let table_paths: Vec<_> = matches.values_of("tables").unwrap().map(|s| s.to_string()).collect();
    let parse_columns = !(matches.occurrences_of("column_help") > 0);
    let separator = if matches.occurrences_of("tsv") > 0 { "\t" } else {","}.to_string();

    Config {
        table_paths,
        parse_columns,
        separator
    }

}


