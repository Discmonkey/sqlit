use crate::tokenizer::{Tokens, Token, TokenType};
use crate::result::{SqlError, SqlResult};
use crate::result::ErrorType::Syntax;
use crate::tokenizer::TokenType::{Identifier, Literal};
use crate::parser::rdp::ParserNodeType::{Where, GroupBy, OrderBy};
use crate::parser::{ParserNode, ParserNodeType};
use crate::parser::ParserNodeType::{Function, OrderByStatement};

pub struct RecursiveDescentParser {
    tokens: Tokens,
}

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

        let optional_clauses = vec!("from", "where", "group by", "order by", "into");
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
            self.tokens.pop_front();
            node.add_child(self.parse_expression()?);
        }

        Ok(node)
    }

    // expression is used for readability but does not actually produce a node, making the walk a bit easier
    fn parse_expression(&mut self) -> ParserResult {
        let mut tree = self.parse_equality()?;

        while tree.children.len() == 1 && tree.tokens.len() == 0 {
            tree = tree.children.pop().unwrap();
        }

        Ok(tree)
    }

    fn parse_equality(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Equality);

        node.add_child(self.parse_comparison()?);

        while let Some(t) = self.tokens.front() {
            if t.is("!=") || t.is("=") {
                node.add_token(self.tokens.pop_front().unwrap());
                node.add_child(self.parse_comparison()?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_comparison(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Comparison);

        node.add_child(self.parse_term()?);

        while let Some(t) = self.tokens.front() {
            if t.is(">") || t.is(">=") || t.is("<") || t.is("<=") {
                node.add_token(self.tokens.pop_front().unwrap());
                node.add_child(self.parse_term()?);
            } else {
                break;
            }
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

        while let Some(t) = self.tokens.front() {
            if t.is("!") || t.is("-") {
                node.add_token(self.tokens.pop_front().unwrap());
                node.add_child(self.parse_unary()?);
            } else {
                node.add_child(self.parse_primary()?);
                break;
            }
        }

        Ok(node)
    }

    fn parse_primary(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Primary);
        let mut found_identifier = false;

        if self.next_token_type_is(Literal) {
            node.add_token(self.tokens.pop_front().unwrap());

            return Ok(node);
        }

        if self.next_token_type_is(Identifier)  {
            node.add_token(self.tokens.pop_front().unwrap());
            found_identifier = true;
        }

        if self.next_token_is("(") {
            if found_identifier {
                node.set_type(Function);
            }

            self.tokens.pop_front();

            if self.next_token_is("select") {
                node.add_child(self.parse_query()?);
            } else {
                node.add_child(self.parse_expression()?);
            }

            self.get_required_token_by_value(")", "non-terminated paren")?;
        }

        Ok(node)
    }

    fn parse_from(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::From);
        self.get_required_token_by_value("from", "missing from statement")?;

        node.add_child(self.parse_table()?);

        Ok(node)
    }

    fn parse_table(&mut self) -> ParserResult {
        let mut node = ParserNode::new(ParserNodeType::Table);

        loop {
            let next = self.tokens.front().unwrap();

            if next.is_type(Identifier) {
                node.add_token(self.tokens.pop_front().unwrap())
            } else if next.is("(") {
                self.tokens.pop_front();
                node.add_child(self.parse_query()?);

                let closing = self.tokens.pop_front().unwrap();

                if !closing.is(")") {
                    return Err(SqlError::new("missing closing paren", Syntax));
                }

                let identifier_required = self.tokens.pop_front().unwrap();

                if !identifier_required.is_type(Identifier) {
                    return Err(SqlError::new("expected identifier after query", Syntax));
                }

                node.add_token(identifier_required);
            }

            let maybe_join = self.tokens.front();

            if let Some(t) = maybe_join {
                if !t.is("left join") && !t.is("inner join") {
                    break;
                }

                node.add_token(self.tokens.pop_front().unwrap());
            } else {
                break;
            }
        }

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
                                         "group by keyword required");

        node.add_token(self.get_required_token_by_type(Identifier,
                                        "group by requires at least one column name")?);

        while self.next_token_is(",") {
            self.tokens.pop_front();

            node.add_token(self.get_required_token_by_type(Identifier,
                "malformed group by clause, can only only group on columns")?
            )
        }

        Ok(node)
    }

    fn parse_order_by(&mut self) -> ParserResult {
        let mut node = ParserNode::new(OrderBy);

        self.get_required_token_by_value("order by",
                                         "order by keyword required");

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

        if self.next_token_is("ASC") || self.next_token_is("DESC") {
            node.add_token(self.tokens.pop_front().unwrap())
        }

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

    #[test]
    fn tokenize_basic_select() {
        let t = Tokenizer::new();
        let mut tokens = t.tokenize("SELECT a, b, c FROM table".to_string()).unwrap();
        let maybe_tree = RecursiveDescentParser::new(tokens).parse();

        match maybe_tree {
            Err(_e) => assert!(false),
            Ok(tree) => assert!(tree.children.len() > 0)
        }


    }
}





