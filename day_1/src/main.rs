use std::fs;
use std::str::FromStr;

fn main() {
	let input = fs::read_to_string("input.txt").unwrap();

	let mut elves = vec![];

	let mut curr = 0;

	for line in input.split('\n') {
		if !line.is_empty() {
			curr += u32::from_str(line).unwrap();
		} else {
			elves.push(curr);
			curr = 0;
		}
	}

	elves.sort();
	elves.reverse();

	println!("Carrying most: {}", elves[0]);
	println!("Top 3: {}", elves[..3].iter().sum::<u32>());
}
