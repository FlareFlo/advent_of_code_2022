use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;
use std::fs;
use std::iter::Rev;
use std::ops::{Deref, Range};
use std::str::FromStr;

fn main() {
	let input = fs::read_to_string("../sample_input.txt").expect("Input should exist");

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
		// To the right
		let mut right = 0;
		let mut max = 0;
		for x_cord in (x + 1)..width {
			let height = get_tree_height((x_cord, y));
			if height  <= max {
				break;
			} else {
				right += 1;
				max = height;
			}
		}

		// To the left
		let mut left = 0;
		let mut max = 0;
		let rng = ((0)..(width - 2)).rev().collect::<Vec<usize>>();
		for x_cord in rng  {
			let height = get_tree_height((x_cord, y));
			println!("{}", height);
			if  height <= max {
				break;
			} else {
				left += 1;
				max = height;
			}
		}


		dbg!(left, right);
		right * left
	};

	println!("Highest total: {}", score_tree( (3, 3)));
	/*
	for (y, row) in map.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			let score = score_tree(*cell, (x,y));
			if score > highest_score {
				highest_score = score;
			}
		}
	}
	 */
	//  println!("{}", highest_score);
}