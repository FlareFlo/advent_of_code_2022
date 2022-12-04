use std::fs;
use std::ops::RangeInclusive;

const PRIORITIES: (RangeInclusive<i32>, RangeInclusive<i32>) = (1..=26, 27..=52);

fn main() {
   let input = fs::read_to_string("input.txt").expect("Input should exist");

   let rucksacks = input.split('\n')
                                                 .map(|contents|contents.chars());


}
