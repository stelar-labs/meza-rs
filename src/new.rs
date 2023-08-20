use opis::Fraction;

use crate::{Meza, Column};

impl<'a> Meza<'a> {
    pub fn new(columns: Vec<&str>) -> Meza<'a> {
        let column_objs: Vec<Column> = columns
            .iter()
            .map(|&header| Column {
                average: Fraction::default(),
                header: header.to_string(),
                variance: Fraction::default(),
                sum: Fraction::default(),
            })
            .collect();

        Meza {
            columns: column_objs,
            rows: Vec::new(),
        }
    }
}