use crate::Day;
use regex::Regex;

pub struct Day3 {}

enum Instruction {
    Do,
    Dont,
    Mul(i32, i32),
}

impl Day for Day3 {
    fn part1(&self, lines: &Vec<&str>) -> i64 {
        run(&parse(lines), false)
    }

    fn part2(&self, lines: &Vec<&str>) -> i64 {
        run(&parse(lines), true)
    }
}

fn run(instructions: &[Instruction], dos_enabled: bool) -> i64 {
    let mut mul_enabled = true;
    let mut result = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Do => mul_enabled = true,
            Instruction::Dont => mul_enabled = false,
            Instruction::Mul(x, y) => {
                if !dos_enabled || mul_enabled {
                    result += (x * y) as i64;
                }
            }
        }
    }

    result
}

fn parse(lines: &Vec<&str>) -> Vec<Instruction> {
    let pattern = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    lines
        .iter()
        .flat_map(|line| {
            pattern.captures_iter(line).map(|caps| {
                let matched = &caps[0];

                if matched.starts_with("do(") {
                    Instruction::Do
                } else if matched.starts_with("don't(") {
                    Instruction::Dont
                } else {
                    Instruction::Mul(caps[1].parse().unwrap(), caps[2].parse().unwrap())
                }
            })
        })
        .collect()
}
