use crate::table::{Column, ColumnType};
use std::cmp::Ordering;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::Runtime;

macro_rules! apply {
    ($column: expr, $method: tt, $($arg:expr),*) => {
        match $column {
            Column::Booleans(b) => b.$method($($arg,)*),
            Column::Dates(d) => d.$method($($arg,)*),
            Column::Floats(f) => f.$method($($arg,)*),
            Column::Ints(i) => i.$method($($arg,)*),
            Column::Strings(s) => s.$method($($arg,)*),
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
    pub fn len(&self) -> usize {
        apply!(self, len,)
    }

    pub fn select(&self, selections: &Vec<Option<bool>>) -> Self {
        match self {
            Column::Booleans(v) => Column::Booleans(select(v, selections)),
            Column::Ints(v) => Column::Ints(select(v, selections)),
            Column::Floats(v) => Column::Floats(select(v, selections)),
            Column::Strings(v) => Column::Strings(select(v, selections)),
            Column::Dates(v) => Column::Dates(select(v, selections)),
        }
    }

    pub fn order(&self, sort_order: &Vec<usize>) -> Self {
        match self {
            Column::Booleans(v) => Column::Booleans(order(v, sort_order)),
            Column::Ints(v) => Column::Ints(order(v, sort_order)),
            Column::Floats(v) => Column::Floats(order(v, sort_order)),
            Column::Strings(v) => Column::Strings(order(v, sort_order)),
            Column::Dates(v) => Column::Dates(order(v, sort_order)),
        }
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

    pub fn merge(&self, other: &Self) -> SqlResult<Self>{
        macro_rules! other {
            ($v1:ident, $t2:ident, $other:ident) => {
                 if let Column::$t2(v2) = $other {
                    v2.into_iter().for_each(|val| {
                        $v1.push(val.clone());
                    })
                } else {
                    return Err(SqlError::new("mismatched type on column merge", Runtime));
                }
            }
        }

        let mut my_clone = self.clone();

        match &mut my_clone {
            Column::Booleans(v1) => other!(v1, Booleans, other),
            Column::Ints(v1) => other!(v1, Ints, other),
            Column::Floats(v1) => other!(v1, Floats, other),
            Column::Strings(v1) => other!(v1, Strings, other),
            Column::Dates(v1) => other!(v1, Dates, other),
        };

        Ok(my_clone)
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