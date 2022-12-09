use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;
use std::fs;
use std::iter::Rev;
use std::ops::{Deref, Range};
use std::str::FromStr;

fn main() {
	let input = fs::read_to_string("../input.txt").expect("Input should exist");

	let width = input.lines().next().unwrap().chars().count();
	let height = input.lines().count();

	let map = input.lines().map(|x|
		x.chars().map(|y|
			i8::from_str(&y.to_string()).expect("Should be integer")
		).collect::<Vec<_>>()
	).collect::<Vec<Vec<_>>>();

	let mut highest_score = 0;

	let get_tree_height = |(x, y): (usize, usize)| -> i8 {
		map[y][x]
	};

	let score_tree = |(x, y): (usize, usize)| {
		let center = get_tree_height((x, y));

		// To the right
		let mut right = 0;
		let mut max = 0;
		let right_rng = ((x + 1)..width).collect::<Vec<usize>>();
		for x_cord in right_rng {
			let height = get_tree_height((x_cord, y));
			if height > max {
				right += 1;
				max = height;
			} else {
				break;
			}
		}

		// To the left
		let mut left = 0;
		let mut max = 0;
		let left_rng = ((0)..(width - x - 1)).rev().collect::<Vec<usize>>();
		for x_cord in left_rng {
			let height = get_tree_height((x_cord, y));
			if height > max {
				left += 1;
				max = height;
			} else {
				break;
			}
		}

		//  upwards
		let mut upwards = 0;
		let mut max = 0;
		let up_rng = (0..(y)).rev().collect::<Vec<usize>>();
		eprintln!("up = {:?}", &up_rng);
		for y_cord in up_rng {
			let height = get_tree_height((x, y_cord));
			if height > max {
				upwards += 1;
				max = height;
			} else {
				break;
			}
		}

		//  downwards
		let mut downwards = 0;
		let mut max = 0;
		let down_rng = ((y + 1)..height).collect::<Vec<usize>>();
		for y_cord in down_rng {
			let height = get_tree_height((x, y_cord));
			if height > max {
				downwards += 1;
				max = height;
			} else {
				break;
			}
		}

		println!(
			"   {}	\n{}--{}--{}\n   {}"
			, upwards, left, (right * left * upwards * downwards), right, downwards);
		right * left * upwards * downwards
	};

	score_tree((2, 3));
	// for (y, row) in map.iter().enumerate() {
	// 	for (x, _) in row.iter().enumerate() {
	// 		let score = score_tree((x, y));
	// 		if score > highest_score {
	// 			highest_score = score;
	// 		}
	// 	}
	// }
	//  println!("{}", highest_score);
}