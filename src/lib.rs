
mod apply;
mod cell;
mod column;
mod csv;
mod datum;
mod filter;
mod matrix;
mod new;
mod row;
mod search;
mod slice;
mod sort;
mod statistics;

use opis::{Integer, Fraction};

#[derive(Clone, Debug)]
pub struct Meza<'a> {
    pub columns: Vec<Column>,
    pub rows: Vec<Row<'a>>,
}

#[derive(Clone, Debug)]
pub struct Column {
    pub average: Fraction,
    pub header: String,
    pub variance: Fraction,
    pub sum: Fraction
}

#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq)]
pub enum Datum {
    String(String),
    Integer(Integer),
    Fraction(Fraction)
}

#[derive(Clone, Debug)]
pub enum Row<'a> {
    Borrowed(&'a Vec<Cell<'a>>),
    Owned(Vec<Cell<'a>>)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Cell<'a> {
    Borrowed(&'a Datum),
    Owned(Datum)
}
