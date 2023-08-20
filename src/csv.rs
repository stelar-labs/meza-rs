use std::{fs::{File, self}, error::Error, io::{BufReader, BufRead, Write}};

use opis::Fraction;

use crate::{Meza, Column, Datum, Cell, Row};

impl<'a> Meza<'a> {

        pub fn from_csv(csv_path: &str) -> Result<Meza<'a>, Box<dyn Error>> {
            
            let csv_open = File::open(csv_path)?;
            
            let csv_buffer = BufReader::new(csv_open);
    
            let mut csv_lines = csv_buffer.lines();
            
            let column_line = csv_lines.next().ok_or("Empty CSV file")??;
            
            let column_headers: Vec<String> = column_line.split(',').map(|y| y.trim().to_string()).collect();
            
            let columns: Vec<Column> = column_headers
                .iter()
                .map(|x| Column {
                    average: Fraction::default(),
                    header: x.clone(),
                    variance: Fraction::default(),
                    sum: Fraction::default(),
                })
                .collect();

            let mut meza = Meza {
                columns,
                rows: Vec::new(),
            };
        
            for line_result in csv_lines {

                let line = line_result?;
    
                let row: Vec<&str> = line.split(',').map(|item| item.trim()).collect();

                let cell_rows = row
                    .iter()
                    .map(|x| Cell::Owned(Datum::String(x.to_string())))
                    .collect();
    
                meza.rows.push(Row::Owned(cell_rows));

            }
            
            Ok(meza)
    
        }
            
        pub fn to_csv(&self, csv_path: &str) -> Result<(), Box<dyn Error>> {
            
            if fs::metadata(csv_path).is_ok() {
                fs::remove_file(csv_path)?;
            }
    
            let mut csv_file = File::create(csv_path)?;
            
            let header_line = self.columns
                .iter()
                .map(|x| x.header.as_str())
                .collect::<Vec<&str>>()
                .join(",") + "\n";

            csv_file.write_all(header_line.as_bytes())?;

            for row in &self.rows {
                
                let row_line = row.value().iter()
                    .map(|x| x.value().to_string())
                    .collect::<Vec<String>>()
                    .join(",") + "\n";

                csv_file.write_all(row_line.as_bytes())?;

            }
    
            Ok(())
        }

    }