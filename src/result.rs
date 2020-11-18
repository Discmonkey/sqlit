use crate::table::Table;
use crate::scalar::Scalar;

pub enum Result {
    Table(Table),
    Scalar(Scalar)
}


