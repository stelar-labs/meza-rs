use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use std::io::Write;

#[derive(Clone, Debug)]
pub enum MezaType {
    String,
    Integer,
    Floating
}
#[derive(Clone, Debug)]
pub struct Meza<'a> {
    pub columns: Vec<String>,
    pub rows: Vec<&'a Vec<String>>,
    pub types: Vec<MezaType>,
    pub data: Vec<Vec<String>>
}

impl Meza<'_> {

    pub fn from_csv(csv_path: &str) -> Result<Meza, Box<dyn Error>> {
        
        let csv_open = File::open(csv_path)?;

        let csv_buffer = BufReader::new(csv_open);

        let mut data: Vec<Vec<String>> = vec![];
        
        let mut csv_lines = csv_buffer.lines();

        let column_line = csv_lines.next().unwrap();

        let column_line = column_line?;

        let columns: Vec<String> = column_line.split(',').map(|y| y.into()).collect();

        let types: Vec<MezaType> = vec![MezaType::String; columns.len()];

        loop {

            match csv_lines.next() {

                Some(line) => {

                    let line = line?;

                    let row: Vec<String> = line.split(',').map(|y| y.into()).collect();

                    data.push(row)
                    
                },

                None => break

            }
            
        }

        let result  = Meza { columns, rows: vec![], types, data };

        Ok(result)

    }

    pub fn to_csv(&self, csv_path: &str) -> Result<(), Box<dyn Error>> {

        if Path::new(csv_path).is_file() {
            
            fs::remove_file(csv_path)?;

        };

        let mut csv_file = OpenOptions::new().append(true).create(true).open(csv_path)?;

        let mut column_str = self.columns[1..]
            .iter()
            .fold(self.columns[0].clone(), |acc, x| format!("{},{}", acc, x));

        column_str.push_str("\n");

        write!(csv_file, "{}", &column_str)?;

        if self.rows.is_empty() {

            for row in &self.data {

                let mut row_str = row[1..]
                    .iter()
                    .fold(row[0].clone(), |acc, x| format!("{},{}", acc, x));

                row_str.push_str("\n");

                write!(csv_file, "{}", &row_str)?;

            }

        } else {

            for row in &self.rows {

                let mut row_str = row[1..]
                    .iter()
                    .fold(row[0].clone(), |acc, x| format!("{},{}", acc, x));

                row_str.push_str("\n");

                write!(csv_file, "{}", &row_str)?;

            }

        }

        Ok(())

    }
    
    pub fn update_csv(&self, csv_path: &str) -> Result<(), Box<dyn Error>> {

        if Path::new(csv_path).is_file() {

            let mut csv_file = OpenOptions::new().append(true).create(true).open(csv_path)?;

            if self.rows.is_empty() {

                for row in &self.data {
    
                    let mut row_str = row[1..]
                        .iter()
                        .fold(row[0].clone(), |acc, x| format!("{},{}", acc, x));
    
                    row_str.push_str("\n");
    
                    write!(csv_file, "{}", &row_str)?;
    
                }
    
            } else {
    
                for row in &self.rows {
    
                    let mut row_str = row[1..]
                        .iter()
                        .fold(row[0].clone(), |acc, x| format!("{},{}", acc, x));
    
                    row_str.push_str("\n");
    
                    write!(csv_file, "{}", &row_str)?;
    
                }
    
            }
    
            Ok(())

        } else {

            Err("No file!")?

        }

    }

    pub fn sort(&mut self, column: &str, order: bool) -> Result<(), Box<dyn Error>> {

        match self.columns.iter().position(|x| x == column) {
            
            Some(i) => {

                if self.rows.is_empty() {

                    match self.types[i] {
                        MezaType::String => self.data.sort_by(|a, b| a[i].cmp(&b[i])),
                        MezaType::Integer => self.data.sort_by(|a, b| i128::from_str_radix(&a[i], 10).unwrap().cmp(&i128::from_str_radix(&b[i], 10).unwrap())),
                        MezaType::Floating => self.data.sort_by(|a, b| f64::from_str(&a[i]).unwrap().total_cmp(&f64::from_str(&b[i]).unwrap()))
                    }

                    if !order {
                        self.data.reverse();
                    }

                } else {

                    match self.types[i] {
                        MezaType::String => self.rows.sort_by(|a, b| a[i].cmp(&b[i])),
                        MezaType::Integer => self.rows.sort_by(|a, b| i128::from_str_radix(&a[i], 10).unwrap().cmp(&i128::from_str_radix(&b[i], 10).unwrap())),
                        MezaType::Floating => self.rows.sort_by(|a, b| f64::from_str(&a[i]).unwrap().total_cmp(&f64::from_str(&b[i]).unwrap()))
                    }

                    if !order {
                        self.rows.reverse();
                    }

                }

                Ok(())

            },

            None => Err("No column!")?
        
        }

    }

    pub fn filter<F: Fn(&Vec<String>) -> bool>(&self, criteria: F) -> Meza {
        
        let mut result = Meza {
            columns: self.columns.clone(),
            rows: vec![],
            types: self.types.clone(),
            data: vec![]
        };

        if self.rows.is_empty() {

            for row in &self.data {

                if criteria(row) {

                    result.rows.push(row)

                }

            }

        } else {
                
            for row in &self.rows {

                if criteria(row) {

                    result.rows.push(row)

                }

            }

        }

        result

    }

    pub fn is_empty(&self) -> bool {

        self.rows.is_empty() && self.data.is_empty()

    }

    pub fn new<'a>(columns: &'a [&'a str]) -> Meza<'a> {

        Meza {
            columns: columns.iter().map(|x| x.to_string()).collect(),
            rows: vec![],
            types: vec![MezaType::String; columns.len()],
            data: vec![]
        }

    }

    pub fn column(&self, name: &str) -> Result<Vec<&String>, Box<dyn Error>> {

        match self.columns.iter().position(|x| x == name) {

            Some(i) => {

                let column = if self.rows.is_empty() {

                    self.data.iter().map(|x| &x[i]).collect()

                } else {

                    self.rows.iter().map(|x| &x[i]).collect()

                };

                Ok(column)

            },

            None => Err("No column!")?

        }

    }

    pub fn head(&self, count: usize) -> Meza {

        let mut meza = Meza {
            columns: self.columns.clone(),
            data: vec![],
            rows: vec![],
            types: self.types.clone()
        };

        if self.rows.is_empty() {
        
            if self.rows.len() >= count {

                meza.rows = self.data[..count].iter().map(|x| x).collect()

            } else {

                meza.rows = self.data.iter().map(|x| x).collect()

            }

        } else {

            if self.rows.len() >= count {

                meza.rows = self.rows[..count].iter().map(|&x| x).collect()

            } else {

                meza.rows = self.rows.iter().map(|&x| x).collect()

            }

        }

        meza

    }

    pub fn remove_duplicates(&mut self) {

        if self.rows.is_empty() {

            self.data.sort();

            self.data.dedup();

        } else {

            self.rows.sort();

            self.rows.dedup();

        }

    }

    pub fn append(&mut self, row: Vec<String>) -> Result<(), Box<dyn Error>> {

        if self.is_empty() {

            self.data.push(row);

            Ok(())

        } else if !self.data.is_empty() {

            self.data.push(row);

            Ok(())

        } else {
            
            Err("No Data!")?

        }

    }
    
}
