use crate::table::{Column, ColumnType};
use std::cmp::Ordering;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::{Runtime, Type};
use std::fmt::Display;
use std::io::Write;

/// apply block returns a non-column, which makes it useful for general vector operations such as len()
macro_rules! apply_block {
    ($column: expr, $value: ident, $block: block) => {
        match $column {
            Column::Booleans($value) => $block,
            Column::Dates($value) => $block,
            Column::Floats($value) => $block,
            Column::Ints($value) => $block,
            Column::Strings($value) => $block,
        }
    }
}

/// map block returns a column, which makes it useful for operations that return another Column object such as limit
macro_rules! map_block {
    ($column: expr, $value: ident, $block: block) => {
        match $column {
            Column::Booleans($value) => Column::Booleans($block),
            Column::Dates($value) => Column::Dates($block),
            Column::Floats($value) => Column::Floats($block),
            Column::Ints($value) => Column::Ints($block),
            Column::Strings($value) => Column::Strings($block),
        }
    }
}

macro_rules! cross_apply {
    ($col_1: expr, $col_2: expr, $v1: ident, $v2: ident, $block: block, $err: block) => {
        match ($col_1, $col_2) {
            (Column::Booleans($v1), Column::Booleans($v2)) => $block,
            (Column::Dates($v1), Column::Dates($v2))  => $block,
            (Column::Floats($v1), Column::Floats($v2))  => $block,
            (Column::Ints($v1), Column::Ints($v2)) => $block,
            (Column::Strings($v1), Column::Strings($v2)) => $block,
            _ => $err
        }
    }
}

fn select<T: Clone>(values: &Vec<T>, selections: &Vec<Option<bool>>) -> Vec<T> {
    values.into_iter().zip(selections.iter()).filter_map(|(val, s)| {
        if let Some(true) = s {
            Some(val.clone())
        } else {
            None
        }
    }).collect()
}

fn order<T: Clone>(values: &Vec<T>, order: &Vec<usize>) -> Vec<T>{
    let mut new_vec: Vec<T> = Vec::new();

    new_vec.resize(values.len(), values[0].clone());

    order.iter().enumerate().for_each(|(num, assignment)| {
        if assignment < &new_vec.len() {
            new_vec[num] = values[*assignment].clone()
        }
    });

    new_vec
}


impl Column {

    pub fn push_null(&mut self) {
        apply_block!(self, v, {
            v.push(None);
        });
    }

    pub fn extend(&mut self, other: &Self) -> SqlResult<()> {
        cross_apply!(self, other, v1, v2, {
            v2.iter().for_each(|value| {
               v1.push(value.clone())
            });

            Ok(())
        }, {
            Err(SqlError::new("cannot extend column with mismatched type", Type))
        })
    }

    pub fn elem_order(&self, i1: usize, i2: usize) -> Ordering {
        if i1 > self.len() || i2 > self.len() {
            return Ordering::Equal;
        }

        match self {
            Column::Booleans(b) => {
                if let (Some(b1), Some(b2)) = (b[i1], b[i2]) {
                    if b1 == b2 {
                        Ordering::Equal
                    } else if b1 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Equal
                }

            },

            Column::Ints(i) => i[i1].cmp(&i[i2]),
            Column::Floats(f) => f[i1].partial_cmp(&f[i2]).unwrap(),
            Column::Dates(d) => d[i1].cmp(&d[i2]),
            Column::Strings(s) => s[i1].cmp(&s[i2])
        }
    }

    pub fn len(&self) -> usize {
        apply_block!(self, v, {
            v.len()
        })
    }

    pub fn limit(&self, size: usize) -> Self {
        map_block!(self, vector, {
            vector.iter().map(|item| item.clone()).take(size).collect()
        })
    }

    /// returns a new empty Column of the same type
    pub fn new_empty(&self) -> Self {
        map_block!(self, v, {
            vec![]
        })
    }

    /// create a new column by concat-ing self and other
    pub fn concat(&self, other: &Self) -> SqlResult<Self>{
        let mut my_clone = self.clone();

        my_clone.extend(other)?;

        Ok(my_clone)
    }

    pub fn order(&self, sort_order: &Vec<usize>) -> Self {
        map_block!(self, v, {
            order(v, sort_order)
        })
    }

    pub fn row(&self, idx: usize) -> Self {
        map_block!(self, v, {
            if idx > v.len() {
                vec![v[0].clone()]
            } else {
                vec![v[idx].clone()]
            }
        })
    }

    pub fn select(&self, selections: &Vec<Option<bool>>) -> Self {
        map_block!(self, v, {
            select(v, selections)
        })
    }

    pub fn type_(&self) -> ColumnType {
        match self {
            Column::Booleans(_) => ColumnType::Boolean,
            Column::Ints(_) => ColumnType::Int,
            Column::Floats(_) => ColumnType::Float,
            Column::Dates(_) => ColumnType::Date,
            Column::Strings(_) => ColumnType::String
        }
    }

    pub fn as_writable(&self, idx: usize, mut writer: &mut dyn Write, null: &str) {
        apply_block!(self, v, {
            let elem = if idx > v.len() {
                v[0].clone()
            } else {
                v[idx].clone()
            };

            if let Some(s) = elem {
                write!(&mut writer, "{}", s);
            } else {
                write!(&mut writer, "{}", null);
            }
        });
    }
}

#[cfg(test)]
mod test {
    use crate::table::Column;

    #[test]
    fn test_order() {
        let c = Column::Ints(vec![1, 2, 3, 4].into_iter().map(Some).collect());
        let order = vec![3, 2, 1, 0];

        let new = c.order(&order);

        match new {
            Column::Ints(mut i) => {
                i.into_iter().zip((1..=4).rev()).for_each(|(a, b)| {
                    assert_eq!(a, Some(b));
                })
            },
            _ => assert!(false)
        }
    }
}