#![feature(if_let_guard)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::DirEntry;
use std::process::exit;
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
	pub fn is_dir(&self) -> bool {
		match self {
			Entry::File(_) => {
				false
			}
			Entry::Directory(_) => {
				true
			}
		}
	}
	pub fn is_file(&self) -> bool {
		!self.is_dir()
	}
	pub fn file(&mut self) -> Option<&mut File> {
		match self {
			Entry::File(file) => {Some(file)}
			Entry::Directory(_) => {None}
		}
	}
	pub fn dir(&mut self) -> Option<&mut Dir> {
		match self {
			Entry::File(_) => {None}
			Entry::Directory(dir) => {Some(dir)}
		}
	}
}

fn main() {
	let file = fs::read_to_string("input.txt").expect("Input should exist");

	let mut at = 1;
	let entries = eval_line(&file.lines().collect::<Vec<_>>(), &mut at);
	dbg!(entries);

}

fn eval_line(lines: &[&str], at: &mut  usize) -> Vec<Entry> {
	let mut entries = vec![];
	loop {
		let line = &lines[*at];

		match () {
			_ if line.starts_with("$ cd ..") => {
				*at += 1;
				return entries;
			}
			// Ignore as this does nothing right now
			_ if line.starts_with("$ ls") => {
			}
			_ if line.starts_with("$ cd") => {
				*at += 1;
				let dirname = line.split(" ").nth(2).expect("Should have a last element");
				println!("{}", "-------------------------");
				dbg!(&entries, dirname);
				let target_folder = entries.iter_mut()
										   .filter(|x|x.get_name() == dirname)
										   .next().expect("Should always find exactly one directory")
										   .dir().expect("Has to be a directory");
				let mut target_folder_contents = eval_line(lines, at);
				target_folder.entries.append(&mut target_folder_contents);
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
		*at += 1;
	}
}


