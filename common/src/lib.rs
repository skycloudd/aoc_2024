use core::{fmt::Debug, str::FromStr};
use reqwest::{header::COOKIE, Method};
use std::path::PathBuf;

pub fn parse_into_columns<A, B, T>(input: &str) -> (A, B)
where
    A: Default + Extend<T>,
    B: Default + Extend<T>,
    T: FromStr<Err: Debug>,
{
    input.lines().map(split_into_numbers).unzip()
}

pub fn split_into_numbers<T>(line: &str) -> (T, T)
where
    T: FromStr<Err: Debug>,
{
    let mut parts = line.split_whitespace();

    let left = parts.next().unwrap().parse().unwrap();
    let right = parts.next().unwrap().parse().unwrap();

    (left, right)
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
