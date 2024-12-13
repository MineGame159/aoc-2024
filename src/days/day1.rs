use crate::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn part1(&self, lines: &Vec<&str>) -> i64 {
        let first = get_list(lines, 0);
        let second = get_list(lines, 1);

        let mut sum = 0;

        for i in 0..lines.len() {
            sum += i64::from((first[i] - second[i]).abs())
        }

        sum
    }

    fn part2(&self, lines: &Vec<&str>) -> i64 {
        let first = get_list(lines, 0);
        let second = get_list(lines, 1);

        let mut sum = 0;

        for number in first {
            let mut count = 0;

            for other in &second {
                if *other == number {
                    count += 1;
                } else if *other > number {
                    break;
                }
            }

            sum += i64::from(number * count);
        }

        sum
    }
}

fn get_list(lines: &Vec<&str>, index: i32) -> Vec<i32> {
    let mut list = Vec::with_capacity(lines.len());

    for line in lines {
        let mut it = line.split("   ");

        for _ in 0..index {
            it.next();
        }

        list.push(it.next().unwrap().parse().unwrap())
    }

    list.sort();

    list
}
