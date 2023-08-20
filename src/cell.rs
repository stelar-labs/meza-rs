use std::error::Error;

use opis::Fraction;

use crate::{Cell, Datum};

impl Cell<'_> {
    pub fn value(&self) -> &Datum {
        match self {
            Cell::Borrowed(x) => &x,
            Cell::Owned(x) => x,
        }
    }
}

// impl TryInto<Fraction> for Cell<'_> {
//     fn try_into(self) -> Result<Fraction, Box<dyn Error>> {
//         (&self).try_into()
//     }
//     type Error = Box<dyn Error>;
// }

impl TryInto<Fraction> for &Cell<'_> {
    fn try_into(self) -> Result<Fraction, Box<dyn Error>> {
        match self.value() {
            Datum::String(string) => Fraction::try_from(&string[..]),
            Datum::Integer(integer) => Ok(integer.into()),
            Datum::Fraction(fraction) => Ok(fraction.clone()),
        }
    }

    type Error = Box<dyn Error>;
}
