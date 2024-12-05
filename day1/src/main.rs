use common::{parse_into_columns, read_file};
use core::fmt::Display;
use std::collections::BinaryHeap;

fn main() {
    let input = read_file(2024, 1);

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> impl Display {
    let (mut left, mut right) = parse_into_columns::<BinaryHeap<_>, BinaryHeap<_>, i32>(input);

    let mut result = 0;

    while let (Some(l), Some(r)) = (left.pop(), right.pop()) {
        result += (l - r).abs();
    }

    result
}

fn part2(input: &str) -> impl Display {
    let (left, right) = parse_into_columns::<Vec<_>, Vec<_>, usize>(input);

    // let mut result = 0;

    // for i in left {
    //     result += i * right.iter().filter(|j| i == **j).count();
    // }

    // result

    left.into_iter()
        .map(|i| i * right.iter().filter(|j| i == **j).count())
        .sum::<usize>()
}
