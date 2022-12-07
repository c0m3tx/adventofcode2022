use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");
const TOTAL_SIZE: usize = 70000000;
const MIN_SIZE_FOR_UPDATE: usize = 30000000;

struct Terminal {
    current_folder: String,
    folders: HashMap<String, HashMap<String, usize>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            current_folder: "/".to_string(),
            folders: HashMap::new(),
        }
    }

    fn cd(&mut self, folder: &str) {
        let folder = folder.to_string();

        if folder == ".." {
            self.current_folder =
                parent_folder(&self.current_folder).expect("Can't go up from root");
        } else if folder == "/" {
            self.current_folder = "/".into();
        } else {
            let mut splitted_folders: Vec<&str> = self
                .current_folder
                .split("/")
                .filter(|x| !x.is_empty())
                .collect();
            splitted_folders.push(&folder);
            self.current_folder = format!("/{}", splitted_folders.join("/"));
        }
    }

    fn touch(&mut self, name: impl ToString, size: usize) {
        self.folders
            .entry(self.current_folder.clone())
            .or_default()
            .insert(name.to_string(), size);
    }

    fn mkdir(&mut self, name: impl ToString) {
        let name = name.to_string();
        let mut splitted_folders: Vec<&str> = self
            .current_folder
            .split("/")
            .filter(|x| !x.is_empty())
            .collect();
        splitted_folders.push(&name);
        let folder = format!("/{}", splitted_folders.join("/"));
        self.folders.entry(folder).or_default();
    }

    fn analyse_folder_sizes(&self) -> HashMap<String, usize> {
        let mut folder_sizes: HashMap<String, usize> = HashMap::new();

        for folder in self.folders.keys() {
            let inner_folders = self.folders.keys().filter(|x| x.starts_with(folder));
            let size = inner_folders
                .map(|fold| {
                    self.folders
                        .get(fold)
                        .map(|files| files.values().sum())
                        .unwrap_or(0)
                })
                .sum();

            folder_sizes.insert(folder.clone(), size);
        }

        folder_sizes
    }
}

fn parent_folder(folder: &str) -> Option<String> {
    if folder == "/" {
        None
    } else {
        let mut splitted_folders: Vec<&str> = folder.split("/").filter(|x| !x.is_empty()).collect();
        splitted_folders.pop();
        let output = format!("/{}", splitted_folders.join("/"));
        Some(output)
    }
}

fn parse_input(input: &str) -> Terminal {
    let mut terminal = Terminal::new();

    for line in input.lines() {
        if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("$ cd") {
            let folder = line.split_at(4).1.trim();
            terminal.cd(folder);
        } else {
            let (part_1, part_2) = line.split_once(" ").expect("Invalid command");
            if part_1 == "dir" {
                terminal.mkdir(part_2);
            } else {
                terminal.touch(part_2, part_1.parse().expect("Invalid file size"));
            }
        }
    }

    terminal
}

fn part_1(input: &str) -> usize {
    let terminal = parse_input(input);
    let folder_sizes = terminal.analyse_folder_sizes();

    folder_sizes
        .into_iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum()
}

fn part_2(input: &str) -> usize {
    let terminal = parse_input(input);
    let folder_sizes = terminal.analyse_folder_sizes();

    let current_free_memory = TOTAL_SIZE - folder_sizes.get("/").expect("No root drive").clone();
    let needed_space = MIN_SIZE_FOR_UPDATE - current_free_memory;
    let mut folder_sizes: Vec<usize> = folder_sizes.values().cloned().collect();
    folder_sizes.sort_unstable();

    folder_sizes
        .into_iter()
        .find(|size| *size >= needed_space)
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"$ cd /
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
"#;

    #[test]
    fn terminal_can_go_to_root() {
        let mut terminal = Terminal::new();
        terminal.current_folder = "/something".into();
        terminal.cd("/");

        assert_eq!(terminal.current_folder, "/")
    }

    #[test]
    fn terminal_can_change_directory() {
        let mut terminal = Terminal::new();
        terminal.cd("something");

        assert_eq!(terminal.current_folder, "/something")
    }

    #[test]
    fn terminal_can_change_directory_multiple_times() {
        let mut terminal = Terminal::new();
        terminal.cd("some");
        terminal.cd("inner");
        terminal.cd("path");

        assert_eq!(terminal.current_folder, "/some/inner/path");
    }

    #[test]
    fn terminal_can_go_one_folder_up() {
        let mut terminal = Terminal::new();
        terminal.current_folder = "/some/inner/path".into();

        terminal.cd("..");
        assert_eq!(terminal.current_folder, "/some/inner");
    }

    #[test]
    fn test_parse_input_changes_folders() {
        let terminal = parse_input(TEST_INPUT);
        assert_eq!(terminal.current_folder, "/d");
    }

    #[test]
    fn test_parse_input_saves_files() {
        let terminal = parse_input(TEST_INPUT);
        let folder = terminal.folders.get("/a/e").expect("folder not found");
        assert_eq!(folder.len(), 1);
        let (name, size) = folder.iter().next().expect("file not found");
        assert_eq!(name, "i");
        assert_eq!(*size, 584);
    }

    #[test]
    fn test_calculate_folder_sizes() {
        let terminal = parse_input(TEST_INPUT);

        let folder_sizes = terminal.analyse_folder_sizes();
        assert_eq!(folder_sizes.get("/a/e").unwrap(), &584);
        assert_eq!(folder_sizes.get("/a").unwrap(), &94853);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 24933642);
    }
}
