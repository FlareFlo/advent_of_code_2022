use std::fs::read_to_string;
use std::iter::{Once, once};
use std::str::FromStr;

fn main() {
	let raw_containers = read_to_string("input_containers").unwrap();
	let instructions = read_to_string("input_instructions").unwrap();

	let mut containers = vec![vec![]; 9];

	for (row, line) in raw_containers.lines().enumerate() {
		for col in 0..(line.len() / 4 + 1) {
			let idx = 1 + col * 4;
			let val = line[idx..=idx].to_owned();
			if val == " " {
				continue;
			}
			containers[col].push(val);
		}
	}

	containers.iter_mut().for_each(|x|x.reverse());

	for line in instructions.lines() {
		let mut x = line.split(' ')
						   .map(|x| usize::from_str(x).ok())
						   .filter_map(|x| x);
		let (count, source, destination) = (x.next().unwrap(), x.next().unwrap(), x.next().unwrap());

		for i in 0..count {
			let container = containers[source - 1].pop().unwrap();
			containers[destination - 1].push(container);
		}
	}


	for container in containers {
		print!("{}", container.last().unwrap());
	}
}
