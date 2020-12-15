use crate::column::Column;
use crate::scalar::Scalar;

pub enum Evaluated {
    Column(Column),
    Scalar(Scalar),
    Value(String),
}

pub type SqlResult = std::result::Result<Evaluated, String>;

