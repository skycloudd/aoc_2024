use common::{parse_letter_grid, read_file};
use core::fmt::Display;

fn main() {
    let input = read_file(2024, 4);

    println!("input: {input}");

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> impl Display {
    let grid = parse_letter_grid(input);

    count_word_occurrences(&grid, "XMAS")
}

fn part2(input: &str) -> impl Display {
    let grid = parse_letter_grid(input);

    count_xmas_crosses(&grid)
}

fn count_word_occurrences(grid: &[Vec<char>], word: &str) -> usize {
    (0..grid.len())
        .flat_map(|y| (0..grid[y].len()).map(move |x| (x, y)))
        .map(|(x, y)| {
            (0..8)
                .filter(|&direction| check_word(grid, word, x, y, direction))
                .count()
        })
        .sum()
}

fn check_word(grid: &[Vec<char>], word: &str, x: usize, y: usize, direction: usize) -> bool {
    let (dx, dy) = match direction {
        0 => (1, 0),
        1 => (1, 1),
        2 => (0, 1),
        3 => (-1, 1),
        4 => (-1, 0),
        5 => (-1, -1),
        6 => (0, -1),
        7 => (1, -1),
        _ => unreachable!(),
    };

    let mut x = Some(x);
    let mut y = Some(y);

    for char in word.chars() {
        if let (Some(x_coord), Some(y_coord)) = (x, y) {
            if x_coord >= grid[0].len() || y_coord >= grid.len() {
                return false;
            }

            if grid[y_coord][x_coord] != char {
                return false;
            }

            x = (i32::try_from(x_coord).unwrap() + dx).try_into().ok();
            y = (i32::try_from(y_coord).unwrap() + dy).try_into().ok();
        } else {
            return false;
        }
    }

    true
}

fn count_xmas_crosses(grid: &[Vec<char>]) -> usize {
    (0..grid.len())
        .flat_map(|y| (0..grid[y].len()).map(move |x| (x, y)))
        .filter(|(x, y)| check_xmas_cross(grid, *x, *y))
        .count()
}

fn check_xmas_cross(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if grid[y][x] != 'A' {
        return false;
    }

    if x == 0 || y == 0 || x == grid[0].len() - 1 || y == grid.len() - 1 {
        return false;
    }

    let top_left = grid[y - 1][x - 1];
    let top_right = grid[y - 1][x + 1];
    let bottom_left = grid[y + 1][x - 1];
    let bottom_right = grid[y + 1][x + 1];

    ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M'))
        && ((top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M'))
}
