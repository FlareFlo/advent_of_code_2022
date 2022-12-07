use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::DirEntry;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct File {
    // List of directories from root
    full_path: Vec<String>,

    size: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Dir {
    full_path: String,

    entries: HashSet<Entry>
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Entry {
    File(File),
    Directory(Dir),
}

impl Entry {
    pub fn get_size(&self) -> usize {
        match self {
            Entry::File(file) => {file.size}
            Entry::Directory(dir) => {
                dir.entries.iter().map(|x|x.get_size()).sum()
            }
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Input should exist");

    let mut curr_path: Vec<String> = vec![];
    let mut root = Entry::Directory(Dir { full_path: "".to_string(), entries: HashSet::new() });

    let mut current = None;

    for line in file.lines() {
        match () {
            _ if line.starts_with('$') => {
                let instruction = &line[2..];

                match () {
                    _ if instruction.starts_with("ls") => {
                        continue;
                    }
                    _ if instruction.starts_with("cd") => {
                        let split = instruction.split("cd").last().expect("Should contain any values").trim();
                        match () {
                            _ if split == ".." => {
                                curr_path.pop();
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ if line.starts_with("dir") => {

            }
            _ => {

            }
        }
    }

}
