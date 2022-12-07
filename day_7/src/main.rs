#![feature(if_let_guard)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::DirEntry;
use std::str::FromStr;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct File {
	pub size: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Dir {
	pub entries: Vec<Entry>,
}

impl Dir {
	pub fn get_size(&self) -> usize {
		self.entries.iter().map(|x| x.get_size()).sum()
	}
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Entry {
	File(File),
	Directory(Dir),
}

impl Entry {
	pub fn get_size(&self) -> usize {
		match self {
			Entry::File(file) => { file.size }
			Entry::Directory(dir) => {
				dir.get_size()
			}
		}
	}
}

fn main() {
	let file = fs::read_to_string("input.txt").expect("Input should exist");

	let mut at = 0;
	let entries = eval_line(&file.lines().collect::<Vec<_>>(), &mut at);


}

fn eval_line(lines: &[&str], at: &mut  usize) -> Vec<Entry> {
	let mut entries = vec![];
	loop {
		let line = &lines[*at];
		match () {
			_ if line== &"$ cd .." => {
				return entries;
			}
			// Ignore as this does nothing right now
			_ if line == &"$ ls" => {}
			_ if line.starts_with("$ cd") => {
				entries.append(&mut eval_line(lines, at))
			}
			_ if line.starts_with("dir") => {
				entries.push(Entry::Directory(Dir {
					entries: vec![],
				}));
			}
			_ if let Ok(size) = usize::from_str(line.split(" ").next().expect("Should have space within it")) => {
				entries.push(Entry::File(File {
					size,
				}))
			}
			_ => {
				panic!("Unclear expression for line {}", line);
			}
		}
		*at += 1;
	}
}


