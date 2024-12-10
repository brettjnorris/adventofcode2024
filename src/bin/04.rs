advent_of_code::solution!(4);

use regex::Regex;
use advent_of_code::matrix;
use matrix::Matrix;

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Matrix::from_string(input);
    let match_count = puzzle.get_row_strings().iter().map(|row| {
        let mut count = 0;
        let re = Regex::new(r"XMAS").unwrap();
        count += re.find_iter(row).count();

        let reversed: String = row.chars().rev().collect();
        count += re.find_iter(&reversed).count();

        count as usize
    }).sum::<usize>();

    Some(match_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
