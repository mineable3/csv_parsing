pub mod csv {
    use std::fs;
    use std::time::Instant;

    pub struct CsvReader {
        /// Keeps track of the current line the CsvReader is on
        current_line: u32,
        /// Contains the entire contents of the file
        raw_string: String,
    }

    impl CsvReader {

        /// Creates a new CsvReader Struct
        pub fn new(file_path: &str) -> Self {
            let data = fs::read_to_string(file_path).unwrap_or(String::from("The file was blank"));

            return CsvReader {
                current_line: 0,
                raw_string: data,
            }
        }
        
        pub fn get_next_line(&mut self) -> Vec<i32> {
            let mut iter = self.raw_string.lines();

            for _ in 0..self.current_line {
                iter.next();
            }
            
            // Gets the current line of the reader
            let target = iter.next().unwrap_or("");

            // Takes that line and turns each , into a newline
            let filtered_target = target.replace(",", "\n");

            // Creates an iterator for the target line
            let target_iter = filtered_target.lines();

            let mut output = Vec::<i32>::new();

            for num in target_iter {
                output.push(num.parse::<i32>().unwrap());
            }

            self.current_line += 1;

            return output;
        }

        /// Finds the amount of time it takes for a function to run
        pub fn benchmark(&mut self) -> u128 {
            let now = Instant::now();

            self.get_next_line();

            return now.elapsed().as_nanos();
        }
    }
}
