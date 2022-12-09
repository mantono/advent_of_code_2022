use std::{collections::LinkedList, str::FromStr};

use aoc::{Puzzle, Solver};

pub struct Aoc;

impl Solver for Aoc {
    type Item = Line;

    fn solve_one(inputs: &[Self::Item]) -> String {
        let mut nav = Nav::new();
        for line in inputs {
            nav.process(line.clone());
            println!("Line: {:?}", line);
        }

        format!("{:?}", nav)
    }

    fn puzzle() -> aoc::Puzzle {
        Puzzle::new(2022, 7)
    }
}

#[derive(Debug)]
struct Nav {
    path: LinkedList<String>,
    files: Vec<(String, usize)>,
}

impl Nav {
    fn new() -> Nav {
        Nav {
            path: LinkedList::new(),
            files: Vec::with_capacity(512),
        }
    }

    fn process(&mut self, cmd: Line) {
        match cmd {
            Line::Cd(dir) => match dir.as_str() {
                "/" => self.path.clear(),
                ".." => {
                    self.path.pop_front();
                }
                _ => self.path.push_front(dir),
            },
            Line::Entry(size, file) => {
                let mut full_path: String =
                    self.path.iter().cloned().collect::<Vec<String>>().join("/");
                full_path.push_str(&file);
                self.files.push((full_path, size));
            }
            _ => (),
        }
    }
}

// It's a UNIX system.
// I know this.
#[derive(Debug, Clone)]
pub enum Line {
    Cd(String),
    Ls,
    Dir,
    Entry(usize, String),
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..3] {
            "dir" => Ok(Line::Dir),
            "$ l" => Ok(Line::Ls),
            "$ c" => Ok(Line::Cd(s[5..].to_string())),
            _ => {
                let mut parts = dbg!(s).split_ascii_whitespace();
                let size: usize = parts.next().expect("File size").parse().expect("Number");
                let name: String = parts.next().expect("File name").to_string();
                Ok(Line::Entry(size, name))
            }
        }
    }
}

struct File {
    path: LinkedList<String>,
    size: usize,
}

impl File {
    fn new(path: LinkedList<String>, size: usize) -> File {
        File { path, size }
    }

    fn parent(&self) -> Dir {
        let mut parent_path = self.path.clone();
        parent_path.pop_front();
        Dir::new(parent_path)
    }
}

struct Dir {
    path: LinkedList<String>,
}

impl Dir {
    fn new(path: LinkedList<String>) -> Dir {
        Dir { path }
    }

    fn parent(&self) -> Option<Dir> {
        match self.path.len() {
            0 => panic!("Invalid state"),
            1 => None,
            _ => {
                let mut parent_path = self.path.clone();
                parent_path.pop_front();
                Some(Dir::new(parent_path))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc::Solver;

    #[test]
    fn test_part1() {
        let input = r#"
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
        assert_eq!("95437", crate::day7::Aoc::solve(input));
    }
}
