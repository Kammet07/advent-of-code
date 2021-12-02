pub mod utils {
    use std::fs;

    pub fn read_file(path: &str) -> Vec<i16> {
        let file = fs::read_to_string(path)
            .expect("file reading bad :(");

        let mut result: Vec<i16> = Vec::new();
        let file_vector: Vec<&str> = file.split("\n").collect();

        for string in file_vector {
            result.push(string.parse().expect("error parsing"));
        }

        return result;
    }
}