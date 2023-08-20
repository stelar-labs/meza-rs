use opis::Fraction;

use crate::{Meza, Datum};

impl<'a> Meza<'a> {

    pub fn calculate_sum(&mut self, column: &str) {
        if let Some(column_index) = self.column_index(column).ok() {
            self.index_calculate_sum(column_index);
        }
    }

    pub fn index_calculate_sum(&mut self, column_index: usize) {
        
        if column_index < self.columns.len() {

            let mut sum = Fraction::default();

            for row in &self.rows {
                
                match row.value().get(column_index) {
                    Some(cell) => {
                        match cell.value() {
                            Datum::String(string) => match Fraction::try_from(&string[..]) {
                                Ok(fraction) => sum += fraction,
                                Err(_) => (),
                            },
                            Datum::Integer(integer) => sum += integer,
                            Datum::Fraction(fraction) => sum += fraction,
                        }
                    },
                    None => (),
                }
            }
            
            self.columns[column_index].sum = sum;

        }
    }

    pub fn calculate_average(&mut self, column: &str) {
        if let Some(column_index) = self.column_index(column).ok() {
            self.index_calculate_average(column_index);
        }
    }

    pub fn index_calculate_average(&mut self, column_index: usize) {

        if column_index < self.columns.len() {
            if !self.rows.is_empty() {
                let sum = &self.columns[column_index].sum;
                let rows = Fraction::from(&self.rows.len());
                self.columns[column_index].average = (sum / &rows).unwrap()
            }
        }

    }

    pub fn calculate_variance(&mut self, column: &str) {
        if let Some(column_index) = self.column_index(column).ok() {
            self.index_calculate_variance(column_index);
        }
    }

    pub fn index_calculate_variance(&mut self, column_index: usize) {
        if column_index < self.columns.len() {
            let average = &self.columns[column_index].average;
            if self.rows.len() > 1 {
                let mut variances = Fraction::default();
                for row in &self.rows {
                    match row.value().get(column_index) {
                        Some(cell) => {
                            let cell_fraction: Result<Fraction,_> = cell.try_into();
                            match cell_fraction {
                                Ok(fraction) => {
                                    let diff = &fraction - average;
                                    let diff_sqr = &diff * &diff;
                                    variances += diff_sqr
                                },
                                Err(_) => (),
                            }
                        },
                        None => (),
                    }
                }
                self.columns[column_index].variance = (variances / Fraction::from(&(self.rows.len() - 1))).unwrap()
            }
        }
    }
}
