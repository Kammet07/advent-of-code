pub mod day_01 {
    use crate::utils::utils::read_file;

    fn parse_input() -> Vec<u16> {
        let file = read_file("assets/day_01.txt");

        let mut result: Vec<u16> = Vec::new();

        for string in file {
            result.push(string.parse().expect("error parsing"));
        }

        return result;
    }

    pub fn solve_part_one() -> u16 {
        let input = parse_input();

        let mut result = 0;

        for number in 1..input.len() {
            if input[number - 1] < input[number] {
                result = result + 1;
            }
        }

        return result;
    }


    pub fn solve_part_two() -> u16 {
        let input = parse_input();

        let mut result = 0;

        for number in 0..(input.len() - 3) {
            if (input[number] + input[number + 1] + input[number + 2]) < (input[number + 1] + input[number + 2] + input[number + 3]) {
                result = result + 1;
            }
        }

        return result;
    }
}