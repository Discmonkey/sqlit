use std::str::Chars;
use std::collections::VecDeque;
use regex::Regex;
use std::fmt::Write;

mod common;

type Index = usize;

pub trait SepFinder {

    /// consumes characters that are related to a separator, returns the new Index.
    fn consume_sep(&self, chars: &Vec<char>, index: Index, length: Index) -> Index;

    /// at sep return true if the front of chars points to a separator
    fn at_sep(&self, chars: &Vec<char>, index: Index, length: Index) -> bool;
}


macro_rules! sep_finder_implementation {
    ($target: ident, $char: expr) => {
        impl SepFinder for $target {
            fn consume_sep(&self, chars: &Vec<char>, mut index: Index, length: Index) -> Index {
                let mut found_sep = false;

                while index < length {
                    if chars[index] == ' ' {
                        index += 1;
                    } else if chars[index] == $char && !found_sep {
                        index += 1;
                    } else {
                        break;
                    }
                }

                index
            }

            fn at_sep(&self, chars: &Vec<char>, mut index: Index, length: Index) -> bool {
                let mut found_sep = false;

                while index < length {
                    if chars[index] == ' ' {
                        index += 1;
                    } else if chars[index] == $char && !found_sep {
                        found_sep = true;
                    } else {
                        break;
                    }
                }

                return found_sep;
            }
        }
    }
}

/// reads csv's.
pub struct CsvFinder {
}


sep_finder_implementation!(CsvFinder, ',');

/// reads tsv's.
pub struct TsvFinder {}

sep_finder_implementation!(TsvFinder, '\t');


/// reads files with separations of more than one space.
pub struct SpacesFinder {
}

impl SepFinder for SpacesFinder {
    fn consume_sep(&self, chars: &Vec<char>, mut index: Index, length: Index) -> Index {
        while index < length && chars[index] == ' ' {
            index += 1;
        }

        index
    }

    fn at_sep(&self, chars: &Vec<char>, mut index: Index, length: Index) -> bool {
        let mut count = 0;
        while index < length && chars[index] == ' ' {
            index += 1;
            count += 1;
        }

        return count > 2;
    }
}

macro_rules! not {
    ($expr: expr) => {
        !$expr
    }
}

pub fn read_line(line: String, separator_reader: &Box<dyn SepFinder>) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut fields = Vec::new();
    let mut index = 0;
    let length = chars.len();

    while index < length {
        if separator_reader.at_sep(&chars, index, length) {
            index = separator_reader.consume_sep(&chars, index, length);
        } else {
            let (field, new_index) = read_field(&chars, index, length, separator_reader);
            index = new_index;

            fields.push(field);
        }
    };

    fields
}

pub fn read_field(chars: &Vec<char>, mut index: Index, length: Index, separator_reader: &Box<dyn SepFinder>) -> (String, Index) {
    let mut s = String::new();

    while index < length && !separator_reader.at_sep(chars, index, length) {
        match chars[index] {
            // on opening quotes we don't need to check the separator until the next opening/closing character
            '"' | '\'' => {
                let c = chars[index];
                index += 1;
                // read until we find the next instance of this character
                index = read_until(c, chars, index, length, &mut s);
            },

            _ => {
                s.push(chars[index]);
                index += 1;
            }
        }
    }

    (s, index)
}

fn read_until(character: char,
              characters: &Vec<char>, mut index: usize, length: usize,
              target: &mut String) -> Index {

    while index < length {
        if characters[index] == character {
            index += 1;
            break;
        } else {
            target.write_char(characters[index]);
            index += 1;
        }
    }

    index
}


#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use crate::ingest::{CsvFinder, read_line, SepFinder};

    #[test]
    fn test_line_read() {
        time_test!();

        let line = "this,is,a,4,'csv'";

        let sep = CsvFinder{};

        let parts = read_line(line.to_string(), &(Box::new(CsvFinder{}) as Box<dyn SepFinder>));

        parts.into_iter().zip(vec!["this", "is", "a", "4", "csv"].into_iter()).for_each(|(l, r)| {
            assert_eq!(l, r);
        });
    }

    #[test]
    fn test_dynamic_pass() {
        let line = "this,is,a,4,'csv'";

        let parts = read_line(line.to_string(), &(Box::new(CsvFinder{}) as Box<dyn SepFinder>));

        parts.into_iter().zip(vec!["this", "is", "a", "4", "'csv'"].into_iter()).for_each(|(l, r)| {
            assert_eq!(l, r);
        });
    }

    #[test]
    fn tough_csv() {
        let header = "valence,year,acousticness,artists,danceability,duration_ms,energy,explicit,id,instrumentalness,key,liveness,loudness,mode,name,popularity,release_date,speechiness,tempo";
        let l1 = "0.0594,1921,0.982,\"['Sergei Rachmaninoff', 'James Levine', 'Berliner Philharmoniker']\",0.279,831667,0.211,0,4BJqT0PrAfrxzMOxytFOIz,0.878,10,0.665,-20.096,1,\"Piano Concerto No. 3 in D Minor, Op. 30: III. Finale. Alla breve\",4,1921,0.0366,80.954";
        let l2 = "0.963,1921,0.732,['Dennis Day'],0.8190000000000001,180533,0.341,0,7xPhfUan2yNtyFG0cUWkt8,0.0,7,0.16,-12.441,1,Clancy Lowered the Boom,5,1921,0.415,60.93600000000001";

        let sep = Box::new(CsvFinder{}) as Box<dyn SepFinder>;

        let length_header = read_line(header.to_string(), &sep).len();

        let l1_header = read_line(l1.to_string(), &sep);

        let l2_header = read_line(l2.to_string(), &sep);

        assert_eq!(length_header, l1_header.len());
        assert_eq!(length_header, l2_header.len())

    }

}