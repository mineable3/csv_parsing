# CSV Parsing

The CSV file type stands for "Comma Separated Values". This is an easy way to store table or matrix like data.
Each comma indicates the next column and each new line indicates the next row. Many modern day spreadsheet
software, such as Excel and Google Sheets, take advantage of this file type and are able to import and export
CSV files. The purpose of this project is to create a fast way to read the contents of the file and store them
in a Vec type for a programmer to use later.

# Code examples

## Setup

To use the most efficient method of reading data from a file, convert_file(), you must edit the
convert_file_recursive() method to match your data. Starting on line 58 of csv_reader.rs is how data is collected
from the file. Change the data types and the amount of each type, to match your needs/dataset.

## Code

```
use crate::csv_reader::csv_reader::CsvReader;

pub mod csv_reader;

fn main() {
    let mut reader = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/unfiltered_data.csv");
    let rowsize = 5;

    let data = reader.convert_file();

    // Retrieving data

    // row, column = (0,0)
    let _ = data.[0 * rowsize + 0];

    // row, column = (1,2)
    let _ = data.[1 * rowsize + 2];

    // Lets actually use some of our data
    // row, column = (1000,4)
    let some_value = data.[1000 * rowsize + 4];

    match some_value {
        DataTypes::INT(x) => println!("{x}"),
        DataTypes::FLOAT(x) => println!("{x}"),
        DataTypes::STRING(x) => println!("{x}"),
    }

    // Benchmark how fast the reader can parse one line of data
    let iterations = 1_000_000;
    let time = reader.benchmark_convert(iterations);
    println!("Benchmark data: {:?} nanoseconds per line read", time);
}

```
