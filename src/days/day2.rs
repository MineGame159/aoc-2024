use crate::Day;

pub struct Day2 {}

impl Day for Day2 {
    fn part1(&self, lines: &[&str]) -> i64 {
        parse(lines).iter().filter(|report| analyze_report(report)).count() as i64
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        parse(lines)
            .iter()
            .filter(|report| {
                if analyze_report(report) {
                    return true;
                }

                for i in 0..report.len() {
                    let mut new_report = (*report).clone();
                    new_report.remove(i);

                    if analyze_report(&new_report) {
                        return true;
                    }
                }

                false
            })
            .count() as i64
    }
}

fn analyze_report(report: &Vec<i32>) -> bool {
    let mut safe = 0;
    let mut idk: i32 = 0;

    for i in 1..report.len() {
        if report[i] == report[i - 1] {
            continue;
        }

        if report[i] > report[i - 1] {
            idk += 1;
        } else {
            idk -= 1;
        }

        let diff = (report[i] - report[i - 1]).abs();

        if diff < 1 || diff > 3 {
            continue;
        }

        safe += 1;
    }

    let size = report.len() as i32 - 1;
    safe >= size && idk.abs() == size
}

fn parse(lines: &[&str]) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| line.split(' ').map(|level| level.parse().unwrap()).collect::<Vec<i32>>())
        .collect()
}
