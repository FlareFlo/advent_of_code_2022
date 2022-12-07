#![feature(if_let_guard)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::DirEntry;
use std::str::FromStr;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct File {
	pub name: String,
	pub size: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Dir {
	pub name: String,
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
	pub fn get_name(&self) -> &str {
		match self {
			Entry::File(file) => {
				&file.name
			}
			Entry::Directory(dir) => {
				&dir.name
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
		*at += 1;
		let line = &lines[*at - 1];
		match () {
			_ if line.starts_with("$ cd ..") => {
				return entries;
			}
			// Ignore as this does nothing right now
			_ if line.starts_with("$ ls") => {
				continue;
			}
			_ if line.starts_with("$ cd") => {
				let dirname = line.split(" ").nth(2).expect("Should have a last element");
				// entries.append(&mut eval_line(lines, at))
				println!("{}", dirname);
				if dirname == "/" {
					continue;
				}
				let filtered = entries.iter_mut().filter(|x|x.get_name() == dirname).next().expect("Should find one element");
				match filtered {
					Entry::Directory(dir) => {
						dir.entries = eval_line(&lines, at);
					}
					_ => {
						panic!("Not a dir entry?!")
					}
				}
			}
			_ if line.starts_with("dir") => {
				entries.push(Entry::Directory(Dir {
					name: line.split(" ").nth(1).expect("Should have spacebar").to_owned(),
					entries: vec![],
				}));
			}
			_ if let Ok(size) = usize::from_str(line.split(" ").next().expect("Should have space within it")) => {
				entries.push(Entry::File(File {
					name: line.split(" ").nth(1).expect("Should exist").to_owned(),
					size,
				}))
			}
			_ => {
				panic!("Unclear expression for line {}", line);
			}
		}
		dbg!(*at, &entries);
	}
}


