use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("./day7.input");

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    size: usize,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Folder {
    name: String,
    files: HashMap<String, File>,
    folders: HashMap<String, Folder>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path(Vec<String>);

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            files: HashMap::new(),
            folders: HashMap::new(),
        }
    }

    fn get_folder_mut(&mut self, path: &Path) -> Option<&mut Self> {
        if path.0.is_empty() {
            return Some(self);
        }

        let (name, rest) = path.0.split_first().unwrap();

        self.folders
            .get_mut(name)
            .and_then(|folder| folder.get_folder_mut(&Path(rest.to_vec())))
    }

    fn add_folder(&mut self, folder: Folder) {
        self.folders.insert(folder.name.clone(), folder);
    }

    fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }

    fn get_size(&self) -> usize {
        let mut size = 0;

        for file in self.files.values() {
            size += file.size;
        }

        for folder in self.folders.values() {
            size += folder.get_size();
        }

        size
    }
}

#[derive(Debug, Clone)]
struct State {
    file_system: Folder,
    current_folder: Path,
}

impl State {
    fn new() -> Self {
        Self {
            file_system: Folder::new("".to_string()),
            current_folder: Path(vec![]),
        }
    }
}

fn execute_lines(lines: &[&str], state: &mut State) {
    let mut lines = lines.iter().peekable();

    while let Some(line) = lines.next() {
        if line.starts_with("$") {
            let command_parts = line.split_whitespace().collect_vec();

            match command_parts[1] {
                "cd" => {
                    if command_parts[2] == "/" {
                        state.current_folder = Path(vec![]);
                    } else if command_parts[2] == ".." {
                        state.current_folder.0.pop();
                    } else {
                        state.current_folder.0.push(command_parts[2].to_string());
                    }
                }
                "ls" => loop {
                    let next = lines.peek();
                    match next {
                        None => break,
                        Some(x) if x.starts_with("$") => break,
                        Some(line) => {
                            drop(next);
                            let line_parts = line.split_whitespace().collect_vec();
                            if line_parts[0].starts_with("dir") {
                                let folder_name = line_parts[1];
                                let mut new_path = Path(state.current_folder.0.clone());
                                new_path.0.push(folder_name.to_string());
                                let folder = Folder::new(folder_name.to_string());

                                state
                                    .file_system
                                    .get_folder_mut(&state.current_folder)
                                    .unwrap()
                                    .add_folder(folder);
                            } else {
                                let size: usize = line_parts[0].parse().unwrap();
                                let name = line_parts[1].to_string();
                                let file = File { size, name };

                                state
                                    .file_system
                                    .get_folder_mut(&state.current_folder)
                                    .unwrap()
                                    .add_file(file);
                            }
                        }
                    }

                    lines.next().unwrap();
                },
                _ => {
                    panic!("Unknown command: {}", command_parts[1]);
                }
            }
        } else {
            panic!("Unknown line: {}", line);
        }
    }
}

fn sum_subfolder_sizes(folder: &Folder, size: &mut usize) {
    let folder_size = folder.get_size();

    if folder_size <= 100_000 {
        *size += folder_size;
    }

    for subfolder in folder.folders.values() {
        sum_subfolder_sizes(subfolder, size);
    }
}

pub fn day7a() {
    let lines = INPUT.lines().collect_vec();
    let mut state = State::new();
    execute_lines(&lines, &mut state);

    let mut size = 0;
    sum_subfolder_sizes(&state.file_system, &mut size);

    println!("Day 7a: {:#?}", size);
}

fn find_folder_to_remove<'a, 'b>(
    folder: &'a Folder,
    min_size: usize,
    candidates: &'b mut Vec<(usize, &'a Folder)>,
) {
    let folder_size = folder.get_size();

    if folder_size > min_size {
        candidates.push((folder_size, folder));
    }

    for subfolder in folder.folders.values() {
        find_folder_to_remove(subfolder, min_size, candidates);
    }
}

pub fn day7b() {
    let lines = INPUT.lines().collect_vec();
    let mut state = State::new();
    execute_lines(&lines, &mut state);

    let total_size = state.file_system.get_size();
    let free_space = 70000000 - total_size;
    let min_delete_size = 30000000 - free_space;

    let mut candidates = vec![];
    find_folder_to_remove(&state.file_system, min_delete_size, &mut candidates);

    let smallest_candidate = candidates
        .iter()
        .min_by_key(|(size, _)| *size)
        .unwrap()
        .clone();

    println!("Day 7b: {:#?}", smallest_candidate.0);
}
