use core::{fmt::Debug, str::FromStr};
use reqwest::{header::COOKIE, Method};
use std::path::PathBuf;

/// # Examples
///
/// ```
/// use common::parse_into_columns;
///
/// let (a, b) = parse_into_columns::<Vec<u32>, Vec<u32>, u32>("12 34\n5 6\n7 8");
/// assert_eq!(a, vec![12, 5, 7]);
/// assert_eq!(b, vec![34, 6, 8]);
/// ```
pub fn parse_into_columns<A, B, T>(input: &str) -> (A, B)
where
    A: Default + Extend<T>,
    B: Default + Extend<T>,
    T: FromStr<Err: Debug>,
{
    input.lines().map(split_into_numbers).unzip()
}

/// # Examples
///
/// ```
/// use common::split_into_numbers;
///
/// let (a, b) = split_into_numbers::<u32>("123 456");
/// assert_eq!(a, 123);
/// assert_eq!(b, 456);
/// ```
pub fn split_into_numbers<T>(line: &str) -> (T, T)
where
    T: FromStr<Err: Debug>,
{
    let mut parts = line.split_whitespace();

    let left = parts.next().unwrap().parse().unwrap();
    let right = parts.next().unwrap().parse().unwrap();

    (left, right)
}

/// # Examples
///
/// ```
/// use common::parse_into_vec_of_vecs;
///
/// let input = "1 2 3\n4 5\n6 7 8 9";
/// let result = parse_into_vec_of_vecs::<u32>(input);
/// assert_eq!(result, vec![
///     vec![1, 2, 3],
///     vec![4, 5],
///     vec![6, 7, 8, 9]
/// ]);
pub fn parse_into_vec_of_vecs<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr<Err: Debug>,
{
    input.lines().map(parse_into_vec).collect()
}

/// # Examples
///
/// ```
/// use common::parse_letter_grid;
///
/// let input = "abcd\nefgh\nijkl\nmnop";
/// let result = parse_letter_grid(input);
/// assert_eq!(result, vec![
///     vec!['a', 'b', 'c', 'd'],
///     vec!['e', 'f', 'g', 'h'],
///     vec!['i', 'j', 'k', 'l'],
///     vec!['m', 'n', 'o', 'p']
/// ]);
pub fn parse_letter_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// # Examples
///
/// ```
/// use common::parse_into_vec;
///
/// let result = parse_into_vec::<u32>("1 2 3 4");
/// assert_eq!(result, vec![1, 2, 3, 4]);
/// ```
pub fn parse_into_vec<T>(line: &str) -> Vec<T>
where
    T: FromStr<Err: Debug>,
{
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn read_file(year: i32, day: i32) -> String {
    if let Some(text) = read_cache(year, day) {
        return text;
    }

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let text = reqwest::blocking::Client::new()
        .request(Method::GET, url)
        .header(COOKIE, session_cookie())
        .send()
        .unwrap()
        .text()
        .unwrap();

    cache_input(year, day, &text);

    text
}

fn read_cache(year: i32, day: i32) -> Option<String> {
    std::fs::read_to_string(cache_path(year, day)).ok()
}

fn cache_input(year: i32, day: i32, text: &str) {
    std::fs::write(cache_path(year, day), text).unwrap();
}

fn cache_path(year: i32, day: i32) -> PathBuf {
    let directory = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("cache")
        .join(year.to_string())
        .join(day.to_string());

    std::fs::create_dir_all(&directory).unwrap();

    directory.join("input")
}

fn session_cookie() -> String {
    std::fs::read_to_string("session_cookie.txt").unwrap()
}
