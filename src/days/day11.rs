use crate::Day;

pub struct Day11 {}

impl Day for Day11 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let mut from = parse(lines);
        let mut to = Vec::new();

        for _ in 0..25 {
            for stone in &from {
                apply(*stone, &mut to);
            }

            (from, to) = (to, from);
            to.clear();
        }

        from.len() as i64
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let mut from = parse(lines);
        let mut to = Vec::new();

        for i in 0..40 {
            for stone in &from {
                apply(*stone, &mut to);
            }

            (from, to) = (to, from);
            to.clear();

            //println!("{}", i);
        }

        from.len() as i64
    }
}

fn apply(stone: u64, stones: &mut Vec<u64>) {
    if stone == 0 {
        stones.push(1);
        return;
    }

    let digits = digits(stone);

    if digits % 2 == 0 {
        let (left, right) = split(digits, stone);

        stones.push(left);
        stones.push(right);

        return;
    }

    stones.push(stone * 2024);
}

fn split(digits: u32, mut n: u64) -> (u64, u64) {
    let mut a = 0;
    let mut ai = 1;

    let mut b = 0;
    let mut bi = 1;

    let half = digits / 2;

    for _ in 0..half {
        b += n % 10 * bi;
        bi *= 10;

        n /= 10;
    }

    for _ in 0..half {
        a += n % 10 * ai;
        ai *= 10;

        n /= 10;
    }

    (a, b)
}

fn digits(mut n: u64) -> u32 {
    let mut len = 0;

    while n > 0 {
        n /= 10;
        len += 1;
    }

    len
}

fn parse(lines: &[&str]) -> Vec<u64> {
    lines[0].split(' ').map(|str| str.parse().unwrap()).collect()
}
