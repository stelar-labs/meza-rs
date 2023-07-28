use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::path::Path;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct Cell {
    pub float32: Option<f32>,
    pub string: String
}

#[derive(Clone, Debug)]
pub struct Meza<'a> {
    pub averages: Vec<Option<f32>>,
    pub columns: Vec<String>,
    pub rows: Vec<&'a Vec<Cell>>,
    pub column_types: Vec<ColumnType>,
    pub data: Vec<Vec<Cell>>
}

#[derive(Clone, Debug)]
pub enum ColumnType {
    String,
    Float32
}

impl<'a> Meza<'a> {

    pub fn from_csv(csv_path: &str) -> Result<Meza<'a>, Box<dyn Error>> {
        
        let csv_open = File::open(csv_path)?;
        
        let csv_buffer = BufReader::new(csv_open);

        let mut csv_lines = csv_buffer.lines();
        
        let column_line = csv_lines.next().ok_or("Empty CSV file")??;
        
        let columns: Vec<String> = column_line.split(',').map(|y| y.trim().to_string()).collect();
        
        let column_types: Vec<ColumnType> = vec![ColumnType::String; columns.len()];

        let averages = vec![None; columns.len()];
        
        let mut data: Vec<Vec<Cell>> = Vec::new();

        for line_result in csv_lines {
            let line = line_result?;
            let row: Vec<String> = line.split(',').map(|y| y.trim().to_string()).collect();
            let cells = row
                .iter()
                .map(|x| Cell {
                    float32: None,
                    string: x.to_string(),
                })
                .collect();

            data.push(cells);
        }

