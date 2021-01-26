use clap::{App, Arg};
use crate::ingest::{SepFinder, TsvFinder, SpacesFinder, CsvFinder};

pub struct Config {
    pub table_paths: Vec<String>,
    pub parse_columns: bool,
    pub separator: Box<dyn SepFinder>,
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
            .about("looks for a tab as the delimiter between columns")
            .short('t')
            .long("tsv"))
        .arg(Arg::new("spaces")
            .about("look for two + spaces as the delimiter between columns")
            .short('s')
            .long("spaces")
        .long("tsv")).get_matches();

    let table_paths: Vec<_> = matches.values_of("tables").unwrap().map(|s| s.to_string()).collect();
    let parse_columns = !(matches.occurrences_of("column_help") > 0);
    let separator = if matches.occurrences_of("tsv") > 0 {
            Box::new(TsvFinder{}) as Box<dyn SepFinder>
        } else if matches.occurrences_of("spaces") > 0 {
            Box::new(SpacesFinder{}) as Box<dyn SepFinder>
        } else {
            Box::new(CsvFinder{}) as Box<dyn SepFinder>
        };


    Config {
        table_paths,
        parse_columns,
        separator,
    }

}


