use crate::{Row, Cell};

impl Row<'_> {
    pub fn value(&self) -> &Vec<Cell> {
        match self {
            Row::Borrowed(x) => &x,
            Row::Owned(x) => x,
        }
    }
}