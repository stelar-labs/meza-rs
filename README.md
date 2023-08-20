# Meza

Meza is an in-memory data table written in Rust.

## Author

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Features

- Data Tables
- Column Slice Tables
- Import & Export for CSV
- Column Sum, Average & Variance
- Row & Cell Search
- Meza to `Matrix<Fraction>`

## Usage

### new
```
pub fn new(columns: Vec<&str>) -> Meza<'a>
```

### csv
```
pub fn from_csv(csv_path: &str) -> Result<Meza<'a>, Box<dyn Error>>
pub fn to_csv(&self, csv_path: &str) -> Result<(), Box<dyn Error>>
```

### filter
```
pub fn filter<F>(&'a self, criteria: F) -> Meza<'a> where F: Fn(&Row<'a>) -> bool
```

### search
```
pub fn row_search(&self, key_column: &str, key: Cell) -> Result<&Row<'a>, Box<dyn Error>>
pub fn index_row_search(&self, key_index: usize, key: Cell) -> Result<&Row<'a>, Box<dyn Error>>
pub fn index_cell_search(&self, key_index: usize, value_index: usize, key: Cell) -> Result<&Cell, Box<dyn Error>>
pub fn cell_search(&self, key_column: &str, value_column: &str, key: Cell) -> Result<&Cell, Box<dyn Error>>
```

### slice
```
pub fn row_slice(&'a self, start: usize, end: usize) -> Result<Meza<'a>, Box<dyn Error>>
pub fn column_slice(&'a self, columns: Vec<&str>) -> Result<Meza<'a>, Box<dyn Error>>
```

### sort
```
pub fn sort(&mut self, column: &str, order: bool) -> Result<(), Box<dyn Error>>
pub fn index_sort(&mut self, column_index: usize, order: bool) -> Result<(), Box<dyn Error>>
```

### statistics
```
pub fn calculate_sum(&mut self, column: &str)
pub fn index_calculate_sum(&mut self, column_index: usize)
pub fn calculate_average(&mut self, column: &str)
pub fn index_calculate_average(&mut self, column_index: usize)
pub fn calculate_variance(&mut self, column: &str)
pub fn index_calculate_variance(&mut self, column_index: usize)

```

## License

MIT License

Copyright Stelar Labs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
