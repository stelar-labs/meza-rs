use std::error::Error;
use opis::Fraction;

use crate::{Meza, Column};

impl Column {
    pub fn reset_statistics(&mut self) {
        self.average = Fraction::default();
        self.sum = Fraction::default();
        self.variance = Fraction::default();
    }
}

impl<'a> Meza<'a> {
    
    pub fn column_index(&self, column: &str) -> Result<usize, Box<dyn Error>> {
        self.columns
            .iter()
            .position(|x| x.header == column)
            .ok_or_else(|| Box::<dyn Error>::from("No column!"))
    }
}
