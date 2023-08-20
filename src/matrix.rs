use std::error::Error;

use opis::{Fraction, Matrix};

use crate::{Meza, Datum};

impl<'a> Into<Matrix<Fraction>> for Meza<'a> {
    fn into(self) -> Matrix<Fraction> {
        let mut matrix_data: Vec<Vec<Fraction>> = Vec::new();

        for row in &self.rows {
            let matrix_row: Vec<Fraction> = row
                .value()
                .iter()
                .map(|cell| match cell.value().try_into() {
                    Ok(fraction) => fraction,
                    Err(_) => Fraction::zero(),
                })
                .collect();

            matrix_data.push(matrix_row);
        }

        Matrix(matrix_data)
    }
}

// impl<'a> TryInto<Matrix<Fraction>> for Meza<'a> {

//     fn try_into(self) -> Result<Matrix<Fraction>, Box<dyn Error>> {
//         let mut matrix_data: Vec<Vec<Fraction>> = Vec::new();

//         for row in &self.rows {
//             let mut matrix_row: Vec<Fraction> = Vec::new();

//             for cell in row.value() {
//                 let cell_value: &Datum = cell.value();
//                 match cell_value.try_into() {
//                     Ok(fraction) => matrix_row.push(fraction),
//                     Err(err) => return Err(err.into()),
//                 }
//             }

//             matrix_data.push(matrix_row);
//         }

//         Ok(Matrix(matrix_data))
//     }

//     type Error = Box<dyn Error>;
// }