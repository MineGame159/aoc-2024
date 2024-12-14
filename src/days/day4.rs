use crate::Day;

pub struct Day4 {}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS: [char; 3] = ['M', 'A', 'S'];

impl Day for Day4 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let mut count = 0;

        for y in 0..lines.len() as isize {
            for x in 0..lines[y as usize].len() as isize {
                count += is_word(lines, x, y, 1, 0, XMAS);
                count += is_word(lines, x, y, -1, 0, XMAS);

                count += is_word(lines, x, y, 0, 1, XMAS);
                count += is_word(lines, x, y, 0, -1, XMAS);

                count += is_word(lines, x, y, 1, 1, XMAS);
                count += is_word(lines, x, y, -1, 1, XMAS);
                count += is_word(lines, x, y, 1, -1, XMAS);
                count += is_word(lines, x, y, -1, -1, XMAS);
            }
        }

        count
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let mut count = 0;

        for y in 0..lines.len() as isize {
            for x in 0..lines[y as usize].len() as isize {
                count += is_x_word(lines, x, y, MAS);
            }
        }

        count
    }
}

fn is_x_word<const N: usize>(lines: &[&str], x: isize, y: isize, word: [char; N]) -> i64 {
    let mut count = 0;

    count += is_word(lines, x - 1, y - 1, 1, 1, word);
    count += is_word(lines, x + 1, y - 1, -1, 1, word);

    count += is_word(lines, x - 1, y + 1, 1, -1, word);
    count += is_word(lines, x + 1, y + 1, -1, -1, word);

    if count >= 2 {
        return 1;
    }

    0
}

fn is_word<const N: usize>(lines: &[&str], x: isize, y: isize, dir_x: isize, dir_y: isize, word: [char; N]) -> i64 {
    let mut chars = ['\0'; N];

    for i in 0..N {
        let x = x + dir_x * (i as isize);
        let y = y + dir_y * (i as isize);

        if y < 0 || y >= lines.len() as isize || x < 0 || x >= lines[y as usize].len() as isize {
            return 0;
        }

        chars[i] = lines[y as usize].chars().nth(x as usize).unwrap();
    }

    if chars == word {
        return 1;
    }

    0
}
