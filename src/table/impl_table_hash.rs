use crate::table::{Column, Table};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

impl Table {

    pub fn hash_row(&self, idx: usize) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.columns.iter().for_each(|column| {
            match column {
                Column::Booleans(v) => v[idx].hash(&mut hasher),
                Column::Ints(v) => v[idx].hash(&mut hasher),
                Column::Floats(v) => ((v[idx] * 1e6).round() as i64).hash(&mut hasher),
                Column::Strings(v) => v[idx].hash(&mut hasher),
                Column::Dates(v) => v[idx].hash(&mut hasher),
            }
        });

        hasher.finish()
    }
}