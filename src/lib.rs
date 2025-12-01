#![feature(bool_to_result)]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn read_input(name: &str) -> Result<Vec<String>, io::Error> {
    Ok(BufReader::new(File::open(format!("inputs/{name}.txt"))?)
        .lines()
        .map_while(Result::ok)
        .collect())
}
