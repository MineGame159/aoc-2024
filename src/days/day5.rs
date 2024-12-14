use crate::Day;
use std::cmp::Ordering;

pub struct Day5 {}

impl Day for Day5 {
    fn part1(&self, lines: &[&str]) -> i64 {
        let rules = parse_rules(lines);
        let updates = parse_updates(lines);

        updates
            .iter()
            .filter(|update| is_correct(&rules, update))
            .map(|update| update[update.len() / 2] as i64)
            .sum()
    }

    fn part2(&self, lines: &[&str]) -> i64 {
        let rules = parse_rules(lines);
        let updates = parse_updates(lines);

        updates
            .iter()
            .filter(|update| !is_correct(&rules, update))
            .map(|update| {
                let mut update = update.clone();

                update.sort_by(|x, y| {
                    if rules.contains(&(*x, *y)) {
                        return Ordering::Less;
                    }

                    if rules.contains(&(*y, *x)) {
                        return Ordering::Greater;
                    }

                    Ordering::Equal
                });

                update
            })
            .map(|update| update[update.len() / 2] as i64)
            .sum()
    }
}

fn is_correct(rules: &[(usize, usize)], update: &[usize]) -> bool {
    for i in 0..update.len() {
        let page1 = update[i];

        for j in i + 1..update.len() {
            let page2 = update[j];

            if rules.contains(&(page2, page1)) {
                return false;
            }
        }
    }

    true
}

fn parse_rules(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut rules = Vec::new();

    for line in lines {
        if line.contains('|') {
            let mut it = line.split('|');
            rules.push((it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap()));
        }
    }

    rules
}

fn parse_updates(lines: &[&str]) -> Vec<Vec<usize>> {
    let mut updates = Vec::new();

    for line in lines {
        if line.contains(',') {
            updates.push(line.split(',').map(|page| page.parse().unwrap()).collect());
        }
    }

    updates
}
