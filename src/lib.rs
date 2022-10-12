use clap::Parser;
use std::str::{FromStr, self};

pub mod bubble_sort;
pub mod insertion_sort;

#[derive(Debug, PartialEq)]
pub enum SortingAlgo {
    BubbleSort,
    InsertionSort,
}

impl FromStr for SortingAlgo {
    fn from_str(s: &str) -> Result<SortingAlgo, &'static str> {
        if s.eq_ignore_ascii_case("bubblesort") {
            return Ok(SortingAlgo::BubbleSort);
        } else if s.eq_ignore_ascii_case("insertionsort") {
            return Ok(SortingAlgo::InsertionSort);
        } else {
            return Err("Invalid input command");
        }
    }
    type Err = &'static str;
}

#[derive(Debug, Parser)]
pub struct UserInput {
    pub input_commnad: SortingAlgo,
    pub input_value: Vec<i32>,
}
