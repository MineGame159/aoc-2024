use crate::Day;
use std::collections::HashSet;

pub struct Day6 {}

impl Day for Day6 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let (map, mut guard) = parse(lines);

        let path = patrol(&map, &mut guard, std::time::Duration::from_secs(1));
        path.len() as i64
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let (mut map, guard) = parse(lines);
        let mut count = 0;

        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                if map[y][x] || (guard.x == x && guard.y == y) {
                    continue;
                }

                map[y][x] = true;
                let mut guard2 = guard.clone();

                let path = patrol(&map, &mut guard2, std::time::Duration::from_millis(2));
                if path.len() == 0 {
                    count += 1;
                }

                map[y][x] = false;
            }
        }

        count
    }
}

fn patrol(map: &Vec<Vec<bool>>, guard: &mut Guard, limit: std::time::Duration) -> HashSet<(usize, usize)> {
    let mut path = HashSet::new();
    path.insert((guard.x, guard.y));

    let start = std::time::Instant::now();

    loop {
        let duration = std::time::Instant::now() - start;
        if duration > limit {
            return HashSet::new();
        }

        let offset = guard.dir.offset();
        let new_x = (guard.x as isize) + offset.0;
        let new_y = (guard.y as isize) + offset.1;

        if new_y < 0 || new_y >= map.len() as isize || new_x < 0 || new_x >= map[new_y as usize].len() as isize {
            break;
        }

        if map[new_y as usize][new_x as usize] {
            guard.dir = guard.dir.rotate_right();
        } else {
            guard.x = new_x as usize;
            guard.y = new_y as usize;

            path.insert((guard.x, guard.y));
        }
    }

    path
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        use Direction::*;

        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn offset(&self) -> (isize, isize) {
        use Direction::*;

        match self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }
}

#[derive(Clone)]
struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
}

fn parse(lines: &[&str]) -> (Vec<Vec<bool>>, Guard) {
    let mut map = Vec::new();

    let mut guard = Guard {
        x: 0,
        y: 0,
        dir: Direction::Up,
    };

    for y in 0..lines.len() {
        let mut row = Vec::with_capacity(lines.len());

        for x in 0..lines[y].len() {
            match lines[y].chars().nth(x).unwrap() {
                '.' => row.push(false),
                '#' => row.push(true),
                '^' => {
                    row.push(false);

                    guard.x = x;
                    guard.y = y;
                }
                _ => panic!("Invalid character"),
            }
        }

        map.push(row);
    }

    (map, guard)
}
