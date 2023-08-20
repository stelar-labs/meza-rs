use crate::{Meza, Row};

impl<'a> Meza<'a> {
    pub fn size(&self) -> (usize, usize) {
        (self.rows.len(), self.columns.len())
    }
    pub fn filter<F>(&'a self, criteria: F) -> Meza<'a>
    where
        F: Fn(&Row<'a>) -> bool,
    {
        let filtered_rows: Vec<Row<'a>> = self.rows
            .iter()
            .filter(|&row| criteria(row))
            .map(|row| {
                match row {
                    Row::Borrowed(_) => row.clone(),
                    Row::Owned(cells) => Row::Borrowed(cells),
                }
            })
            .collect();

        Meza {
            columns: self.columns.clone(),
            rows: filtered_rows,
        }
    }
}