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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let type_;
        match self.token_type {
            TokenType::Identifier => type_ = "identifier",
            TokenType::Keyword => type_ = "keyword",
            TokenType::Literal => type_ = "literal",
            TokenType::Operator => type_ = "operator",
            TokenType::Separator => type_ = "separator"
        };

        write!(f, "({}: {})", type_, self.text)
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
            (?P<operator>[-+/*]|or\s|and\s)
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

            if let Some(m) = cap.name("keyword") {
                v.push_back(Token::new(m.to_string(), TokenType::Keyword));
            } else if let Some(m) = cap.name("operator") {
                v.push_back(Token::new(m.to_string(), TokenType::Operator));
            } else if let Some(m) = cap.name("literal") {
                v.push_back(Token::new(m.to_string(), TokenType::Literal));
            } else if let Some(m) = cap.name("identifier") {
                v.push_back(Token::new(m.to_string(), TokenType::Identifier));
            } else if let Some(m) = cap.name("separator") {
                v.push_back(Token::new(m.to_string(), TokenType::Separator));
            }
        }

        Ok(v)
    }

}