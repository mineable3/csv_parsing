use crate::csv::csv::CsvReader;

pub mod csv;

fn main() {
    let mut reader1 = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/filtered_data.csv");
    let mut reader2 = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/unfiltered_data.csv");
    // let mut reader = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/data.csv");

    let iterations = 10_000_000;
    println!("Benchmark filtered data: {:?} nanoseconds per line read", reader1.benchmark_filtered(iterations));
    println!("Benchmark real data: {:?} nanoseconds per line read", reader2.benchmark_real(iterations));
}

