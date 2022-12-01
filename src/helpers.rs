/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::str::FromStr;
use core::fmt::Debug;
use regex::Regex;

pub fn lines_as_numbers<T: FromStr> (input: &str) -> Vec<T> 
where <T as FromStr>::Err: Debug {
    input
        .lines()
        .enumerate()
        .map(|(_, string)|
            string.parse::<T>().unwrap()
        )
        .collect()
 }

 pub fn as_parts<'a>(input: &'a str) -> Vec<&'a str>{
    Regex::new("\n\n")
        .unwrap()
        .split(input)
        .map(|t| t.trim())
        .collect()
 }