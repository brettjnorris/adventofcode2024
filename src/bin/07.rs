advent_of_code::solution!(7);

enum Calculation { None, Add, Multiply }
enum Item { Number(usize), Calculation(Calculation) }
#[derive(Debug)]
struct Equation {
    pub value: usize,
    parsed_equation: String
}

impl Equation {
    fn from_string(s: &str) -> Self {
        let mut value = 0;
        let mut operators: Vec<usize> = vec![];
        let mut parsed_equation = String::new();

        let parts = s.split(": ").collect::<Vec<&str>>();
        if (parts.len() == 2) {
            value = parts[0].parse().unwrap();
            let foo = parts[1].split_whitespace().collect::<Vec<&str>>();

            parsed_equation = foo.join(" _ ");
        }

        Self { value, parsed_equation}
    }

    fn evaluate_left_to_right(expr: &str) -> i64 {
        let mut tokens = expr.split_whitespace();
        let mut result = tokens.next().unwrap().parse::<i64>().unwrap();

        while let Some(op) = tokens.next() {
            let num = tokens.next().unwrap().parse::<i64>().unwrap();
            result = match op {
                "+" => result + num,
                "*" => result * num,
                _ => panic!("Unsupported operator: {}", op),
            };
        }

        result
    }

    fn generate_permutations(s: &str) -> Vec<String> {
        let placeholders = s.matches('_').count(); // Count the number of `_` in the string
        let total_combinations = 1 << placeholders; // 2^placeholders combinations
        let mut results = Vec::new();

        for i in 0..total_combinations {
            let mut chars = s.chars();
            let mut combination = String::new();
            let mut bit_index = 0;

            while let Some(c) = chars.next() {
                if c == '_' {
                    // Replace `_` with `+` or `*` based on the current bit in `i`
                    if (i & (1 << bit_index)) != 0 {
                        combination.push('+');
                    } else {
                        combination.push('*');
                    }
                    bit_index += 1;
                } else {
                    combination.push(c);
                }
            }

            results.push(combination);
        }

        results
    }

    fn is_valid(&self) -> bool {
        let permutations = Self::generate_permutations(self.parsed_equation.as_str());
        permutations.iter().any(|permutation| {
            let calc_result = Equation::evaluate_left_to_right(&permutation);
             calc_result == self.value as i64


        })
    }
}



pub fn part_one(input: &str) -> Option<u64> {
    let equations: Vec<Equation> = input.lines().map(|x| Equation::from_string(x)).collect();

    let sum: usize = equations.iter().filter_map(|equation| {
        match equation.to_owned().is_valid() {
            true => Some(equation.value as usize),
            false => None
        }
    }).sum();

    Some(sum as u64)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_eval() {
        let result = Equation::evaluate_left_to_right("11 + 6 * 16 + 20");
        assert_eq!(result, 292);
    }
}
