use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::path::PathBuf;

pub struct Directory {
    file_size: i32,
    children_names: Vec<String>,
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Directory")
            .field("file_size", &self.file_size)
            .field("children_names", &self.children_names)
            .finish()
    }
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> HashMap<String, Directory> {
    let mut current_directory = PathBuf::new();
    let mut directories: HashMap<String, Directory> = HashMap::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let location = line.split(' ').next_back().unwrap();
            match location {
                "/" => {
                    while current_directory.pop() {}
                    current_directory.push("/");
                }
                ".." => {
                    current_directory.pop();
                }
                x => current_directory.push(x),
            }
            let current_directory_name = current_directory.to_str().unwrap();
            directories
                .entry(current_directory_name.to_string())
                .or_insert(Directory {
                    file_size: 0,
                    children_names: Vec::new(),
                });
        } else if line.starts_with("dir") {
            let current_directory_name = current_directory.to_str().unwrap();
            directories
                .entry(current_directory_name.to_string())
                .or_insert(Directory {
                    file_size: 0,
                    children_names: Vec::new(),
                });

            let directory = directories.remove(current_directory_name).unwrap();
            let file_size = directory.file_size;
            let mut children_names = directory.children_names;

            let mut child_path = current_directory.clone();
            child_path.push(line.split(' ').next_back().unwrap());
            let child_directory_name = child_path.to_str().unwrap();
            children_names.push(child_directory_name.to_string());
            directories.insert(
                current_directory_name.to_string(),
                Directory {
                    file_size,
                    children_names,
                },
            );
            directories
                .entry(child_directory_name.to_string())
                .or_insert(Directory {
                    file_size: 0,
                    children_names: Vec::new(),
                });
        } else if !line.starts_with("$ ls") {
            let current_directory_name = current_directory.to_str().unwrap();
            let size: i32 = line.split(' ').next().unwrap().parse().unwrap();

            directories
                .entry(current_directory_name.to_string())
                .or_insert(Directory {
                    file_size: 0,
                    children_names: Vec::new(),
                });
            let directory = directories.remove(current_directory_name).unwrap();

            directories.insert(
                current_directory_name.to_string(),
                Directory {
                    file_size: directory.file_size + size,
                    children_names: directory.children_names,
                },
            );
        }
    }
    directories
}

#[aoc(day7, part1)]
pub fn solve_part1(directories: &HashMap<String, Directory>) -> i32 {
    let mut result = 0;
    for (_, directory) in directories {
        let mut children_names = VecDeque::from_iter(&directory.children_names);
        let mut size = directory.file_size;
        while !children_names.is_empty() {
            let current_child_name = children_names.pop_back().unwrap();
            let current_child = directories.get(current_child_name).unwrap();
            size += current_child.file_size;
            for child_name in &current_child.children_names {
                children_names.push_back(child_name);
            }
        }
        if size <= 100000 {
            result += size;
        }
    }
    result
}

#[aoc(day7, part2)]
pub fn solve_part2(directories: &HashMap<String, Directory>) -> i32 {
    let mut directory_sizes: HashMap<String, i32> = HashMap::new();
    for (directory_name, directory) in directories {
        let mut children_names = VecDeque::from_iter(&directory.children_names);
        let mut directory_size = directory.file_size;
        while !children_names.is_empty() {
            let current_child_name = children_names.pop_back().unwrap();
            let current_child = directories.get(current_child_name).unwrap();
            directory_size += current_child.file_size;
            for child_name in &current_child.children_names {
                children_names.push_back(child_name);
            }
        }
        directory_sizes.insert(directory_name.to_string(), directory_size);
    }
    let available_space = 70000000;
    let required_space = 30000000;
    let used_space = directory_sizes.get("/").unwrap();
    let missing_space = required_space - (available_space - used_space);
    println!(
        "available:{} required:{} used:{} missing:{}",
        available_space, required_space, used_space, missing_space
    );
    let mut min_deleted_space = available_space;
    for (_, directory_size) in directory_sizes {
        if directory_size >= missing_space && directory_size < min_deleted_space {
            min_deleted_space = directory_size;
        }
    }
    min_deleted_space
}