        Ok(Meza {
            columns,
            rows: Vec::new(),
            column_types,
            data,
            averages,
        })

    }

    pub fn to_csv(&self, csv_path: &str) -> Result<(), Box<dyn Error>> {
        
        if Path::new(csv_path).is_file() {
            fs::remove_file(csv_path)?;
        }

        let mut csv_file = OpenOptions::new().write(true).create(true).open(csv_path)?;

        let column_str = self.columns.join(",");
        writeln!(csv_file, "{}", column_str)?;

        if !self.data.is_empty() {
            for row in &self.data {
                let row_str = row.iter().map(|cell| &cell.string[..]).collect::<Vec<&str>>().join(",");
                writeln!(csv_file, "{}", row_str)?;
            }
        } else {
            for row in &self.rows {
                let row_str = row.iter().map(|cell| &cell.string[..]).collect::<Vec<&str>>().join(",");
                writeln!(csv_file, "{}", row_str)?;
            }
        }

        Ok(())
    }

    pub fn update_csv(&self, csv_path: &str) -> Result<(), Box<dyn Error>> {
        
        if self.data.is_empty() && self.rows.is_empty() {
            return Ok(());
        }

        let mut csv_file = OpenOptions::new().write(true).create(true).truncate(true).open(csv_path)?;

        if !self.data.is_empty() {
            for row in self.data.iter() {
                let row_str = row.iter().map(|cell| &cell.string[..]).collect::<Vec<&str>>().join(",");
                writeln!(csv_file, "{}", row_str)?;
            }
        } else {
            for row in self.rows.iter() {
                let row_str = row.iter().map(|cell| &cell.string[..]).collect::<Vec<&str>>().join(",");
                writeln!(csv_file, "{}", row_str)?;
            }
        }

        Ok(())

    }

    pub fn sort(&mut self, column: &str, order: bool) -> Result<(), Box<dyn Error>> {
        
        let i = self
            .columns
            .iter()
            .position(|x| x == column)
            .ok_or_else(|| Box::<dyn Error>::from("No column!"))?;

        if self.rows.is_empty() {
            match &self.column_types[i] {
                ColumnType::String => self.data.sort_by(|a, b| a[i].string.cmp(&b[i].string)),
                ColumnType::Float32 => self.data.sort_by(|a, b| a[i].float32.partial_cmp(&b[i].float32).unwrap()),
            }
        } else {
            match &self.column_types[i] {
                ColumnType::String => self.rows.sort_by(|a, b| a[i].string.cmp(&b[i].string)),
                ColumnType::Float32 => self.rows.sort_by(|a, b| a[i].float32.partial_cmp(&b[i].float32).unwrap()),
            }
        }

        if !order {
            if self.rows.is_empty() {
                self.data.reverse();
            } else {
                self.rows.reverse();
            }
        }

        Ok(())
    }

    pub fn filter<F: Fn(&Vec<Cell>) -> bool>(&self, criteria: F) -> Meza {
        
        let mut meza = Meza {
            columns: self.columns.clone(),
            rows: vec![],
            column_types: self.column_types.clone(),
            data: vec![],
            averages: vec![None; self.columns.len()],
        };

        if self.rows.is_empty() {
            meza.data = self.data.iter().cloned().filter(|row| criteria(row)).collect();
        } else {
            meza.rows = self.rows.iter().cloned().filter(|row| criteria(row)).collect();
        }

        meza

    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty() && self.data.is_empty()
    }

    pub fn new(columns: Vec<&str>) -> Meza {
        Meza {
            columns: columns.iter().map(|x| x.to_string()).collect(),
            rows: vec![],
            column_types: vec![ColumnType::String; columns.len()],
            data: vec![],
            averages: vec![None; columns.len()],
        }
    }

    pub fn column(&self, name: &str) -> Result<Vec<&Cell>, Box<dyn Error>> {
        
        let index = self
            .columns
            .iter()
            .position(|x| x == name)
            .ok_or_else(|| Box::<dyn Error>::from("No column!"))?;

        let column = if self.rows.is_empty() {
            self.data.iter().map(|row| &row[index]).collect()
        } else {
            self.rows.iter().map(|row| &row[index]).collect()
        };

        Ok(column)

    }

    pub fn dedup(&mut self) {

        if self.rows.is_empty() {
            self.data.sort();
            self.data.dedup();
        } else {
            self.rows.sort();
            self.rows.dedup();
        }

    }
    
    pub fn slice(&self, start: usize, end: usize) -> Meza {
        let mut meza = Meza {
            columns: self.columns.clone(),
            data: vec![],
            rows: vec![],
            column_types: self.column_types.clone(),
            averages: vec![None; self.columns.len()],
        };

        if self.rows.is_empty() {
            meza.data = if end <= self.data.len() {
                self.data[start..end].to_vec()
            } else {
                self.data.clone()
            };
        } else {
            meza.rows = if end <= self.rows.len() {
                self.rows[start..end].to_vec()
            } else {
                self.rows.clone()
            };
        }

        meza
    }

    pub fn average(&mut self, column: &str) -> Result<f32, Box<dyn Error>> {
        
        let i = match self.columns.iter().position(|x| x == column) {
            Some(i) => i,
            None => Err("Column not found!")?,
        };
        
        let mut sum = 0.0;

        let mut count = 0;

        if self.data.is_empty() {
            for row in &self.rows {
                if let Some(cell) = row.get(i) {
                    match cell.float32 {
                        Some(value) => {
                            sum += f32::from(value);
                            count += 1;
                        }
                        None => continue,
                    }
                }
            }
        } else {
            for row in &self.data {
                if let Some(cell) = row.get(i) {
                    match cell.float32 {
                        Some(value) => {
                            sum += f32::from(value);
                            count += 1;
                        }
                        None => continue,
                    }
                }
            }
        }
        
        let avg = if count > 0 {
            sum / count as f32
        } else {
            0.0
        };

        self.averages[i] = Some(avg);

        Ok(avg)

    }
    
    pub fn variance(&mut self, column: &str) -> Result<f32, Box<dyn Error>> {
        
        let i = self
            .columns
            .iter()
            .position(|x| x == column)
            .expect("Column not found!");

        let average = match self.averages[i] {
            Some(avg) => avg,
            None => self.average(column)?
        };

        let values: Vec<f32> = if self.rows.is_empty() {
            self.data
                .iter()
                .filter_map(|row| row[i].float32)
                .collect()
        } else {
            self.rows
                .iter()
                .filter_map(|row| row[i].float32)
                .collect()
        };

        let squared_differences: Vec<f32> = values
            .iter()
            .map(|value| (value - average).powi(2))
            .collect();

        let sum_squared_differences: f32 = squared_differences.iter().sum();

        let variance = sum_squared_differences / (values.len() as f32);

        Ok(variance)

    }

}

use std::cmp::Ordering;

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.string.cmp(&other.string)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Cell {}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}
