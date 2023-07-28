# Meza

Meza is an in-memory data table written in Rust.

## Author

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Features

- Data Tables
- Sort, Filter, Slice & Deduplicate Tables
- Import & Export for CSV
- Column Average & Variance

## Usage

### new
`new: columns(Vec<&str>) -> Meza`

```
let table = meza::Meza::new(vec!["team","goal"]);
```
### from_csv
`from_csv: &str -> Result<Meza<'a>, Box<dyn Error>>`
### to_csv
`to_csv:&self, &str -> Result<(), Box<dyn Error>>`
### sort
`sort: &mut self, column(&str), order(bool) -> Result<(), Box<dyn Error>>`
### filter
`filter: &self, criteria(Fn(&Vec<Cell>) -> bool)) -> Meza`
### slice
`slice: &self, start(usize), end(usize) -> Meza`
### dedup
`dedup: &mut self`
### average
`average: &mut self, column(&str) -> Result<f32, Box<dyn Error>>`
### variance
`variance: &mut self, column(&str) -> Result<f32, Box<dyn Error>>`

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
