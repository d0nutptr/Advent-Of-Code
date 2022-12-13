use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use crate::solutions::Day;
use crate::solutions::AdventSolution;

impl AdventSolution for Day<7> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        let file_system = FileSystem::from(input);

        let total_sum = file_system.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| matches!(entry, Entry::Directory(_)))
            .filter_map(|(idx, _)| file_system.get_size_if_known(idx))
            .filter(|size| *size <= 100_000)
            .sum();

        total_sum
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        const MAX_DISK_SPACE: u64 = 70_000_000;
        const NEEDED_DISK_SPACE: u64 = 30_000_000;

        let file_system = FileSystem::from(input);

        let root_size = file_system
            .get_size_if_known(0)
            .expect("Missing root");
        let needed_space = NEEDED_DISK_SPACE - (MAX_DISK_SPACE - root_size);

        let ordered_directory_sizes = file_system.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| matches!(entry, Entry::Directory(_)))
            .filter_map(|(idx, _)| file_system.get_size_if_known(idx))
            .map(|elem| Reverse(elem))
            .collect::<BinaryHeap<_>>();

        ordered_directory_sizes
            .into_iter_sorted()
            .find(|Reverse(size)| *size >= needed_space)
            .map(|Reverse(size)| size)
            .unwrap()
    }
}

#[derive(Debug)]
struct FileSystem {
    // root is at index 0
    entries: Vec<Entry>
}

impl FileSystem {
    fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }

    fn create_directory_if_not_exist(&mut self, parent: impl Into<Option<usize>>, name: &str) -> usize {
        let new_directory_index = self.entries.len();
        let parent_idx = parent.into();

        // update the parent with a reference to our new directory
        if let Some(parent_idx) = &parent_idx {
            // check if it exists and return index or create it
            let Entry::Directory(Directory {
                children,
                ..
             }) = self.entries
                .get_mut(*parent_idx)
                .expect("Incorrect parent directory index") else {
                panic!("Parent was not a directory");
            };

            if let Some(existing_directory_index) = children.get(name) {
                return *existing_directory_index;
            }

            children.insert(name.to_string(), new_directory_index);
        }

        // create the new directory and add it
        let directory = Directory {
            parent: parent_idx,
            name: name.to_string(),
            children: Default::default(),
            size_hint: None,
        };

        self.entries.push(Entry::Directory(directory));

        new_directory_index
    }

    fn create_file_if_not_exist(&mut self, directory: usize, name: &str, size: u64) -> usize {
        let new_file_index = self.entries.len();

        let Entry::Directory(Directory {
             children,
             ..
         }) = self.entries
            .get_mut(directory)
            .expect("Incorrect parent directory index") else {
            panic!("Parent was not a directory");
        };

        // if file exists, return index
        if let Some(existing_file_index) = children.get(name) {
            return *existing_file_index;
        }

        children.insert(name.to_string(), new_file_index);

        let file = File {
            name: name.to_string(),
            size
        };

        self.entries.push(Entry::File(file));

        new_file_index
    }

    fn get_directory(&self, directory_idx: usize) -> Option<&Directory> {
        match self.entries.get(directory_idx) {
            Some(Entry::Directory(dir)) => Some(dir),
            _ => None
        }
    }

    fn get_directory_mut(&mut self, directory_idx: usize) -> Option<&mut Directory> {
        match self.entries.get_mut(directory_idx) {
            Some(Entry::Directory(dir)) => Some(dir),
            _ => None
        }
    }

    fn get_size_if_known(&self, entry_index: usize) -> Option<u64> {
        match self.entries.get(entry_index)? {
            Entry::Directory(Directory { size_hint: Some(size), .. }) => Some(*size),
            Entry::Directory(_) => None, // expensive to recurse
            Entry::File(file) => Some(file.size),
        }
    }

    fn calculate_size(&mut self, entry_index: usize) -> Option<u64> {
        match self.entries.get(entry_index)? {
            Entry::Directory(Directory { size_hint: Some(size), .. }) => *size,
            Entry::Directory(directory) => {
                let children = directory.children
                    .values()
                    .map(|entry| *entry)
                    .collect::<Vec<_>>();

                let size = children
                    .into_iter()
                    .filter_map(|idx| self.calculate_size(idx))
                    .sum();

                self.get_directory_mut(entry_index)
                    .unwrap()
                    .size_hint = Some(size);

                size
            },
            Entry::File(file) => file.size,
        }.into()
    }
}

#[derive(Debug)]
enum Entry {
    Directory(Directory),
    File(File)
}

#[derive(Debug)]
struct Directory {
    #[allow(dead_code)]
    name: String,
    size_hint: Option<u64>,
    parent: Option<usize>,
    children: HashMap<String, usize>
}

#[derive(Debug)]
struct File {
    size: u64,
    #[allow(dead_code)]
    name: String,
}

impl From<&str> for FileSystem {
    fn from(value: &str) -> Self {
        let mut filesystem = FileSystem::new();
        let mut current_index = None;

        for line in value.lines() {
            match InputLine::from(line) {
                InputLine::ChangeDirectory("..") => {
                    current_index = filesystem.get_directory(current_index.unwrap())
                        .unwrap()
                        .parent;
                },
                InputLine::ChangeDirectory(name) => {
                    current_index = Some(filesystem.create_directory_if_not_exist(current_index, name));
                },
                InputLine::ListDirectory => { /* no-op */ },
                InputLine::FileOutput { size, name }
                    => { filesystem.create_file_if_not_exist(current_index.unwrap(), name, size); },
                InputLine::DirectoryOutput { name }
                    => { filesystem.create_directory_if_not_exist(current_index, name); },
            }
        }

        // precalculate the directory sizes
        filesystem.calculate_size(0);

        filesystem
    }
}

enum InputLine<'a> {
    ChangeDirectory(&'a str),
    ListDirectory,
    FileOutput {
        size: u64,
        name: &'a str
    },
    DirectoryOutput {
        name: &'a str,
    }
}

impl<'a> From<&'a str> for InputLine<'a> {
    fn from(value: &'a str) -> Self {
        let mut components = value.split(" ");

        match (components.next(), components.next()) {
            (Some("$"), Some("cd")) => InputLine::ChangeDirectory(components.next().expect("Missing directory name")),
            (Some("$"), Some("ls")) => InputLine::ListDirectory,
            (Some("dir"), Some(name)) => InputLine::DirectoryOutput {
                name
            },
            (Some(size), Some(name)) => InputLine::FileOutput {
                size: size.parse::<u64>().unwrap(),
                name
            },
            _ => unreachable!("Unparseable Line")
        }
    }
}