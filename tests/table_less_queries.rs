use sqlit;


fn eval_query(query: &str) -> sqlit::result::SqlResult<sqlit::table::Table> {
    let input = query.to_string();

    let store = sqlit::table::Store::new();
    let mut ops = sqlit::ops::OpContext::new();

    let tokenizer = sqlit::tokenizer::Tokenizer::new();

    let tokens = tokenizer.tokenize(input);

    let parsed = sqlit::parser::rdp::RecursiveDescentParser::new(tokens).parse()?;

    sqlit::eval::eval(parsed, &mut ops, &store)
}

#[test]
fn test_string_equality() {
    let result = eval_query("select 'hello' = 'hello'");

    match result {
        Err(_e) => assert!(false),
        Ok(t) => {
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
fn test_select_from_select() {
    let result = eval_query("select second, first from (select 1 as first, 2 as second) table");

    match result {
        Err(e) => {
            println!("{}", e);
            assert!(false)
        },
        Ok(t) => {
            let cols = t.into_columns();

            assert_eq!(cols.len(), 2);

            match cols[0].column.as_ref() {
                sqlit::table::Column::Ints(i) => {
                    assert_eq!(i[0].unwrap(), 2)
                },
                _ => assert!(false)
            }
        }
    }
}

#[test]
fn case_sensitive_string_comp() {
    let result = eval_query("select 1 from (select 'Hello' as hello) table where hello = 'Hello'");

    match result {
        Err(e) => {
            println!("{}", e);
            assert!(false)
        },
        Ok(t) => {
            let cols = t.into_columns();

            assert_eq!(cols.len(), 1);

            match cols[0].column.as_ref() {
                sqlit::table::Column::Ints(b) => {
                    assert_eq!(b[0].unwrap(), 1)
                },
                _ => assert!(false)
            }
        }
    }

}