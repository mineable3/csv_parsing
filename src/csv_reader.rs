pub mod csv_reader {
    use std::fs;
    use std::time::Instant;
    use std::str::Lines;
    use std::{thread, time::Duration};

    pub struct CsvReader {
        /// Keeps track of the current line the CsvReader is on
        current_line: u32,
        /// Contains the entire contents of the file
        raw_string: String,
    }

    pub enum DataTypes {
        INT(i8),
        STRING(String),
        FLOAT(f64),
    }

    impl CsvReader {

        /// Creates a new CsvReader Struct
        pub fn new(file_path: &str) -> Self {
            let mut data = fs::read_to_string(file_path).unwrap_or(String::from("The file was blank"));

            data.remove(0);
            data = data.trim().to_string();

            return CsvReader {
                current_line: 0,
                raw_string: data,
            }
        }

        pub fn convert_file(&mut self) -> Vec<DataTypes> {
            let full_file = String::from(self.raw_string.clone());
            let iter = full_file.lines();

            let output = Vec::<DataTypes>::new();
            return self.convert_file_recursive(iter, output);
        }

        fn convert_file_recursive(&mut self, mut iter: Lines, mut data: Vec<DataTypes>) -> Vec<DataTypes> {
            // Gets the current line of the reader
            let target = match iter.next() {
                Some(t) => t,
                None => {
                    return data
                },
            };

            // Takes that line and turns each , into a newline
            let filtered_target = target.replace(",", "\n");

            // Creates an iterator for the target line
            let mut target_iter = filtered_target.lines();

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            data.push(DataTypes::INT(trimmed.parse::<i8>().unwrap_or(0)));

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            data.push(DataTypes::STRING(trimmed));

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            data.push(DataTypes::FLOAT(trimmed.parse::<f64>().unwrap_or(0.0)));

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            data.push(DataTypes::FLOAT(trimmed.parse::<f64>().unwrap_or(0.0)));

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            data.push(DataTypes::FLOAT(trimmed.parse::<f64>().unwrap_or(0.0)));

            return self.convert_file_recursive(iter, data);
        }

        pub fn get_real_lines(&mut self, looping: bool) -> Vec<DataTypes> {
            let mut iter = self.raw_string.lines();

            for _ in 0..self.current_line {
                iter.next();
            }
            
            // Gets the current line of the reader
            let target = match iter.next() {
                Some(t) => t,
                None => {
                    self.current_line = 0;
                    return self.get_real_lines(looping);
                },
            };

            // Takes that line and turns each , into a newline
            let filtered_target = target.replace(",", "\n");

            // Creates an iterator for the target line
            let mut target_iter = filtered_target.lines();

            let mut output = Vec::<DataTypes>::new();

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            output.push(DataTypes::INT(trimmed.parse::<i8>().unwrap_or(0)));

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            output.push(DataTypes::STRING(trimmed));

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            output.push(DataTypes::FLOAT(trimmed.parse::<f64>().unwrap_or(0.0)));

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            output.push(DataTypes::FLOAT(trimmed.parse::<f64>().unwrap_or(0.0)));

            let trimmed = String::from(target_iter.next().unwrap_or_default().trim());
            output.push(DataTypes::FLOAT(trimmed.parse::<f64>().unwrap_or(0.0)));

            self.current_line += 1;

            return output;
        }

        pub fn get_next_line(&mut self, looping: bool) -> Vec<f64> {
            let mut iter = self.raw_string.lines();

            for _ in 0..self.current_line {
                iter.next();
            }
            
            // Gets the current line of the reader
            let target = match iter.next() {
                Some(t) => t,
                None => {
                    self.current_line = 0;
                    return self.get_next_line(looping);
                },
            };

            // Takes that line and turns each , into a newline
            let filtered_target = target.replace(",", "\n");

            // Creates an iterator for the target line
            let target_iter = filtered_target.lines();

            let mut output = Vec::<f64>::new();

            for num in target_iter {
                let trimmed = String::from(num.trim());
                output.push(trimmed.parse::<f64>().unwrap());
            }

            self.current_line += 1;

            return output;
        }

        /// Finds the amount of time it takes for a function to run
        /// Currently at 1920 nanoseconds per line read
        pub fn benchmark_real(&mut self, iterations: usize) -> u128 {
            let mut sum = 0;

            for _ in 0..iterations {
                thread::sleep(Duration::from_millis(20));
                let now = Instant::now();

                let data = self.get_real_lines(true);
                
                for i in 0..data.len() {
                    match &data[i] {
                        DataTypes::INT(x) => println!("{x}"),
                        DataTypes::FLOAT(x) => println!("{x}"),
                        DataTypes::STRING(x) => println!("{x}"),
                    }
                }

                sum += now.elapsed().as_nanos() / 5;
            }

            return sum / iterations as u128; 
        }

        /// Finds the amount of time it takes for a function to run
        /// Currently at 1916 nanoseconds per line read
        pub fn benchmark_filtered(&mut self, iterations: usize) -> u128 {
            let mut sum = 0;

            for _ in 0..iterations {
                thread::sleep(Duration::from_millis(20));
                let now = Instant::now();

                let data = self.get_next_line(true);
                println!("{:?}", data[0]);
                println!("{:?}", data[1]);
                println!("{:?}", data[2]);
                println!("{:?}", data[3]);

                sum += now.elapsed().as_nanos();
            }

            return sum / iterations as u128; 
        }

        pub fn benchmark_convert(&mut self, iterations: usize) -> u128 {
            let mut sum = 0;

            let now = Instant::now();
            let data: Vec<DataTypes> = self.convert_file();
            sum += now.elapsed().as_nanos();

            for i in 0..iterations {
                thread::sleep(Duration::from_millis(20));
                let now = Instant::now();

                // let _: DataTypes = match &data[i % data.len()] {
                for j in 0..5 {
                    match &data[i % data.len() + j] {
                        DataTypes::INT(x) => println!("{x}"),
                        DataTypes::FLOAT(x) => println!("{x}"),
                        DataTypes::STRING(x) => println!("{x}"),
                    };
                }

                sum += now.elapsed().as_nanos();
            }

            return sum / iterations as u128; 
        }
    }
}
