use regex::Regex;

advent_of_code::solution!(17);

struct Cpu {
    register_a: isize,
    register_b: isize,
    register_c: isize,
    program_counter: usize,
    program: Vec<isize>,
}

impl Cpu {
    fn from_input(input: &str) -> Cpu {
        let lines = input.lines().collect::<Vec<&str>>();
        let register_a = Self::parse_number(lines[0]);
        let register_b = Self::parse_number(lines[1]);
        let register_c = Self::parse_number(lines[2]);

        let program = Self::parse_program(input);

        Cpu {
            register_a,
            register_b,
            register_c,
            program_counter: 0,
            program
        }
    }

    fn parse_number(input: &str) -> isize {
        let re = Regex::new(r"(\d+)").unwrap();
        let captures = re.captures(input).unwrap();
        captures.get(1).unwrap().as_str().parse::<isize>().unwrap()
    }

    fn parse_program(input: &str) -> Vec<isize> {
        let mut numbers: Vec<isize> = vec![];
        let re = Regex::new(r"Program:\s*([\d,]+)").unwrap();

        // Find the "Program" line and extract the numbers
        if let Some(captures) = re.captures(input) {
            if let Some(program_numbers) = captures.get(1) {
                // Split the captured numbers into a vector
                numbers = program_numbers
                    .as_str()
                    .split(',')
                    .filter_map(|num| num.parse::<isize>().ok())
                    .collect();

                println!("Program numbers: {:?}", numbers);
            }
        } else {
            println!("Program line not found");
        }

        numbers
    }

    fn execute(&mut self) -> String {
        let mut i = 0;
        let mut outputs: Vec<isize> = Vec::new();
        while self.program_counter < self.program.len() {
            if i > 10000 {
                break;
            }
            if let Some(val) = self.step() {
                outputs.push(val);
            }
            i += 1;
        }

        outputs.into_iter().map(|output| output.to_string()).collect::<Vec<String>>().join(",")
    }

    fn advance(&mut self) -> isize {
        if self.program_counter < self.program.len() {
            let data = self.program[self.program_counter];
            self.program_counter += 1;

            data
        } else {
            0
        }
    }

    fn step(&mut self) -> Option<isize> {
        // println!("register_a: {:?}, register_b: {:?}, register_c: {:?}, program_counter: {:?}", self.register_a, self.register_b, self.register_c, self.program_counter);
        let instruction = self.advance();
        println!("instruction: {:?}", instruction);

        match instruction {
            0 => { self.adv() },
            1 => { self.bxl() },
            2 => { self.bst() },
            3 => { self.jnz() },
            4 => { self.bxc() },
            5 => { self.out() },
            6 => { self.bdv() },
            7 => { self.cdv() },
            _ => { None }
        }
    }

    fn adv(&mut self) -> Option<isize> {
        let operand = self.advance();
        self.register_a = self.division(operand).unwrap();
        None
    }

    fn bxl(&mut self) -> Option<isize> {
        let operand = self.advance();
        let result = self.register_b ^ operand;
        println!("bxl: {:?} {:?} {:?}", self.register_b, operand, result);
        self.register_b = result;
        None
    }

    fn bst(&mut self) -> Option<isize> {
        let operand = self.advance();
        let combo = self.combo_operand(operand);
        let result = combo % 8;
        self.register_b = result;
        None
    }

    fn jnz(&mut self) -> Option<isize> {
        if (self.register_a != 0) {
            let operand = self.advance();
            println!("jnz: {:?} {:?}", self.register_a, operand);
            self.program_counter = operand as usize;
        }

        None
    }

    fn bxc(&mut self) -> Option<isize> {
        let operand = self.advance();
        self.register_b = self.register_b ^ self.register_c;
        None
    }

    fn out(&mut self) -> Option<isize> {
        let operand = self.advance();
        let combo = self.combo_operand(operand);
        let result = combo % 8;
        println!("out: {:?} {:?} {:?}", operand, combo, result);
        Some(result)
    }

    fn bdv(&mut self) -> Option<isize> {
        let operand = self.advance();
        self.register_b = self.division(operand).unwrap();
        None
    }

    fn cdv(&mut self) -> Option<isize> {
        let operand = self.advance();
        self.register_c = self.division(operand).unwrap();
        None
    }

    fn division(&mut self, operand: isize) -> Option<isize> {
        let numerator = self.register_a;
        let combo = self.combo_operand(operand);
        let denominator = 2_isize.pow(combo as u32);
        let result = numerator / denominator;

        println!("numer: {:?}, denom: {:?}, combo: {:?}, result: {:?}", numerator, denominator, combo, result);

        Some(result)
    }

    fn combo_operand(&self, operand: isize) -> isize {
        match operand {
            0 => { 0 },
            1 => { 1 },
            2 => { 2 },
            3 => { 3 },
            4 => { self.register_a.clone() },
            5 => { self.register_b.clone() },
            6 => { self.register_c.clone() },
            _ => { 0 }
        }
    }
}



enum Instruction { Adv, Bxl, Bst, Jnz, Bxc, Out, Bdv, Cdv }

pub fn part_one(input: &str) -> Option<String> {
    let mut cpu = Cpu::from_input(input);
    Some(cpu.execute())
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
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_case_one() {
        let input = "Register A: 0\nRegister B: 0\nRegister C: 9\nProgram: 2,6";
        let mut cpu = Cpu::from_input(input);
        assert_eq!(cpu.register_b, 0);
        cpu.execute();
        assert_eq!(cpu.register_b, 1);
    }

    #[test]
    fn test_case_two() {
        let input = "Register A: 10\nRegister B: 0\nRegister C: 0\nProgram: 5,0,5,1,5,4";
        let mut cpu = Cpu::from_input(input);
        let ouptut = cpu.execute();
        assert_eq!(ouptut, "0,1,2");
    }

    #[test]
    fn test_case_three() {
        let input = "Register A: 2024\nRegister B: 0\nRegister C: 0\nProgram: 0,1,5,4,3,0";
        let mut cpu = Cpu::from_input(input);
        let output = cpu.execute();
        assert_eq!(cpu.register_a, 0);
        assert_eq!(output, "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn test_case_four() {
        let input = "Register A: 0\nRegister B: 29\nRegister C: 0\nProgram: 1,7";
        let mut cpu = Cpu::from_input(input);
        let output = cpu.execute();
        assert_eq!(cpu.register_b, 26);
    }

    #[test]
    fn test_case_five() {
        let input = "Register A: 0\nRegister B: 2024\nRegister C: 43690\nProgram: 4,0";
        let mut cpu = Cpu::from_input(input);
        let output = cpu.execute();
        assert_eq!(cpu.register_b, 44354);
    }
}
