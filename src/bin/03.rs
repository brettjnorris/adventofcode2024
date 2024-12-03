advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let sum = re.find_iter(input).map(|m| {
        handle_mut_instruction(m.as_str()).unwrap_or(0)
    }).sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mut mut_enabled = true;
    let sum = re.find_iter(input).map(|m| {
        let instruction = m.as_str();
        match instruction {
            "do()" => {
                mut_enabled = true;
                None
            },
            "don't()" => {
                mut_enabled = false;
                None
            },
            _ => {
                if mut_enabled {
                    handle_mut_instruction(instruction)
                } else {
                    None
                }
            }
        }.unwrap_or(0)
    }).sum::<u32>();

    Some(sum)
}

fn handle_mut_instruction(instruction: &str) -> Option<u32> {
    match parse_arguments_from_match(instruction) {
        Some((x, y)) => Some(x * y),
        None => None,
    }
}

fn parse_arguments_from_match(instruction: &str) -> Option<(u32, u32)> {
    let re = Regex::new(r"(\d+)").unwrap();
    let captures: Vec<u32> = re.find_iter(instruction).map(|m| m.as_str().parse::<u32>().unwrap()).collect();

    if captures.len() == 2 {
        Some((captures[0], captures[1]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
