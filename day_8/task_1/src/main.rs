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

	let mut trees: HashSet<(usize, usize)> = HashSet::new();

	// From left
	walk_forest(&map, &mut trees, 0..width, 0..height, true);

	// From right
	walk_forest(&map, &mut trees, (0..width).rev(), 0..height, true);

	// From top
	walk_forest(&map, &mut trees, 0..width, 0..height, false);

	//From bottom
	walk_forest(&map, &mut trees, 0..width, (0..height).rev(),false);

	println!("{}", trees.len());
}

fn walk_forest(
	map: &Vec<Vec<i8>>,
	trees: &mut HashSet<(usize, usize)>,
	x_dim: impl Iterator<Item=usize> + Clone,
	y_dim: impl Iterator<Item=usize> + Clone,
	from_side: bool,
) {
	if from_side {
		for i in y_dim {
			let mut curr_max = -1;
			for j in x_dim.clone() {
				idx_insert(map, trees, &mut curr_max, (j, i));
			}
		}
	} else {
		for i in x_dim {
			let mut curr_max = -1;
			for j in y_dim.clone() {
				idx_insert(map, trees, &mut curr_max, (i, j));
			}
		}
	}
}

fn idx_insert(
	map: &Vec<Vec<i8>>,
	trees: &mut HashSet<(usize, usize)>,
	current_maximum: &mut i8,
	(x, y): (usize, usize))
{
	let tree = map[y][x];
	if tree > *current_maximum {
		trees.insert((x, y));
		*current_maximum = tree;
	}
}