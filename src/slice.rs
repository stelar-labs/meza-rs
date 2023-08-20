use std::error::Error;

use crate::{Meza, Row, Column, Cell};

impl<'a> Meza<'a> {

    pub fn row_slice(&'a self, start: usize, end: usize) -> Result<Meza<'a>, Box<dyn Error>> {
        
        if start > end || end > self.rows.len() {
            return Err("Invalid row indices".into());
        }

        let columns: Vec<Column> = self
            .columns
            .iter()
            .map(|column| {
                let mut cloned_column = column.clone();
                cloned_column.reset_statistics();
                cloned_column
            })
            .collect();

        let rows: Vec<Row<'a>> = self.rows[start..end]
            .iter()
            .map(|row| Row::Borrowed(row.value()))
            .collect();

        Ok(Meza { columns, rows })

    }

    pub fn column_slice(
        &'a self,
        columns: Vec<&str>,
    ) -> Result<Meza<'a>, Box<dyn Error>> {
        let column_indices: Vec<usize> = columns
            .iter()
            .map(|&col| self.column_index(col))
            .collect::<Result<Vec<usize>, _>>()?;

        self.index_column_slice(column_indices)
    }

    pub fn index_column_slice(
        &'a self,
        column_indices: Vec<usize>,
    ) -> Result<Meza<'a>, Box<dyn Error>> {
        let mut new_columns: Vec<Column> = Vec::new();

        for &index in &column_indices {
            if index < self.columns.len() {
                let mut cloned_column = self.columns[index].clone();
                cloned_column.reset_statistics();
                new_columns.push(cloned_column);
            } else {
                return Err("Invalid column index".into());
            }
        }

        let new_rows: Vec<Row<'a>> = self.rows.iter().map(|row| {
            let new_cells: Vec<Cell<'a>> = column_indices
                .iter()
                .map(|&index| {
                    if index < row.value().len() {
                        Cell::Borrowed(row.value()[index].value())
                    } else {
                        panic!("Invalid column index");
                    }
                })
                .collect();
            Row::Owned(new_cells)
        }).collect();

        Ok(Meza {
            columns: new_columns,
            rows: new_rows,
        })
    }

}
