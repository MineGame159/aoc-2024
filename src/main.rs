use aoc::{days, Day};

fn main() {
    const DAY: i32 = 1;
    let day = get_day(DAY);

    let path = format!("days/day{}.txt", DAY);
    let data = std::fs::read_to_string(path).expect(format!("Failed to read file for day: {}", DAY).as_str());
    let lines = data.lines().collect();

    println!("Part 1: {}", day.part1(&lines));
    println!("Part 2: {}", day.part2(&lines));
}

fn get_day(number: i32) -> Box<dyn Day> {
    match number {
        1 => Box::new(days::Day1 {}),
        _ => panic!("Invalid day number: {}", number),
    }
}
