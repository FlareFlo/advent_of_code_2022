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
	//								Top left Top right
	pub fn max_dimensions(&self) -> (Point, Point) {
		let (
			mut top,
			mut bottom,
			mut left,
			mut right
		) = (
			usize::MAX,
			usize::MIN,
			usize::MAX,
			usize::MIN
		);

		let mut set_bounds_point =|lhs: Point| {
			if lhs.0 < left {
				left = lhs.0;
			}
			if lhs.0 > right {
				right = lhs.0;
			}
			if lhs.1 < top {
				top = lhs.1;
			}
			if lhs.1 > bottom {
				bottom = lhs.1;
			}
		};

		for item in &self.items {
			set_bounds_point(item.0);
			set_bounds_point(item.1);
		}

		(Point(left, top), Point(right, bottom))
	}
	pub fn max_dimensions_all(items: &[Self]) -> (Point, Point) {
		let (
			mut top,
			mut bottom,
			mut left,
			mut right
		) = (
			usize::MAX,
			usize::MIN,
			usize::MAX,
			usize::MIN
		);

		// Dupe i know, dont have time to fix this right now
		let mut set_bounds_point =|lhs: Point| {
			if lhs.0 < left {
				left = lhs.0;
			}
			if lhs.0 > right {
				right = lhs.0;
			}
			if lhs.1 < top {
				top = lhs.1;
			}
			if lhs.1 > bottom {
				bottom = lhs.1;
			}
		};

		for item in items {
			let local = item.max_dimensions();
			set_bounds_point(local.0);
			set_bounds_point(local.1);
		}
		(Point(left, top), Point(right, bottom))
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

	let paths = input.lines().map(|path| RockPath::from_line(path)).collect::<Vec<_>>();

	eprintln!(" {:?}", RockPath::max_dimensions_all(&paths));
}
