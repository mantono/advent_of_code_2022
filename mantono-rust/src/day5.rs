use std::{
    collections::VecDeque,
    fmt::{Display, Write},
    str::FromStr,
};

use aoc::Solver;

pub struct Aoc;

impl Solver for Aoc {
    type Item = String;

    fn solve_one(inputs: &[Self::Item]) -> String {
        let mut stacks: [Vec<char>; 9] = Default::default();
        for line in inputs {
            let row: Row = match line.parse() {
                Ok(r) => r,
                Err(_) => break,
            };
            for s in 1..=9 {
                if let Some(container) = row.stack(s) {
                    stacks[s - 1].insert(0, container);
                }
            }
        }
        println!("{:?}", stacks);
        let mut stacks = Stacks(stacks);

        inputs
            .into_iter()
            .skip(10)
            .map(|line| line.parse::<Move>().unwrap())
            .for_each(|mve| {
                stacks.apply(mve, false);
            });

        stacks.to_string()
    }

    fn solve_two(inputs: &[Self::Item]) -> String {
        let mut stacks: [Vec<char>; 9] = Default::default();
        for line in inputs {
            let row: Row = match line.parse() {
                Ok(r) => r,
                Err(_) => break,
            };
            for s in 1..=9 {
                if let Some(container) = row.stack(s) {
                    stacks[s - 1].insert(0, container);
                }
            }
        }
        println!("{:?}", stacks);
        let mut stacks = Stacks(stacks);

        inputs
            .into_iter()
            .skip(10)
            .map(|line| line.parse::<Move>().unwrap())
            .for_each(|mve| {
                stacks.apply(mve, true);
            });

        stacks.to_string()
    }

    fn puzzle() -> aoc::Puzzle {
        aoc::Puzzle::new(2022, 5)
    }

    fn parse_input<T: Into<String>>(input: T) -> Vec<Self::Item> {
        input
            .into()
            .lines()
            .map(|line| match Self::parse(line) {
                Ok(val) => val,
                Err(_) => panic!("Unable to parse line: '{}'", line),
            })
            .collect()
    }
}

struct Stacks(pub [Vec<char>; 9]);

impl Stacks {
    pub fn apply(&mut self, mve: Move, multi: bool) {
        if multi {
            let mut buffer: Vec<char> = Vec::with_capacity(12);
            for _ in 0..mve.num {
                let stack_from = self.0.get_mut(mve.from as usize - 1).unwrap();
                let container: char = stack_from.pop().expect("No element found");
                buffer.push(container);
            }
            let stack_to = self.0.get_mut(mve.to as usize - 1).unwrap();
            buffer.reverse();
            for c in buffer {
                stack_to.push(c);
            }
        } else {
            for _ in 0..mve.num {
                let stack_from = self.0.get_mut(mve.from as usize - 1).unwrap();
                let container: char = stack_from.pop().expect("No element found");
                let stack_to = self.0.get_mut(mve.to as usize - 1).unwrap();
                stack_to.push(container);
            }
        }
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..9 {
            let e: &char = self.0.get(i).unwrap().last().unwrap();
            f.write_char(*e)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Row(String);

const STACK_OFFSET: usize = 3;
const STACK_WIDTH: usize = 4;

impl Row {
    pub fn stack(&self, n: usize) -> Option<char> {
        match n {
            1..=9 => {
                let idx: usize = (n * STACK_WIDTH) - STACK_OFFSET;
                self.0
                    .get(idx..(idx + 1))
                    .map(|c| c.chars().next())
                    .flatten()
                    .filter(|c| c.is_ascii_alphanumeric())
            }
            _ => panic!("Invalid stack: {}", n),
        }
    }
}

impl FromStr for Row {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|c| c.is_ascii_digit()) {
            Err(String::from("Invalid row"))
        } else {
            Ok(Row(s.to_string()))
        }
    }
}

struct Move {
    pub num: u8,
    pub from: u8,
    pub to: u8,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        parts.next();
        let num: u8 = parts.next().unwrap().parse().unwrap();
        parts.next();
        let from: u8 = parts.next().unwrap().parse().unwrap();
        parts.next();
        let to: u8 = parts.next().unwrap().parse().unwrap();
        let mve: Move = Move { num, from, to };
        Ok(mve)
    }
}
