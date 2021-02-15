use sqlit;

// null_test
//
// first,second,third
// 0,"hello",null
// 0,"bye",true
// 1,"null",false
// null,"null",false

fn eval_query(query: &str, table: &str) -> sqlit::result::SqlResult<sqlit::table::Table> {
    let input = query.to_string();

    let store = sqlit::table::Store::from_paths(vec![table.to_string()],
                                                &(Box::new(sqlit::ingest::CsvFinder{}) as Box<dyn sqlit::ingest::SepFinder>), "null").map_err(|_| {
        sqlit::result::SqlError::new("could not read in table", sqlit::result::ErrorType::Runtime)
    })?;

    let mut ops = sqlit::ops::OpContext::new();

    let tokenizer = sqlit::tokenizer::Tokenizer::new();

    let tokens = tokenizer.tokenize(input);

    let parsed = sqlit::parser::rdp::RecursiveDescentParser::new(tokens).parse()?;

    sqlit::eval::eval(parsed, &mut ops, &store)
}

#[test]
fn test_column_is_hello() {
    let result = eval_query("select second = 'hello' from null_test where second = 'hello'", "tests/data/null_test.csv");

    match result {
        Err(_e) => assert!(false),
        Ok(t) => {

            assert!(t.len() > 0);

            let cols = t.into_columns();

            assert_eq!(cols.len(), 1);

            match cols[0].column.as_ref() {
                sqlit::table::Column::Booleans(b) => {
                    assert!(b[0].unwrap());
                },
                _ => assert!(false)
            }
        }
    }
}

#[test]
fn test_join() {
    let result = eval_query("select * from null_test a left join null_test b on a.first = b.first", "tests/data/null_test.csv");

    if !result.is_ok() {
        println!("{}", result.err().unwrap());
        assert!(false);

        return;
    }


    let t = result.unwrap();

    println!("{}", t);
    assert_eq!(t.len(), 6);

    assert_eq!(t.to_columns().len(), 6);
}

#[test]
fn test_string_comp() {
    let result = eval_query("select * from capitals_test where one = 'First Last'", "tests/data/capitals_test.csv");

    match result {
        Ok(r) => {
            assert_eq!(r.len(), 1);
        }
        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    };
}