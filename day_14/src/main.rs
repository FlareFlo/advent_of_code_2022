use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RockPath {
	items: Vec<(Point, Point)>,
}

impl RockPath {
	pub fn from_line(line: &str) -> Self {
		let mut items = vec![];

		let split = line.split(" -> ").collect::<Vec<_>>();
		let mut str_at = 0;

		while items.len() < split.len() - 1 {
			let start = Point::from_comma_str(split[str_at]);
			let end = Point::from_comma_str(split[str_at + 1]);
			items.push((start, end));
			str_at += 1;
		}


		Self {
			items,
		}
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point(usize, usize);

impl Point {
	pub fn from_comma_str(str: &str) -> Self {
		let mut split = str.split(",");
		let mut get_next_split = || usize::from_str(split.next().unwrap()).unwrap();
		Self {
			0: get_next_split(),
			1: get_next_split(),
		}
	}
}

impl Display for Point {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "({},{})", self.0, self.1)
	}
}


fn main() {
	let input = fs::read_to_string("sample_input.txt").unwrap();

	let paths = input.lines().map(|path| RockPath::from_line(path));
}
