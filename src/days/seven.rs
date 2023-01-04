use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use anyhow::{anyhow, bail, Context};

use super::Day;

pub const DAY_SEVEN: Day = Day { part_one, part_two };

#[derive(Debug)]
enum Token {
    CD(String),
    LS,
    Dir(String),
    File(File),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::CD(dir) => write!(f, "cd ({})", dir),
            Token::LS => write!(f, "ls"),
            Token::Dir(dir) => write!(f, "dir ({})", dir),
            Token::File(File { size, name }) => write!(f, "file, size ({}) name ({})", size, name),
        }
    }
}

impl FromStr for Token {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let first_word = words.next();
        let second_word = words.next();
        let third_word = words.next();

        if words.next().is_some() {
            bail!("({}) has too many parts to be a Token", s);
        }

        match (first_word, second_word, third_word) {
            (Some("$"), Some("cd"), Some(dir)) => Ok(Token::CD(dir.to_string())),
            (Some("$"), Some("ls"), None) => Ok(Token::LS),
            (Some("dir"), Some(dir), None) => Ok(Token::Dir(dir.to_string())),
            (Some(size), Some(name), None) => Ok(Token::File(File {
                size: size
                    .parse()
                    .with_context(|| format!("{} is not a valid size", size))?,

                name: name.to_string(),
            })),
            (_, _, _) => Err(anyhow!("{} is not a valid token", s)),
        }
    }
}

#[derive(Debug)]
struct File {
    size: usize,
    name: String,
}

#[derive(Debug, Default)]
struct Directory {
    child_dirs: HashSet<String>,
    files: Vec<File>,
}

struct FileSystem {
    dirs: HashMap<String, Directory>,
    current_path: Vec<String>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            dirs: HashMap::new(),
            current_path: vec!["/".to_string()],
        }
    }

    pub fn cd(&mut self, dir: &str) {
        if dir == "/" {
            self.current_path = vec!["/".to_string()];
        } else if dir == ".." {
            self.current_path.pop();
        } else {
            self.current_path.push(dir.to_string());
        }
    }

    pub fn make_dir_if_needed(&mut self, dir: &str) {
        let parent_path = self.current_path();
        let parent_dir = self.dirs.entry(parent_path).or_default();
        parent_dir.child_dirs.insert(dir.to_string());
        if !self.dirs.contains_key(dir) {
            self.dirs.insert(dir.to_string(), Default::default());
        }
    }

    pub fn current_path(&self) -> String {
        self.current_path.join("/")
    }

    pub fn add_file(&mut self, file: File) {
        let path = self.current_path();
        let dir = self.dirs.entry(path).or_default();
        dir.files.push(file);
    }

    pub fn process_token(&mut self, token: Token) {
        match token {
            Token::CD(dir) => self.cd(&dir),
            Token::LS => (),
            Token::Dir(dir) => self.make_dir_if_needed(&dir),
            Token::File(file) => self.add_file(file),
        }
    }

    pub fn total_size(&self, dir_path: &str) -> usize {
        let dir = self.dirs.get(dir_path).unwrap();
        dir.files.iter().map(|f| f.size).sum::<usize>()
            + dir
                .child_dirs
                .iter()
                .map(|child| self.total_size(&format!("{}/{}", dir_path, child)))
                .sum::<usize>()
    }
}

impl FromStr for FileSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fs = FileSystem::new();
        for line in s.lines() {
            let token = line.parse::<Token>()?;
            fs.process_token(token);
        }
        Ok(fs)
    }
}

fn part_one(input: &str) -> String {
    let fs = input.parse::<FileSystem>().unwrap();

    let mut to_visit = vec!["/".to_string()];
    let mut answer = 0;
    while !to_visit.is_empty() {
        let dir_path = to_visit.pop().unwrap();
        let dir = fs.dirs.get(&dir_path).unwrap();
        for child in dir.child_dirs.iter() {
            to_visit.push(format!("{}/{}", dir_path, child));
        }
        let dir_size = fs.total_size(&dir_path);
        if dir_size <= 100000 {
            answer += dir_size;
        }
    }
    answer.to_string()
}

fn part_two(input: &str) -> String {
    let fs = input.parse::<FileSystem>().unwrap();

    let mut to_visit = vec!["/".to_string()];
    let mut candidates = Vec::new();

    let used_space = fs.total_size("/");
    let free_space = 70000000 - used_space;
    let need_to_free = 30000000 - free_space;

    while !to_visit.is_empty() {
        let dir_path = to_visit.pop().unwrap();
        let dir = fs.dirs.get(&dir_path).unwrap();
        for child in dir.child_dirs.iter() {
            to_visit.push(format!("{}/{}", dir_path, child));
        }
        let dir_size = fs.total_size(&dir_path);
        if dir_size >= need_to_free {
            candidates.push(dir_size);
        }
    }
    candidates.into_iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_part_one_example() {
        let example = include_str!("../../inputs/7/1/example.txt");
        let answer = part_one(example);
        assert_eq!(answer, "95437");
    }

    #[test]
    fn test_part_one_input() {
        let example = include_str!("../../inputs/7/1/input.txt");
        let answer = part_one(example);
        assert_eq!(answer, "1443806");
    }

    #[test]
    fn test_part_two_example() {
        let example = include_str!("../../inputs/7/2/example.txt");
        let answer = part_two(example);
        assert_eq!(answer, "24933642");
    }

    #[test]
    fn test_part_two_input() {
        let example = include_str!("../../inputs/7/2/input.txt");
        let answer = part_two(example);
        assert_eq!(answer, "942298");
    }
}
