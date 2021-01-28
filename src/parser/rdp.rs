use crate::tokenizer::{Tokens, Token, TokenType};
use crate::result::{SqlError, SqlResult};
use crate::result::ErrorType::Syntax;
use crate::tokenizer::TokenType::{Identifier, Literal};
use crate::parser::rdp::ParserNodeType::{Where, GroupBy, OrderBy};
use crate::parser::{ParserNode, ParserNodeType};
use crate::parser::ParserNodeType::{OrderByStatement};

pub struct RecursiveDescentParser {
    tokens: Tokens,
}

const PAREN_ERROR: &str = "un-terminated paren";

type ParserResult = SqlResult<ParserNode>;

impl RecursiveDescentParser {

    pub fn new(tokens: Tokens) -> Self {
        RecursiveDescentParser {
            tokens
        }
    }

    pub fn parse(&mut self) -> ParserResult {
        self.parse_query()
    }

    fn next_token_is(&self, value: &str) -> bool {
        match self.tokens.front() {
            Some(t) => t.is(value),
            None => false
        }
    }

    fn next_token_type_is(&self, type_: TokenType) -> bool {
        match self.tokens.front() {
            Some(t) => t.is_type(type_),
            None => false
        }
    }

    /// next_next_token_is is needed to distinguish between functions and regular identifiers, ie
    /// test(...args) versus test * test
    /// an ll(1) parser fails on such an example, so, alas, we need this method
    fn next_next_token_is(&self, value: &str) -> bool {
        match self.tokens.get(1) {
            Some(t) => t.is(value),
            None => false,
        }
    }

    fn get_required_token_by_type(&mut self,
                                  token_type: TokenType,
                                  err_message: &str) -> Result<Token, SqlError> {

        match self.tokens.pop_front() {
            None => Err(SqlError::new(err_message, Syntax)),
            Some(t) => {
                if t.is_type(token_type) {
                    Ok(t)
                } else {
                    Err(SqlError::new(err_message, Syntax))
                }
            }
        }
    }

    /// get the next token unsafely
    fn next(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    fn get_required_token_by_value(&mut self,
                                   token_value: &str,
                                   err_message: &str) -> Result<Token, SqlError> {
        match self.tokens.pop_front() {
            None => Err(SqlError::new(err_message, Syntax)),
            Some(t) => {
                if t.is(token_value) {
                    Ok(t)
                } else {
                    Err(SqlError::new(err_message, Syntax))
                }
            }
        }
    }

    fn parse_query(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Query);
        // always consume the next token
        self.get_required_token_by_value("select", "query must begin with select")?;

        // columns are required
        node.add_child(self.parse_columns()?);

        let optional_clauses = vec!("from", "where", "group by", "order by", "limit", "into");
        let mut current_index = 0;

        while let Some(token) = self.tokens.front() {
            let index = optional_clauses.iter().position(|s| { token.is(s) });

            match index {
                Some(i) => {
                    if i < current_index {
                        return Err(SqlError::new("select clauses out of order", Syntax));
                    }

                    current_index = i;

                    match optional_clauses[current_index] {
                        "from" => node.add_child(self.parse_from()?),
                        "where" => node.add_child(self.parse_where()?),
                        "group by" => node.add_child(self.parse_group_by()?),
                        "order by" => node.add_child(self.parse_order_by()?),
                        "into" => node.add_child(self.parse_into()?),
                        "limit" => node.add_child(self.parse_limit()?),
                        _ => ()
                    }
                }
                None => break
            }
        }

        Ok(node)
    }

    fn parse_columns(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Columns);

        node.add_child(self.parse_expression()?);

        while self.next_token_is(",") {
            self.next();

            if self.next_token_is("*") {
                node.add_child(self.parse_star()?);
            } else {
                node.add_child(self.parse_expression()?);
            }
        }

