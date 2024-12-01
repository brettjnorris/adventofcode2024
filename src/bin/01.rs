advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut column_a: Vec<u32> = lines.clone().map(|l| l.split_whitespace().next().unwrap().parse().unwrap()).collect();
    let mut column_b: Vec<u32> = lines.clone().map(|l| l.split_whitespace().last().unwrap().parse().unwrap()).collect();

    column_a.sort();
    column_b.sort();

    let result = column_a
        .into_iter()
        .zip(column_b.into_iter())
        .enumerate()
        .map(|(_i, (a, b))| a.abs_diff(b)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let column_a: Vec<u32> = lines.clone().map(|l| l.split_whitespace().next().unwrap().parse().unwrap()).collect();
    let column_b: Vec<u32> = lines.clone().map(|l| l.split_whitespace().last().unwrap().parse().unwrap()).collect();

    let result = column_a.iter().map(|num| {
        let count = find_count(num, &column_b).unwrap();
        Some((count * num) as u32)
    }).sum::<Option<u32>>().unwrap();

    Some(result)
}

fn find_count(num: &u32, list: &Vec<u32>) -> Option<u32> {
    Some(list.iter().filter(|&v| *v == *num).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
