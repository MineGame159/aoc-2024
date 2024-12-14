use crate::Day;
use std::cmp::PartialEq;
use std::sync::{LazyLock, Mutex};

pub struct Day7 {}

impl Day for Day7 {
    fn part1(&self, lines: &[&str]) -> i64 {
        run(lines, false)
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        run(lines, true)
    }
}

fn run(lines: &[&str], combine_enabled: bool) -> i64 {
    let equations = parse(lines);

    let mut sum = 0;

    for equation in equations {
        if equation.solve(combine_enabled).len() != 0 {
            sum += equation.result;
        }
    }

    sum
}

#[derive(Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Combine,
}

struct Equation {
    result: i64,
    values: Vec<i64>,
}

impl Equation {
    fn solve(&self, combine_enabled: bool) -> Vec<Operator> {
        let mut operators = vec![Operator::Add; self.values.len() - 1];

        loop {
            if self.solves(&operators) {
                return operators;
            }

            let mut i = 0;

            while i < operators.len() {
                let mut stop = false;
                let mut inc_i = false;

                operators[i] = match operators[i] {
                    Operator::Add => {
                        stop = true;
                        Operator::Multiply
                    }
                    Operator::Multiply => {
                        if combine_enabled {
                            stop = true;
                            Operator::Combine
                        } else {
                            inc_i = true;
                            Operator::Add
                        }
                    }
                    Operator::Combine => {
                        inc_i = true;
                        Operator::Add
                    }
                };

                if stop {
                    break;
                }
                if inc_i {
                    i += 1;
                }
            }

            if i == operators.len() {
                break;
            }
        }

        operators.clear();
        operators
    }

    fn solves(&self, operators: &[Operator]) -> bool {
        let mut value_it = self.values.iter();
        let mut result = *value_it.next().unwrap();

        static STR: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::with_capacity(16)));
        let mut str = STR.lock().unwrap();

        for operator in operators {
            let value = value_it.next().unwrap();

            match operator {
                Operator::Add => result += value,
                Operator::Multiply => result *= value,
                Operator::Combine => {
                    std::fmt::write(&mut *str, format_args!("{}{}", result, value)).unwrap();
                    result = str.parse().unwrap();
                    str.clear();
                }
            }
        }

        result == self.result
    }
}

fn parse(lines: &[&str]) -> Vec<Equation> {
    let mut equations = Vec::with_capacity(lines.len());

    for line in lines {
        let mut it = line.split(':');

        equations.push(Equation {
            result: it.next().unwrap().parse().unwrap(),
            values: it
                .next()
                .unwrap()
                .split(' ')
                .filter(|value| !value.is_empty())
                .map(|value| value.parse().unwrap())
                .collect(),
        });
    }

    equations
}
