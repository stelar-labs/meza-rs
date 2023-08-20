use std::error::Error;

use crate::{Meza, Cell, Row};

impl<'a> Meza<'a> {

    pub fn row_search(
        &self,
        key_column: &str,
        key: Cell
    ) -> Result<&Row<'a>, Box<dyn Error>> {
        let key_index = self.column_index(key_column)?;
        self.index_row_search(key_index, key)
    }

    pub fn index_row_search(
        &self,
        key_index: usize,
        key: Cell
    ) -> Result<&Row<'a>, Box<dyn Error>> {
        if key_index >= self.columns.len() {
            return Err("Invalid key index".into());
        }

        let found_row = self
            .rows
            .iter()
            .find(|&row| row.value()[key_index] == key);

        match found_row {
            Some(row) => Ok(row),
            None => Err("Row not found".into()),
        }
    }

    pub fn index_cell_search(
        &self,
        key_index: usize,
        value_index: usize,
        key: Cell,
    ) -> Result<&Cell, Box<dyn Error>> {
        if key_index >= self.columns.len() || value_index >= self.columns.len() {
            return Err("Invalid key or value index".into());
        }

        let found_cell = self
            .rows
            .iter()
            .find(|&row| row.value()[key_index] == key)
            .map(|row| &row.value()[value_index]);

        match found_cell {
            Some(cell) => Ok(cell),
            None => Err("Cell not found".into()),
        }
    }

    pub fn cell_search(
        &self,
        key_column: &str,
        value_column: &str,
        key: Cell,
    ) -> Result<&Cell, Box<dyn Error>> {
        let key_index = self.column_index(key_column)?;
        let value_index = self.column_index(value_column)?;
        self.index_cell_search(key_index, value_index, key)
    }

}