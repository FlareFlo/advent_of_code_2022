use std::collections::HashSet;
use std::fs;
use std::process::exit;

fn main() {
	let file = fs::read_to_string("input.txt").expect("Input should exist").chars().collect::<Vec<char>>();

	for i in 4..file.len() {
		let tgt = file[(i - 4)..i].to_owned();
		if HashSet::<char>::from_iter(tgt).len() == 4 {
			println!("Task 1 is: {}", i);
			break;
		}
	}


	for i in 14..file.len() {
        let tgt = file[(i - 14)..i].to_owned();
		if HashSet::<char>::from_iter(tgt).len() == 14 {
			println!("Task 2 is: {}", i);
			break;
		}
	}
}
