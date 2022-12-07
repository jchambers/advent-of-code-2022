use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let root = parse_terminal_output(BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok()))
            .unwrap();

        let mut directories = vec![];
        root.find_directories(&mut directories);
        let directories = directories;

        {
            let sum_of_sizes: usize = directories.iter()
                .map(|directory| directory.size())
                .filter(|&size| size < 100_000)
                .sum();

            println!("Sum of sizes of directories with individual size under 100,000: {}", sum_of_sizes);
        }

        {
            let disk_size = 70_000_000;
            let required_space = 30_000_000;
            let available_space = disk_size - root.size();
            let min_space_to_free = required_space - available_space;

            assert!(available_space < required_space);

            let smallest_directory_to_delete = directories.iter()
                .map(|directory| directory.size())
                .filter(|&size| size > min_space_to_free)
                .min()
                .unwrap();

            println!("Smallest directory to delete to free required space: {}", smallest_directory_to_delete);
        }

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

fn parse_terminal_output(lines: impl Iterator<Item = String>) -> Result<Directory, Box<dyn Error>> {
    let mut path = vec![];
    let mut root = Directory::new("/");

    for line in lines {
        if let Ok(command) = Command::from_str(line.as_str()) {
            match command {
                Command::ChangeDirectoryRoot => {
                    path.clear();
                    path.push("/".to_string());
                }
                Command::ChangeDirectoryPush(directory) => {
                    path.push(directory)
                },
                Command::ChangeDirectoryPop => {
                    path.pop();
                },
                Command::ListDirectory => {}
            };
        } else if let Ok(entry) = FileSystemEntry::from_str(line.as_str()) {
            root.add(path.iter().map(|segment| segment.as_str()).collect::<Vec<&str>>().as_slice(), entry);
        } else {
            return Err("Could not parse line".into());
        }
    }

    Ok(root)
}

#[derive(Debug)]
struct Directory {
    name: String,
    contents: Vec<FileSystemEntry>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            contents: vec![],
        }
    }

    fn size(&self) -> usize {
        self.contents
            .iter()
            .map(FileSystemEntry::size)
            .sum()
    }

    fn add(&mut self, path: &[&str], entry: FileSystemEntry) {
        if path.len() == 1 {
            self.contents.push(entry);
        } else {
            self.contents
                .iter_mut()
                .find_map(|entry| {
                    if let FileSystemEntry::Directory(subdirectory) = entry {
                        if subdirectory.name == path[1] {
                            Some(subdirectory)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .unwrap()
                .add(&path[1..], entry);
        }
    }

    fn find_directories<'a>(&'a self, directories: &mut Vec<&'a Directory>) {
        directories.push(self);

        self.contents.iter()
            .for_each(|entry| {
                if let FileSystemEntry::Directory(directory) = entry {
                    directory.find_directories(directories);
                }
            });
    }
}

#[derive(Debug)]
enum FileSystemEntry {
    Directory(Directory),
    File(String, usize),
}

impl FromStr for FileSystemEntry {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.split(' ').collect::<Vec<&str>>().as_slice() {
            ["dir", directory_name] => Ok(FileSystemEntry::Directory(Directory::new(directory_name))),
            [size, filename] => Ok(FileSystemEntry::File(filename.to_string(), size.parse()?)),
            _ => Err("Unrecognized filesystem entry".into())
        }
    }
}

impl FileSystemEntry {
    fn size(&self) -> usize {
        match self {
            FileSystemEntry::File(_, size) => *size,
            FileSystemEntry::Directory(directory) => directory.size(),
        }
    }
}

enum Command {
    ChangeDirectoryRoot,
    ChangeDirectoryPush(String),
    ChangeDirectoryPop,
    ListDirectory,
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.split(' ').collect::<Vec<&str>>().as_slice() {
            ["$", "cd", "/"] => Ok(Command::ChangeDirectoryRoot),
            ["$", "cd", ".."] => Ok(Command::ChangeDirectoryPop),
            ["$", "cd", directory] => Ok(Command::ChangeDirectoryPush(directory.to_string())),
            ["$", "ls"] => Ok(Command::ListDirectory),
            _ => Err("Unrecognized command".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const TEST_LINES: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn test_sum_of_sizes() {
        let root = parse_terminal_output(TEST_LINES.lines().map(|line| line.to_string())).unwrap();

        let mut directories = vec![];
        root.find_directories(&mut directories);

        let sum_of_sizes: usize = directories.iter()
            .map(|directory| directory.size())
            .filter(|&size| size < 100_000)
            .sum();

        assert_eq!(95437, sum_of_sizes);
    }
}
