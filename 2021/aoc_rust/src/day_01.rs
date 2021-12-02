pub mod day_01 {
    use crate::utils::utils::read_file;


    pub fn solve_part_one() -> i32 {
        let input = read_file("assets/day_01.txt");

        let mut result = 0;

        for number in 1..input.len() {
            if input[number - 1] < input[number] {
                result = result + 1;
            }
        }

        return result;
    }


    pub fn solve_part_two() -> i32 {
        let input = read_file("assets/day_01.txt");

        let mut result = 0;

        for number in 0..(input.len() - 3) {
            if (input[number] + input[number + 1] + input[number + 2]) < (input[number + 1] + input[number + 2] + input[number + 3]) {
                result = result + 1;
            }
        }

        return result;
    }
}