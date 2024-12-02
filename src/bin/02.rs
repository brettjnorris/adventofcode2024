advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = parse_input(input);

    let passing_reports_count = lines.into_iter().filter(|line| grade_report(line)).count() as u32;

    Some(passing_reports_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = parse_input(input);

    let passing_reports_count = lines.into_iter().filter(|line| {
        let permutations = generate_permutations(line);
        permutations.iter().any(|line| grade_report(line))
    }).count() as u32;

    Some(passing_reports_count)
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|l| l.split_whitespace().map(|elem| elem.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>()
}

fn generate_permutations(original: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut i = 0;
    let mut permutations: Vec<Vec<u32>>= vec![];
    permutations.push(original.clone());

    while i < original.len() {
        permutations.push(omit_vec_elem(original, i));
        i += 1;
    }

    permutations
}

fn omit_vec_elem(line: &Vec<u32>, index: usize) -> Vec<u32> {
    line.into_iter().enumerate().filter(|(i, _elem)| index.abs_diff(*i) != 0).map(|(_, e)| e.clone()).collect::<Vec<u32>>()
}

fn grade_report(report: &Vec<u32>) -> bool {
    let mut direction: isize = 0;

    report.windows(2).all(|elem| {
        let x = elem[0] as usize;
        let y = elem[1] as usize;

        let this_direction: isize = if x > y {
            -1
        } else {
             1
        };

        if direction == 0 {
            direction = this_direction
        } else {
            if direction != this_direction { return false }
        }

        let diff = (x).abs_diff(y);
        if diff > 3 || diff < 1 { return false }

        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
