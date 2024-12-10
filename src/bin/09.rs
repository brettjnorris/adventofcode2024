use std::str::FromStr;

advent_of_code::solution!(9);

#[derive(Debug)]
enum BlockType {
    Free,
    Occupied
}
pub fn part_one(input: &str) -> Option<u64> {
    let expanded = expand_input(input.chars().map(|c| c.to_string()).collect());
    let compacted = compact_fs(expanded);
    let checksum = checksum(&compacted);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<String> {
    let chars: Vec<String> = input.chars().map(|c| c.to_string() ).collect();
    let expanded_input = expand_input(chars);
    let compacted = compact_fs(expanded_input);

    compacted
}

fn expand_input(chars: Vec<String>) -> Vec<String> {
    let mut expanded = vec![];
    let mut current_index: u32 = 0;
    let mut current_state = BlockType::Occupied;

    for x in chars {
        match u32::from_str(x.as_str()) {
            Ok(count) => {
                let input_char: String = match current_state {
                    BlockType::Occupied => {
                        current_state = BlockType::Free;
                        format!("{current_index}")
                    },
                    BlockType::Free => {
                        current_state = BlockType::Occupied;
                        String::from(".") as String
                    }
                };

                for _ in 0..count {
                    expanded.push(input_char.clone());
                }

                if input_char != String::from(".") {
                    current_index += 1;
                }
            },
            _ => { break;}
        }


    }

    expanded
}

fn compact_fs(input: Vec<String>) -> Vec<String> {
    let mut compacted: Vec<String> = input.clone();

    while !is_compacted(&compacted) {
        let last = compacted.pop().unwrap();

        if last == String::from(".") {
            continue
        }

        let position = compacted.iter().position(|x| x == &String::from(".")).unwrap();
        compacted[position] = last;
    }

    compacted
}

fn is_compacted(input: &Vec<String>) -> bool {
    let is_compacted = !input.contains(&String::from("."));
    is_compacted
}

fn checksum(input: &Vec<String>) -> u64 {
    input.iter().enumerate().filter_map(|(i, x)| {
        match u64::from_str(x) {
            Ok(digit) => Some(digit as u64 * i as u64),
            _ => None
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_expand_input() {
        let mut chars = "2333133121414131402".chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let result = expand_input(chars);

        let expected = "00...111...2...333.44.5555.6666.777.888899".chars().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compact_fs() {
        let expanded = "00...111...2...333.44.5555.6666.777.888899".chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let result = compact_fs(expanded);

        let expected = "0099811188827773336446555566".chars().map(|c| c.to_string()).collect::<Vec<String>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_checksum() {
        let compacted = "0099811188827773336446555566".chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let result = checksum(&compacted);

        assert_eq!(result, 1928);
    }
}
