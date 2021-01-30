use crate::ops::MapOp;
use crate::result::{SqlResult, SqlError};
use crate::table::Column;

pub (super) struct IsNull {

}

pub (super) struct NotNull {

}

macro_rules! to_boolean {
    ($vec:ident, $method:tt) => {
        Column::Booleans($vec.into_iter().map(|v| Some(v.$method())).collect())
    }
}

impl MapOp for IsNull {
    fn apply(&self, mut arguments: &Vec<Column>) -> SqlResult<Column> {
        arg_check!(1, arguments, "is_null", !=);

        Ok(match &arguments[0]{
            Column::Dates(d) => to_boolean!(d, is_none),
            Column::Strings(s) => to_boolean!(s, is_none),
            Column::Booleans(b) => to_boolean!(b, is_none),
            Column::Ints(i) => to_boolean!(i, is_none),
            Column::Floats(f) => to_boolean!(f, is_none),
        })
    }
}

impl MapOp for NotNull {
    fn apply(&self, mut arguments: &Vec<Column>) -> SqlResult<Column> {
        arg_check!(1, arguments, "not_null", !=);

        Ok(match &arguments[0] {
            Column::Dates(d) => to_boolean!(d, is_some),
            Column::Strings(s) => to_boolean!(s, is_some),
            Column::Booleans(b) => to_boolean!(b, is_some),
            Column::Ints(i) => to_boolean!(i, is_some),
            Column::Floats(f) => to_boolean!(f, is_some),
        })
    }
}