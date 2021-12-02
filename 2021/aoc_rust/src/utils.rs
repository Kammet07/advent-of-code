pub mod utils {
    use std::fs;

    pub fn read_file(path: &str) -> Vec<String> {
        let file = fs::read_to_string(path)
            .expect("file reading bad :(");

        let file_vector: Vec<&str> = file.lines().collect();
        let mut result: Vec<String> = Vec::new();

        for string in file_vector {
            result.push(string.to_string());
        }

        return result;
    }
}