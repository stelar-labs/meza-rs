use std::{cmp::Ordering, fmt, error::Error};

use opis::Fraction;

use crate::Datum;

impl fmt::Display for Datum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Datum::String(s) => write!(f, "{}", s),
            Datum::Integer(val) => write!(f, "{}", val),
            Datum::Fraction(val) => write!(f, "{}", val),
        }
    }
}

impl PartialOrd for Datum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Datum::String(s1), Datum::String(s2)) => s1.partial_cmp(s2),
            (Datum::Integer(i1), Datum::Integer(i2)) => i1.partial_cmp(i2),
            (Datum::Fraction(f1), Datum::Fraction(f2)) => f1.partial_cmp(f2),
            _ => None,
        }
    }
}

impl TryInto<Fraction> for Datum {
    fn try_into(self) -> Result<Fraction, Box<dyn Error>> {
        (&self).try_into()
    }
    type Error = Box<dyn Error>;
}

impl TryInto<Fraction> for &Datum {
    fn try_into(self) -> Result<Fraction, Box<dyn Error>> {
        match self {
            Datum::String(string) => Fraction::try_from(&string[..]),
            Datum::Integer(integer) => Ok(integer.into()),
            Datum::Fraction(fraction) => Ok(fraction.clone()),
        }
    }

    type Error = Box<dyn Error>;
}