pub mod read_file {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub fn read_file(filename: &str) -> io::Result<String> {
        let file = File::open(filename).unwrap();

        if filename.ends_with(".hks") {
            let reader = BufReader::new(file);
            let mut input = String::new();

            for line in reader.lines() {
                input.push_str(&line.unwrap());
                input.push('\n');
            }
            Ok(input)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid file extension",
            ))
        }
    }
}
