use std::error::Error;

use crate::{Meza, Cell};

impl<'a> Meza<'a> {

    pub fn sort(&mut self, column: &str, order: bool) -> Result<(), Box<dyn Error>> {

        let column_index = self
            .columns
            .iter()
            .position(|x| x.header == column)
            .ok_or_else(|| Box::<dyn Error>::from("No column!"))?;

        self.index_sort(column_index, order)

    }

    pub fn index_sort(&mut self, column_index: usize, order: bool) -> Result<(), Box<dyn Error>> {

        if column_index >= self.columns.len() {
            return Err("Invalid column index".into());
        }
        
        self.rows.sort_by(|row1, row2| {
            let cell1 = &row1.value()[column_index].value();
            let cell2 = &row2.value()[column_index].value();

            let cmp_result = cell1.cmp(cell2);

            if order {
                cmp_result
            } else {
                cmp_result.reverse()
            }
        });

        Ok(())

    }

}
