use crate::table::Column;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::Runtime;

#[derive(PartialOrd, PartialEq)]
pub (super) enum MapType {
    LL,
    SL,
    LS,
}

pub (super) struct AnnotatedInput {
    pub left: Column,
    pub right: Column,
    pub sizes: MapType,
}

pub (super) fn prepare_binary_args(mut input: Vec<Column>) -> SqlResult<AnnotatedInput> {
    if input.len() != 2 {
        return Err(SqlError::new("incorrect number of arguments for binary op", Runtime));
    }

    let right = input.pop().unwrap();
    let left = input.pop().unwrap();

    if left.len() == right.len() {
        Ok(AnnotatedInput {left, right, sizes: MapType::LL})
    } else if left.len() == 1 {
        Ok(AnnotatedInput {left, right, sizes: MapType::SL})
    } else if right.len() == 1 {
        Ok(AnnotatedInput {left, right, sizes: MapType::LS})
    } else {
        Err(SqlError::new("mismatched column lengths in binary ops", Runtime))
    }
}

#[macro_export]
macro_rules! binary_iterate {
    ($l:expr, $r:expr, $sizes:expr, |($a:ident, $b:ident)| $block:block) => {
        {
            let l = $l.into_iter();
            let r = $r.into_iter();

            (if $sizes == MapType::SL {
                l.cycle().zip(r).map(|($a, $b)| $block).collect()
            } else if $sizes == MapType::LS {
                l.zip(r.cycle()).map(|($a, $b)| $block).collect()
            } else {
                l.zip(r).map(|($a, $b)| $block).collect()
            })
        }
    }
}

#[macro_export]
macro_rules! bb {
    ($l:ident, $r:ident) => {
        (Column::Booleans($l), Column::Booleans($r))
    }
}

#[macro_export]
macro_rules! ff {
    ($l:ident, $r:ident) => {
        (Column::Floats($l), Column::Floats($r))
    }
}

#[macro_export]
macro_rules! ii {
    ($l:ident, $r:ident) => {
        (Column::Ints($l), Column::Ints($r))
    }
}

#[macro_export]
macro_rules! dd {
    ($l:ident, $r:ident) => {
        (Column::Dates($l), Column::Dates($r))
    }
}

#[macro_export]
macro_rules! ss {
    ($l:ident, $r:ident) => {
        (Column::Strings($l), Column::Strings($r))
    }
}

#[macro_export]
macro_rules! fi {
    ($l:ident, $r:ident) => {
        (Column::Floats($l), Column::Ints($r))
    }
}

#[macro_export]
macro_rules! if_ {
    ($l:ident, $r:ident) => {
        (Column::Ints($l), Column::Floats($r))
    }
}





#[cfg(test)]
mod test {
    use crate::ops::binary_ops::MapType;

    #[test]
    fn test_binary_iterate() {
        let mut left = vec![1, 2, 3];
        let mut right = vec![1, 2, 3];
        let mut t = MapType::LL;
        let mut output : Vec<usize> = binary_iterate!(left, right, t, |(a, b)| {a + b});

        for (i, num) in vec![2, 4, 6].into_iter().enumerate() {
            assert_eq!(num, output[i]);
        }

        left = vec![1];
        right = vec![1, 2, 3];
        t = MapType::SL;

        let output : Vec<usize> = binary_iterate!(left, right, t, |(a, b)| {a + b});

        for (i, num) in vec![2, 3, 4].into_iter().enumerate() {
            assert_eq!(num, output[i]);
        }

        let left = vec![1, 2, 3];
        let right = vec![1];
        let t = MapType::LS;

        let output : Vec<usize> = binary_iterate!(left, right, t, |(a, b)| {a + b});

        for (i, num) in vec![2, 3, 4].into_iter().enumerate() {
            assert_eq!(num, output[i]);
        }
    }
}