        Ok(node)
    }

    fn parse_star(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::StarOperator);

        self.get_required_token_by_value("*")?;

        Ok(node)
    }

    // expression is used for readability but does not actually produce a node, making the walk a bit easier
    fn parse_expression(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Expression);

        node.add_child(self.parse_equality()?);

        if self.next_token_is("as") {
            self.next();

            node.add_child(self.parse_identifier()?);
        }

        Ok(node)
    }

    fn parse_equality(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Equality);

        node.add_child(self.parse_comparison()?);

        while self.next_token_is("!=") || self.next_token_is("=") {
            node.add_token(self.tokens.pop_front().unwrap());
            node.add_child(self.parse_comparison()?);
        }

        Ok(node)
    }

    fn parse_comparison(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Comparison);

        node.add_child(self.parse_term()?);

        while [">", ">=", "<", "<=", "and", "or", "xor"].iter().any(|val| self.next_token_is(val)) {
            node.add_token(self.tokens.pop_front().unwrap());
            node.add_child(self.parse_term()?);
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Term);

        node.add_child(self.parse_factor()?);

        while let Some(t) = self.tokens.front() {
            if t.is("-") || t.is("+") {
                node.add_token(self.tokens.pop_front().unwrap());
                node.add_child(self.parse_factor()?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Factor);

        node.add_child(self.parse_unary()?);

        while let Some(t) = self.tokens.front() {
            if t.is("/") || t.is("*") {
                node.add_token(self.tokens.pop_front().unwrap());
                node.add_child(self.parse_unary()?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Unary);

        if self.next_token_is("-") || self.next_token_is("!") {
            node.add_token(self.next());
            node.add_child(self.parse_unary()?);
        } else {
            node.add_child(self.parse_primary()?);
        }

        Ok(node)
    }

    fn parse_primary(&mut self) -> ParserResult {

        let mut node = ParserNode::new(ParserNodeType::Primary);

        if self.next_token_type_is(Literal) {
            node.add_child(self.parse_literal()?);
        } else if self.next_token_type_is(Identifier)  {
            if self.next_next_token_is("(") {
                node.add_child(self.parse_function()?);
            } else {
                node.add_child(self.parse_identifier()?)
            }
        } else if self.next_token_is("(") {
            self.tokens.pop_front();

            if self.next_token_is("select") {
                node.add_child(self.parse_query()?);
            } else {
                node.add_child(self.parse_expression()?);
            }

            self.get_required_token_by_value(")", PAREN_ERROR)?;
        } else {
            return Err(SqlError::new("missing expression", Syntax));
        }

        Ok(node)
    }

    fn parse_literal(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Literal);

        node.add_token(self.get_required_token_by_type(Literal, "literal required")?);

        Ok(node)
    }

    fn parse_identifier(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Identifier);

        node.add_token(
            self.get_required_token_by_type(
                Identifier, "missing column identifier")?);

        if self.next_token_is(".") {
            self.next();

            node.add_token(
                self.get_required_token_by_type(
                    Identifier, "missing column identifier")?);
        }

        Ok(node)
    }

    fn parse_function(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Function);

        node.add_token(self.get_required_token_by_type(Identifier, "identifier required for function")?);

        self.get_required_token_by_value("(", "missing opening paren")?;

        node.add_child(self.parse_columns()?);

        self.get_required_token_by_value(")", PAREN_ERROR)?;

        Ok(node)
    }

    fn parse_from(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::From);
        self.get_required_token_by_value("from", "malformed from clause")?;

        node.add_child(self.parse_from_statement()?);

        while ["left join", "right join"].iter().any(|s| self.next_token_is(s)) {

            // need the token due to different rules for joins
            node.add_token(self.next());

            // the table / query we are joining
            node.add_child(self.parse_from_statement()?);

            self.get_required_token_by_value("on",
                                             "join condition starting with on is required")?;

            // the condition we are joining it on
            node.add_child(self.parse_expression()?);
        }

        Ok(node)
    }

    fn parse_from_statement(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::FromStatement);

        if self.next_token_is("(") {
            self.next();

            node.add_child(self.parse_query()?);

            self.get_required_token_by_value(")",
                                             "non-terminated paren in from statement")?;
        }

        node.add_token(self.get_required_token_by_type(Identifier,
                                                       "name required for join table")?);

        Ok(node)
    }

    fn parse_where(&mut self) -> ParserResult {
        let mut node = ParserNode::new(Where);

        self.get_required_token_by_value("where", "invalid where clause")?;

        node.add_child(self.parse_expression()?);

        Ok(node)
    }

    fn parse_group_by(&mut self) -> ParserResult {
        let mut node = ParserNode::new(GroupBy);

        self.get_required_token_by_value("group by",
                                         "group by keyword required")?;

        node.add_child(self.parse_columns()?);

        Ok(node)
    }

    fn parse_order_by(&mut self) -> ParserResult {
        let mut node = ParserNode::new(OrderBy);

        self.get_required_token_by_value("order by",
                                         "order by keyword required")?;

        node.add_child(self.parse_order_by_statement()?);

        while self.next_token_is(",") {
            self.tokens.pop_front();
            node.add_child(self.parse_order_by_statement()?);
        }

        Ok(node)
    }

    fn parse_order_by_statement(&mut self) -> ParserResult {
        let mut node = ParserNode::new(OrderByStatement);

        node.add_token(self.get_required_token_by_type(Identifier, "can only order on columns")?);

        if self.next_token_is("asc") || self.next_token_is("desc") {
            node.add_token(self.tokens.pop_front().unwrap())
        }

        Ok(node)
    }

    fn parse_limit(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Limit);

        self.get_required_token_by_value("limit", "mis-configured limit clause")?;

        node.add_child(self.parse_literal()?);

        Ok(node)
    }

    fn parse_into(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Into);

        if let Some(t) = self.tokens.front() {
            if !t.is("into") {
                return Err(SqlError::new("expected order by clause", Syntax));
            }
        } else {
            return Err(SqlError::new("empty token stream passed to where parser", Syntax));
        }

        // skip into
        self.tokens.pop_front().unwrap();

        // go ahead and add the identifier
        node.add_token(self.tokens.pop_front().unwrap());

        Ok(node)
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Tokenizer;
    use crate::parser::rdp::RecursiveDescentParser;
    use crate::parser::ParserNodeType;

    #[test]
    fn parse_basic_select() {
        let t = Tokenizer::new();
        let mut tokens = t.tokenize("SELECT a, b, c FROM table".to_string());
        let maybe_tree = RecursiveDescentParser::new(tokens).parse();

        match maybe_tree {
            Err(_e) => assert!(false),
            Ok(tree) => assert!(tree.children.len() > 0)
        }
    }

    #[test]
    fn parse_two_function_select() {
        let t = Tokenizer::new();
        let tokens = t.tokenize("select mean(teampoints), mean(assists) from nba_games_stats".to_string());
        let maybe_tree = RecursiveDescentParser::new(tokens).parse();

        match maybe_tree {
            Err(_e) => assert!(false),
            Ok(tree) => {
                let (mut type_, mut tokens, mut nodes) = tree.release();

                assert_eq!(type_, ParserNodeType::Query);

                let columns = nodes.pop_front().unwrap();
                let from = nodes.pop_front().unwrap();

                let (type_, tokens, nodes) = columns.release();

                assert_eq!(type_, ParserNodeType::Columns);


            }
        }
    }

    #[test]
    fn parse_as_clause() {
        let t = Tokenizer::new();
        let tokens = t.tokenize("SELECT mean(assists) as time FROM nba_games_stats".to_string());
        let maybe_tree = RecursiveDescentParser::new(tokens).parse();

        match maybe_tree {
            Err(_e) => assert!(false),
            Ok(tree) => {
                let (mut type_, mut tokens, mut nodes) = tree.release();

                assert_eq!(type_, ParserNodeType::Query);

                let columns = nodes.pop_front().unwrap();
                let from = nodes.pop_front().unwrap();

                let (type_, tokens, nodes) = columns.release();

                assert_eq!(type_, ParserNodeType::Columns);


            }
        }

    }
}





