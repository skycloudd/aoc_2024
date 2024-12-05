use common::{parse_into_vec_of_vecs, read_file};
use core::fmt::Display;

fn main() {
    let input = read_file(2024, 2);

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> impl Display {
    let rows = parse_into_vec_of_vecs::<u32>(input);

    rows.into_iter().filter(|line| is_safe_line(line)).count()
}

fn part2(input: &str) -> impl Display {
    let rows = parse_into_vec_of_vecs::<u32>(input);

    rows.into_iter()
        .filter(|line| is_safe_line(line) || is_tolerated(line))
        .count()
}

fn is_safe_line(line: &[u32]) -> bool {
    is_monotonic(line) && is_all_low_diff(line)
}

fn is_monotonic(line: &[u32]) -> bool {
    let mut orderings = line.windows(2).map(|pair| pair[0].cmp(&pair[1]));

    let first = orderings.next().unwrap();

    orderings.all(|ordering| ordering == first)
}

fn is_all_low_diff(line: &[u32]) -> bool {
    let mut diffs = line.windows(2).map(|pair| pair[0].abs_diff(pair[1]));

    diffs.all(|diff| (1..=3).contains(&diff))
}

fn is_tolerated(line: &[u32]) -> bool {
    (0..line.len())
        .map(|i| line_without_index(line, i))
        .any(|line| is_safe_line(&line))
}

fn line_without_index(line: &[u32], index: usize) -> Vec<u32> {
    let mut line = line.to_vec();
    line.remove(index);
    line
}
