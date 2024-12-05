use common::{read_file, split_into_numbers_by, split_into_numbers_vec_by};
use core::{cmp::Ordering, fmt::Display};

fn main() {
    let input = read_file(2024, 5);

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> impl Display {
    let (rules, updates) = parse(input);

    updates
        .into_iter()
        .filter(|update| update_in_correct_order(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
}

fn part2(input: &str) -> impl Display {
    let (rules, updates) = parse(input);

    updates
        .into_iter()
        .filter(|update| !update_in_correct_order(&rules, update))
        .map(|update| fix_order(&rules, &update))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
}

fn update_in_correct_order(rules: &[(u32, u32)], update: &[u32]) -> bool {
    let mut recreated = vec![];

    for num in update {
        for (before, after) in rules {
            if num == before && recreated.contains(after) {
                return false;
            }
        }

        recreated.push(*num);
    }

    true
}

fn fix_order(rules: &[(u32, u32)], update: &[u32]) -> Vec<u32> {
    let mut update = update.to_vec();

    update.sort_by(|a, b| {
        for (before, after) in rules {
            if a == before && b == after {
                return Ordering::Less;
            }

            if a == after && b == before {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    });

    update
}

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = vec![];
    let mut updates = vec![];

    let mut section = 0;

    for line in input.lines() {
        if line.is_empty() {
            section += 1;
            continue;
        }

        match section {
            0 => {
                let (before, after) = split_into_numbers_by(line, "|");

                rules.push((before, after));
            }
            1 => {
                let update = split_into_numbers_vec_by(line, ",");

                updates.push(update);
            }
            _ => unreachable!(),
        }
    }

    (rules, updates)
}
