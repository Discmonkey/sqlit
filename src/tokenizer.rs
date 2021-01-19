use regex::Regex;
use std::fmt;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenType {
    // x, y, TableName, column name
    Identifier,

    // SELECT, FROM, WHERE, GROUP BY, LEFT JOIN, INNER JOIN, ORDER BY
    Keyword,

    // ( , )
    Separator,

    // + - = != is
    Operator,

    // 234.344 true 1 false NULL 'hello'
    Literal,
}

impl TokenType {
    pub fn to_str(&self) -> &str {
        match self {
            TokenType::Identifier => "identifier",
            TokenType::Keyword => "keyword",
            TokenType::Literal => "literal",
            TokenType::Operator => "operator",
            TokenType::Separator => "separator"
        }
    }


}

#[derive(Debug, Clone)]
pub struct Token {
    text: String,
    token_type: TokenType
}

impl Token {
    pub fn new(text: String, token_type: TokenType) -> Self {
        Self {
            text, token_type
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn is(&self, value: &str) -> bool {
        self.text.as_str() == value
    }

    pub fn is_type(&self, value: TokenType) -> bool {
        value == self.token_type
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "({}: {})", self.token_type.to_str(), self.text)
    }
}

pub type Tokens = VecDeque<Token>;

pub struct Tokenizer {
    re: Regex,
}

impl Tokenizer {

    pub fn new() -> Self {

        let re = Regex::new(r#"(?xi)
            [\s]* #skip white spaces
            (?P<keyword>SELECT\s|FROM\s|WHERE\s|GROUP\s+BY|LEFT\s+JOIN|INNER\s+JOIN|ORDER\s+BY|INTO\s|LIMIT\s|ASC\s|DESC\s|AS\s)
            |
            (?P<operator>>=|<=|[-+/*><=]|or\s|and\s|!=)
            |
            (?P<literal>'.+'|[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?|[0-9]+|true|false|null)
            |
            (?P<identifier>[a-zA-z][_a-zA-Z1-9]*)
            |
            (?P<separator>[,().])"#);

        Self{re: re.unwrap()}
    }

    pub fn tokenize(&self, line:  String) -> Tokens {

        let mut v = VecDeque::new();

        for cap in self.re.captures_iter(&line) {
            // need to fix this at some point...
            if cap.len() == 0 {
                continue;
            }

            for token_type in vec!(TokenType::Keyword, TokenType::Operator, TokenType::Literal, TokenType::Identifier, TokenType::Separator) {
                if let Some(m) = cap.name(token_type.to_str()) {
                    v.push_back(Token::new(m.to_string().trim().to_lowercase(), token_type));
                }
            }
        }

        v
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Tokenizer;
    use crate::tokenizer::TokenType::{Identifier, Separator, Keyword, Literal};

    #[test]
    fn tokenize_basic_select() {
        let t = Tokenizer::new();

        let mut tokens = t.tokenize("SELECT a.c, b, c FROM table".to_string());


        let first = tokens.pop_front().unwrap();
        assert!(first.is("select"));
        assert!(first.is_type(Keyword));

        let second = tokens.pop_front().unwrap();
        assert!(second.is("a"));
        assert!(second.is_type(Identifier));

        let third = tokens.pop_front().unwrap();
        assert!(third.is("."));
        assert!(third.is_type(Separator));

        let fourth = tokens.pop_front().unwrap();
        assert!(fourth.is("c"));
        assert!(fourth.is_type(Identifier));

    }

    #[test]
    fn distinguish() {
        let t = Tokenizer::new();

        let mut tokens = t.tokenize("12.34, a.b, 3.54".to_string());


        [Literal, Separator, Identifier, Separator, Identifier, Separator, Literal]
            .into_iter().for_each(|type_| {
            assert!(tokens.pop_front().unwrap().is_type(*type_));
        });
    }

    #[test]
    fn as_versus_assists() {
        let t = Tokenizer::new();

        let mut tokens = t.tokenize("SELECT year(date) as    y FROM nba_games_stats".to_string());

        [Keyword, Identifier, Separator, Identifier, Separator, Keyword, Identifier, Keyword, Identifier]
            .into_iter().for_each(|type_| {
            assert!(tokens.pop_front().unwrap().is_type(*type_));
        });
    }

}