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

            match &cols[0].column {
                sqlit::table::Column::Booleans(b) => {
                    assert!(b[0].unwrap());
                },
                _ => assert!(false)
            }
        }
    }
}