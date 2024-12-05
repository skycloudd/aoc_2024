use common::read_file;
use core::fmt::Display;
use regex::Regex;
use std::sync::LazyLock;

fn main() {
    let input = read_file(2024, 3);

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> impl Display {
    let instructions = scan_for_instructions(input);

    instructions
        .into_iter()
        .filter_map(|instr| match instr {
            Instruction::Mul(a, b) => Some(a * b),
            Instruction::Do | Instruction::Dont => None,
        })
        .sum::<u32>()
}

fn part2(input: &str) -> impl Display {
    let instructions = scan_for_instructions(input);

    let mut enabled = true;

    instructions
        .into_iter()
        .filter_map(|instr| match instr {
            Instruction::Mul(a, b) if enabled => Some(a * b),
            Instruction::Mul(_, _) => None,
            Instruction::Do => {
                enabled = true;
                None
            }
            Instruction::Dont => {
                enabled = false;
                None
            }
        })
        .sum::<u32>()
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?<mul>mul)\((?<a>\d{1,3}),(?<b>\d{1,3})\)|(?<do>do)\(\)|(?<dont>don't)\(\)")
        .unwrap()
});

fn scan_for_instructions(input: &str) -> Vec<Instruction> {
    REGEX
        .captures_iter(input)
        .map(
            |cap| match (cap.name("mul"), cap.name("do"), cap.name("dont")) {
                (Some(_), None, None) => {
                    let a = cap["a"].parse().unwrap();
                    let b = cap["b"].parse().unwrap();

                    Instruction::Mul(a, b)
                }
                (None, Some(_), None) => Instruction::Do,
                (None, None, Some(_)) => Instruction::Dont,
                _ => unreachable!(),
            },
        )
        .collect()
}

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}
