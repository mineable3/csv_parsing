use crate::csv_reader::csv_reader::CsvReader;

pub mod csv_reader;

fn main() {
    let mut reader1 = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/filtered_data.csv");
    let mut reader2 = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/unfiltered_data.csv");
    let mut reader3 = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/unfiltered_data.csv");
    // let mut reader = CsvReader::new("C:/Users/emmet/Documents/Projects/Rust/csv-reader/data.csv");

    let iterations = 1_00_000;
    let filtered = reader1.benchmark_filtered(iterations);
    let real = reader2.benchmark_real(iterations);
    let convert = reader3.benchmark_convert(iterations);

    println!("Benchmark filtered data: {:?} nanoseconds per line read", filtered);
    println!("Benchmark real data: {:?} nanoseconds per line read", real);
    println!("Benchmark converted data: {:?} nanoseconds per line read", convert);
}

