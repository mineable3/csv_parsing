use crate::csv::csv::CsvReader;

pub mod csv;

fn main() {
    let mut reader = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/data.csv");

    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
    println!("{:?}", reader.benchmark());
}


