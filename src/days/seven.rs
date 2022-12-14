use anyhow::{anyhow, bail, Context};

use std::{
    cell::RefCell,
    collections::{HashMap},
    fmt::Display,
    rc::Rc,
    str::FromStr,
};

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
    subdirs: HashMap<String, Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    pub fn total_size(&self) -> usize {
        let my_file_size = self.files.iter().map(|f| f.size).sum::<usize>();
        let subdirs_size = self
            .subdirs
            .values()
            .map(|d| d.borrow().total_size())
            .sum::<usize>();
        return my_file_size + subdirs_size;
    }
}

#[derive(Debug)]
struct FileSystem {
    root: Rc<RefCell<Directory>>,
    current: Rc<RefCell<Directory>>,
}

impl FileSystem {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Directory::default()));
        let current = root.clone();

        Self { root, current }
    }

    pub fn cd(&mut self, dir: &str) {
        match dir {
            ".." => {
                let parent = self.current.borrow().parent.clone();
                self.current = parent.unwrap().clone();
            }
            "/" => self.current = self.root.clone(),
            new_current_dir => {
                let new_current = self.add_or_get_dir(new_current_dir);
                self.current = new_current;
            }
        }
    }

    pub fn add_or_get_dir(&mut self, dir: &str) -> Rc<RefCell<Directory>> {
        let mut current = self.current.borrow_mut();
        let entry = current.subdirs.entry(dir.to_string()).or_insert_with(|| {
            let new_dir: Directory = Directory {
                parent: Some(self.current.clone()),
                ..Default::default()
            };
            Rc::new(RefCell::new(new_dir))
        });

        entry.clone()
    }

    pub fn add_file(&mut self, file: File) {
        let mut current_dir = self.current.borrow_mut();
        current_dir.files.push(file);
    }

    pub fn process_token(&mut self, token: Token) {
        match token {
            Token::CD(dir) => self.cd(&dir),
            Token::LS => (),
            Token::Dir(dir) => {
                self.add_or_get_dir(&dir);
            }
            Token::File(file) => self.add_file(file),
        }
    }
}

fn part_one(input: &str) -> String {
    let mut fs = FileSystem::new();
    for token in input.lines().map(|line| line.parse::<Token>().unwrap()) {
        fs.process_token(token);
    }

    let root = fs.root.borrow();
    root.total_size().to_string()
}

fn part_two(_input: &str) -> String {
    todo!()
}
