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

    pub fn get_type(&self) -> TokenType {
        self.token_type
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

pub type ErrorIndex = usize;


impl Tokenizer {

    pub fn new() -> Self {

        let re = Regex::new(r#"(?xi)
            [\s]* #skip white spaces
            (?P<keyword>SELECT|FROM|WHERE|GROUP\s+BY|LEFT\s+JOIN|INNER\s+JOIN|ORDER\s+BY|INTO|LIMIT)
            |
            (?P<operator>>=|<=|[-+/*><]|or\s|and\s)
            |
            (?P<literal>'.+'|[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?|[0-9]+|true|false|null)
            |
            (?P<identifier>[a-zA-z][_a-zA-Z1-9]*)
            |
            (?P<separator>[,()])"#);

        Self{re: re.unwrap()}
    }

    pub fn tokenize(&self, line:  String) -> Result<Tokens, &str> {

        let mut v = VecDeque::new();

        for cap in self.re.captures_iter(&line) {
            // need to fix this at some point...
            if cap.len() == 0 {
                continue;
            }

            for token_type in vec!(TokenType::Keyword, TokenType::Operator, TokenType::Literal, TokenType::Identifier, TokenType::Separator) {
                if let Some(m) = cap.name(token_type.to_str()) {
                    v.push_back(Token::new(m.to_string().to_lowercase(), token_type));
                }
            }
        }

        Ok(v)
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Tokenizer;

    #[test]
    fn tokenize_basic_select() {
        let t = Tokenizer::new();

        let tokens = t.tokenize("SELECT a, b, c FROM table".to_string());

        match tokens {
            Ok(mut t) => {
                let first = t.pop_front().unwrap();
                assert_eq!(first.text.as_str(), "SELECT");
            },
            Err(_e) => assert!(false)
        }


    }

}