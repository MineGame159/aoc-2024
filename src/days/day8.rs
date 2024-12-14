use crate::Day;

pub struct Day8 {}

impl Day for Day8 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let mut tiles = parse(lines);
        place_antinodes_at_double_distance(&mut tiles);

        tiles.iter().flatten().filter(|tile| tile.antinode).count() as i64
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let mut tiles = parse(lines);
        place_antinodes_in_line(&mut tiles);

        tiles.iter().flatten().filter(|tile| tile.antinode).count() as i64
    }
}

fn place_antinodes_at_double_distance(tiles: &mut Vec<Vec<Tile>>) {
    let antennas = get_antennas(tiles);

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let antenna1 = &antennas[i];
            let antenna2 = &antennas[j];

            if antenna1.frequency != antenna2.frequency {
                continue;
            }

            let dir_x = (antenna1.x as isize) - (antenna2.x as isize);
            let dir_y = (antenna1.y as isize) - (antenna2.y as isize);

            place_antinode(tiles, (antenna1.x as isize) + dir_x, (antenna1.y as isize) + dir_y);
            place_antinode(tiles, (antenna2.x as isize) - dir_x, (antenna2.y as isize) - dir_y);
        }
    }
}

fn place_antinodes_in_line(tiles: &mut Vec<Vec<Tile>>) {
    let antennas = get_antennas(tiles);

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let antenna1 = &antennas[i];
            let antenna2 = &antennas[j];

            if antenna1.frequency != antenna2.frequency {
                continue;
            }

            let dir_x = (antenna1.x as isize) - (antenna2.x as isize);
            let dir_y = (antenna1.y as isize) - (antenna2.y as isize);

            {
                let mut x = antenna1.x as isize;
                let mut y = antenna1.y as isize;

                while (place_antinode(tiles, x, y)) {
                    x += dir_x;
                    y += dir_y;
                }
            }

            {
                let mut x = antenna2.x as isize;
                let mut y = antenna2.y as isize;

                while (place_antinode(tiles, x, y)) {
                    x -= dir_x;
                    y -= dir_y;
                }
            }
        }
    }
}

fn place_antinode(tiles: &mut Vec<Vec<Tile>>, x: isize, y: isize) -> bool {
    if y >= 0 && y < tiles.len() as isize && x >= 0 && x < tiles[y as usize].len() as isize {
        tiles[y as usize][x as usize].antinode = true;
        return true;
    }

    false
}

struct Antenna {
    frequency: char,
    x: usize,
    y: usize,
}

fn get_antennas(tiles: &Vec<Vec<Tile>>) -> Vec<Antenna> {
    let mut antennas = Vec::new();

    for y in 0..tiles.len() {
        for x in 0..tiles[y].len() {
            let frequency = tiles[y][x].frequency;

            if frequency != '\0' {
                antennas.push(Antenna { frequency, x, y });
            }
        }
    }

    antennas
}

struct Tile {
    frequency: char,
    antinode: bool,
}

fn parse(lines: &[&str]) -> Vec<Vec<Tile>> {
    let mut tiles = Vec::with_capacity(lines.len());

    for y in 0..lines.len() {
        let mut row = Vec::with_capacity(lines[y].len());

        for x in 0..lines[y].len() {
            row.push(Tile {
                frequency: match lines[y].chars().nth(x).unwrap() {
                    '.' => '\0',
                    ch => ch,
                },
                antinode: false,
            });
        }

        tiles.push(row);
    }

    tiles
}
