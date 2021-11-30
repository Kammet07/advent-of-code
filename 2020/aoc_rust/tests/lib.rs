extern crate aoc_rust;

#[cfg(test)]
mod tests {
    use aoc_rust::day_01::day_01::{solve_part_one, solve_part_two};

    #[test]
    fn test_day_01_part_one() {
        assert_eq!(solve_part_one(), 319531);
    }

    #[test]
    fn test_day_01_part_two() {
        assert_eq!(solve_part_two(), 244300320)
    }
}