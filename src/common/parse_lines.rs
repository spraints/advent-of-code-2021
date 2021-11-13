use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;

use super::read_lines::read_lines;

pub fn parse_lines<T, R>(r: R) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    R: Read,
{
    read_lines(r).map(|line| line.parse().unwrap()).collect()
}
