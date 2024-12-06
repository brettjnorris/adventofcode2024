advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input).unwrap();
    let valid_pages = pages.iter().filter_map(|page| {
        let valid = rules.iter().all(|rule| {
            validate_page_order(page.to_vec(), rule.to_vec())
        });

        if (valid) {
            Some(page)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    let result = valid_pages.iter().map(|page| middle_page_number(*page).unwrap_or(0) as u32).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input).unwrap();
    let invalid_pages = get_pages(pages, rules, false);

    None
}

fn parse_input(input: &str) -> Option<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    if sections.len() == 2 {
        let rules = sections[0].split('\n').map(|elem| {
            elem.split('|').map(|elem| {
                elem.parse::<usize>().unwrap()
            } ).collect()
        }).collect::<Vec<_>>();

        let pages = sections[1].split('\n').map(|elem| {
            elem.split(',').map(|elem| {
                elem.parse::<usize>().unwrap_or(0)
            } ).collect()
        }).collect::<Vec<_>>();

        Some((rules, pages))
    } else {
        None
    }
}

fn get_pages(pages: Vec<Vec<usize>>, rules: Vec<Vec<usize>>, expect_valid: bool) -> Vec<Vec<usize>> {
    pages.iter().filter_map(|page| {
        let valid = rules.iter().all(|rule| {
            validate_page_order(page.to_vec(), rule.to_vec())
        });

        if (valid == expect_valid) {
            Some(page.clone())
        } else {
            None
        }
    }).collect::<Vec<Vec<usize>>>()
}

fn validate_page_order(page_numbers: Vec<usize>, rule_numbers: Vec<usize>) -> bool {
    let positions = rule_numbers.iter().map(|rule_number| {
        page_numbers.iter().position(|&elem| elem == *rule_number)
    }).collect::<Vec<Option<usize>>>();

     match (positions[0], positions[1]) {
        (Some(x), Some(y)) => x < y,
        _ => true
    }
}

fn middle_page_number(page_numbers: &[usize]) -> Option<usize> {
    let index = page_numbers.len() / 2;
    Some(page_numbers[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
