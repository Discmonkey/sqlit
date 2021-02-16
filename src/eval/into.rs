use crate::parser::ParserNode;
use crate::table::Table;
use crate::result::SqlResult;

pub (super) fn eval(node: ParserNode, table: &Table) -> SqlResult<()> {

    let (_, mut tokens, _) = node.release();

    let mut filename = tokens.pop_front().unwrap().to_string();

    filename = filename.replace("'", "");

    table.write_to_file(&filename);

    Ok(())
}