use aoc::{days, Day};

fn main() {
    const DAY: i32 = 11;
    let day = get_day(DAY);

    let path = format!("days/day{}.txt", DAY);
    let data = std::fs::read_to_string(path).expect(format!("Failed to read file for day: {}", DAY).as_str());
    let lines = data.lines().collect::<Vec<&str>>();

    run(&day, &lines, false);
    run(&day, &lines, true);
}

fn run(day: &Box<dyn Day>, lines: &[&str], part2: bool) {
    let before = std::time::Instant::now();
    let result = if part2 { day.part2(lines) } else { day.part1(lines) };
    let duration = before.elapsed();

    if part2 {
        println!("-- Part 2 --");
    } else {
        println!("-- Part 1 --");
    }

    println!("  Result: {}", result);
    println!("  Took: {:.2?}", duration);
    println!();
}

fn get_day(number: i32) -> Box<dyn Day> {
    match number {
        1 => Box::new(days::Day1 {}),
        2 => Box::new(days::Day2 {}),
        3 => Box::new(days::Day3 {}),
        4 => Box::new(days::Day4 {}),
        5 => Box::new(days::Day5 {}),
        6 => Box::new(days::Day6 {}),
        7 => Box::new(days::Day7 {}),
        8 => Box::new(days::Day8 {}),
        9 => Box::new(days::Day9 {}),
        10 => Box::new(days::Day10 {}),
        11 => Box::new(days::Day11 {}),
        _ => panic!("Invalid day number: {}", number),
    }
}
