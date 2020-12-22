use crate::tokenizer::{Token, Tokens};
use crate::result::{SqlError, SqlResult};
use crate::result::ErrorType::Syntax;
use crate::tokenizer::TokenType::{Identifier, Literal};
use crate::parser::rdp::ParserNodeType::{Where, GroupBy, OrderBy};
use crate::parser::{ParserNode, ParserNodeType};

pub struct RecursiveDescentParser {}

type ParserResult = SqlResult<Box<ParserNode>>;

impl RecursiveDescentParser {

    pub fn parse(&self, tokens: &mut Tokens) -> ParserResult {
        self.parse_query(tokens)
    }

    fn parse_query(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Query));
        // always consume the next token
        let token = tokens.pop_front().unwrap();

        if !token.is("select") {
            return Err(SqlError::new("invalid select statement", Syntax))
        }

        // columns are required
        node.add_child(self.parse_columns(tokens)?);

        let optional_clauses = vec!("from", "where", "group by", "order by", "into");
        let mut current_index = 0;

        while let Some(token) = tokens.front() {
            let index = optional_clauses.iter().position(|s| { token.is(s) });

            match index {
                Some(i) => {
                    if i < current_index {
                        return Err(SqlError::new("select clauses out of order", Syntax));
                    }

                    current_index = i;

                    match optional_clauses[current_index] {
                        "from" => node.add_child(self.parse_from(tokens)?),
                        "where" => node.add_child(self.parse_where(tokens)?),
                        "group by" => node.add_child(self.parse_group_by(tokens)?),
                        "order by" => node.add_child(self.parse_order_by(tokens)?),
                        "into" => node.add_child(self.parse_into(tokens)?),
                        _ => ()
                    }
                }
                None => break
            }
        }

        Ok(node)
    }

    fn parse_columns(&self, tokens: &mut Tokens) -> ParserResult {

        let mut node = Box::new(ParserNode::new(ParserNodeType::Columns));

        node.add_child(self.parse_expression(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is(",") {
                tokens.pop_front();
                node.add_child(self.parse_expression(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    // expression is used for readability but does not actually produce a node, making the walk a bit easier
    fn parse_expression(&self, tokens: &mut Tokens) -> ParserResult {
        let mut tree = self.parse_equality(tokens)?;

        while tree.children.len() == 1 && tree.tokens.len() == 0 {
            tree = tree.children.pop().unwrap();
        }

        Ok(tree)
    }

    fn parse_equality(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Equality));

        node.add_child(self.parse_comparison(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is("!=") || t.is("=") {
                node.add_token(tokens.pop_front().unwrap());
                node.add_child(self.parse_comparison(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_comparison(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Comparison));

        node.add_child(self.parse_term(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is(">") || t.is(">=") || t.is("<") || t.is("<=") {
                node.add_token(tokens.pop_front().unwrap());
                node.add_child(self.parse_term(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_term(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Term));

        node.add_child(self.parse_factor(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is("-") || t.is("+") {
                node.add_token(tokens.pop_front().unwrap());
                node.add_child(self.parse_factor(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_factor(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Factor));

        node.add_child(self.parse_unary(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is("/") || t.is("*") {
                node.add_token(tokens.pop_front().unwrap());
                node.add_child(self.parse_unary(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_unary(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Unary));

        while let Some(t) = tokens.front() {
            if t.is("!") || t.is("-") {
                node.add_token(tokens.pop_front().unwrap());
                node.add_child(self.parse_unary(tokens)?);
            } else {
                node.add_child(self.parse_primary(tokens)?);
                break;
            }
        }

        Ok(node)
    }

    fn parse_primary(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Primary));

        if let Some(t) = tokens.front() {
            if t.get_type() == Literal {
                node.add_token(tokens.pop_front().unwrap());

                return Ok(node);
            }

            if t.get_type() == Identifier {
                node.add_token(tokens.pop_front().unwrap());
            }
        }

        if tokens.len() > 0 && tokens.front().unwrap().is("(") {
            node.add_token(tokens.pop_front().unwrap());

            let next = tokens.front().unwrap();

            if next.is("select") {
                node.add_child(self.parse_query(tokens)?);
            } else if next.is(")") {
                tokens.pop_front();
            } else {
                node.add_child(self.parse_expression(tokens)?);
            }
        }

        Ok(node)
    }

    fn parse_from(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::From));
        let next = tokens.pop_front().unwrap();

        if !next.is("from") {
            return Err(SqlError::new("mis-configured from statement", Syntax));
        }

        node.add_child(self.parse_table(tokens)?);

        Ok(node)
    }

    fn parse_table(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Table));

        loop {
            let next = tokens.front().unwrap();

            if next.is_type(Identifier) {
                node.add_token(tokens.pop_front().unwrap())
            } else if next.is("(") {
                tokens.pop_front();
                node.add_child(self.parse_query(tokens)?);

                let closing = tokens.pop_front().unwrap();

                if !closing.is(")") {
                    return Err(SqlError::new("missing closing paren", Syntax));
                }

                let identifier_required = tokens.pop_front().unwrap();

                if !identifier_required.is_type(Identifier) {
                    return Err(SqlError::new("expected identifier after query", Syntax));
                }

                node.add_token(identifier_required);
            }

            let maybe_join = tokens.front();

            if let Some(t) = maybe_join {
                if !t.is("LEFT JOIN") && !t.is("INNER JOIN") {
                    break;
                }

                node.add_token(tokens.pop_front().unwrap());
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_where(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(Where));

        if let Some(t) = tokens.front() {
            if !t.is("where") {
                return Err(SqlError::new("expected where clause", Syntax));
            }
        } else {
            return Err(SqlError::new("empty token stream passed to where parser", Syntax));
        }

        tokens.pop_front().unwrap();

        node.add_child(self.parse_expression(tokens)?);

        Ok(node)
    }

    fn parse_group_by(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(GroupBy));

        if let Some(t) = tokens.front() {
            if !t.is("group by") {
                return Err(SqlError::new("expected group by clause", Syntax));
            }
        } else {
            return Err(SqlError::new("empty token stream passed to where parser", Syntax));
        }

        tokens.pop_front().unwrap();

        node.add_child(self.parse_columns(tokens)?);

        Ok(node)
    }

    fn parse_order_by(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(OrderBy));

        if let Some(t) = tokens.front() {
            if !t.is("group by") {
                return Err(SqlError::new("expected order by clause", Syntax));
            }
        } else {
            return Err(SqlError::new("empty token stream passed to where parser", Syntax));
        }

        tokens.pop_front().unwrap();

        node.add_child(self.parse_columns(tokens)?);

        Ok(node)
    }

    fn parse_into(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Into));

        if let Some(t) = tokens.front() {
            if !t.is("into") {
                return Err(SqlError::new("expected order by clause", Syntax));
            }
        } else {
            return Err(SqlError::new("empty token stream passed to where parser", Syntax));
        }

        // skip into
        tokens.pop_front().unwrap();

        // go ahead and add the identifier
        node.add_token(tokens.pop_front().unwrap());

        Ok(node)
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Tokenizer;
    use crate::parser::rdp;
    use crate::parser::rdp::RecursiveDescentParser;

    #[test]
    fn tokenize_basic_select() {
        let t = Tokenizer::new();
        let p = RecursiveDescentParser{};


        let mut tokens = t.tokenize("SELECT a, b, c FROM table".to_string()).unwrap();

        let maybe_tree = p.parse(&mut tokens);

        match maybe_tree {
            Err(e) => assert!(false),
            Ok(tree) => assert!(tree.children.len() > 0)
        }


    }
}





