use crate::converters::{Converter, ToBool, ToDate, ToFloat, ToInt};
use crate::column;
use crate::column::Column;

/// This method uses converters to figure out the correct type and create a column
pub fn fill_feeds(raw_values: Vec<String>, feeds: Vec<Box<dyn Feed>>) -> column::Column {

    let mut success = true;

    for mut feed in feeds {

        for i in 0..rows_to_try {
            if !feed.ingest(raw_values[i].as_str()) {
                success = false;
                break;
            }
        }

        if success {
            return feed.into_column()
        }

    }

    column::Column::Strings(raw_values)
}

pub struct ColumnBuilder {
    num_rounds: usize,
    column_type_builders: Vec<Box<dyn Feed>>
}

struct ColumnTypeBuilder<T> {
    values: Vec<T>,
    valid: bool,
    priority: i32,
    converter: Box<dyn Converter<T>>
}

trait Feed {
    fn ingest(&mut self, raw_value: &str) -> bool;
    fn into_column(mut self) -> column::Column;
}

impl ColumnTypeBuilder<T> {
    pub fn new<T>(converter: Box<dyn Converter<T>>) -> Self<T> {
        Self {
            values: Vec::new(),
            valid: true,
            priority: 0,
            converter
        }
    }
}

impl Feed for ColumnTypeBuilder<T> {
    fn ingest(&mut self, raw_value: &str) -> bool {
        match self.converter.convert(raw_value) {

            Some(v) => {
                self.values.push(v);
                true
            },

            None => {
                self.valid = false;
                false
            }
        }
    }

    fn into_column(mut self) -> Column {
        self.converter.make_column(self.values)
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}



const NUM_ROWS_BEFORE_INVALIDATION: usize = 10;

/// ColumnBuilder reads in &str values and attempts to create a column from the values.
///
impl ColumnBuilder {
    pub fn new() -> Self {
        let mut column_type_builders = vec!(
            Box::new(ColumnTypeBuilder::new(Box::new(ToBool{}))),
            Box::new(ColumnTypeBuilder::new(Box::new(ToDate::new()))),
            Box::new(ColumnTypeBuilder::new(Box::new(ToInt{}))),
            Box::new(ColumnTypeBuilder::new(Box::new(ToFloat{}))),
            Box::new(ColumnTypeBuilder::new(Box::new(ToString{})))
        );

        ColumnBuilder {
            num_rounds: 0,
            column_type_builders
        }
    }

    pub fn ingest(&mut self, value: &str) {
        self.num_rounds += 1;

        if self.num_rounds == NUM_ROWS_BEFORE_INVALIDATION {
            let mut found = false;

            if self.boolean_builder.is_valid() {
                found = true;
            } else {

            } else {

            } else {
                
            }
        }
    }

    pub fn finish(&mut self) -> column::Column {

    }
}



