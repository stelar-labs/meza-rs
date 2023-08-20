use std::error::Error;

use crate::{Datum, Meza, Row, Cell};

impl Meza<'_> {
    pub fn index_column_apply<F>(&mut self, action: F, column_index: usize) -> Result<(), Box<dyn Error>>
        where F: Fn(&Datum) -> Datum {
        if column_index >= self.columns.len() {
            return Err("Column index out of range!".into());
        }
        for row in &mut self.rows {
            match row {
                Row::Borrowed(_) => {
                    let mut owned_row = match row {
                        Row::Borrowed(cells) => Row::Owned(cells
                            .iter()
                            .map(|cell| cell.clone())
                            .collect()
                        ),
                        _ => Row::Owned(vec![]),
                    };
                    if let Row::Owned(ref mut cells) = owned_row {
                        if let Some(cell) = cells.get_mut(column_index) {
                            let transformed_datum = action(cell.value());
                            *cell = Cell::Owned(transformed_datum);
                        }
                    }
                    *row = owned_row;
                }
                Row::Owned(cells) => {
                    if let Some(cell) = cells.get_mut(column_index) {
                        let transformed_datum = action(cell.value());
                        *cell = Cell::Owned(transformed_datum);
                    }
                }
            }
        }
        
        Ok(())

    }
    pub fn column_apply<F>(&mut self, action: F, column: &str) -> Result<(), Box<dyn Error>>
        where F: Fn(&Datum) -> Datum {
        let column_index = self.column_index(column)?;
        self.index_column_apply(action, column_index)
    }

}