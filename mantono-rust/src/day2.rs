use std::str::FromStr;

use aoc::Solver;

pub struct Aoc;

impl Solver for Aoc {
    type Item = Strategy;

    fn solve_one(inputs: &[Round]) -> String {
        inputs
            .iter()
            .map(|round| (round.1, round.play()))
            .map(|(shape, outome)| score(shape, outome))
            .sum::<usize>()
            .to_string()
    }

    fn solve_two(inputs: &[Self::Item]) -> String {
        inputs
            .iter()
            .map(|strat| (strat.play(), strat.1))
            .map(|(shape, outome)| score(shape, outome))
            .sum::<usize>()
            .to_string()
    }
}

fn score(shape: Shape, outcome: Outcome) -> usize {
    shape.score() + outcome.score()
}

#[derive(Copy, Clone)]
pub struct Strategy(Shape, Outcome);

impl Strategy {
    fn play(&self) -> Shape {
        match (self.0, self.1) {
            (Shape::Rock, Outcome::Lost) => Shape::Scissors,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Won) => Shape::Paper,
            (Shape::Paper, Outcome::Lost) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Won) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lost) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Won) => Shape::Rock,
        }
    }
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let shape = tokens.next().expect("Could not find shape");
        let outcome = tokens.next().expect("Could not find outcome");
        let shape: Shape = shape.parse()?;
        let outcome: Outcome = outcome.parse()?;
        Ok(Strategy(shape, outcome))
    }
}

pub struct Round(Shape, Shape);

impl Round {
    fn play(&self) -> Outcome {
        match (&self.0, &self.1) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Won,
            (Shape::Rock, Shape::Scissors) => Outcome::Lost,
            (Shape::Paper, Shape::Rock) => Outcome::Lost,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Won,
            (Shape::Scissors, Shape::Rock) => Outcome::Won,
            (Shape::Scissors, Shape::Paper) => Outcome::Lost,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let left = tokens.next().expect("Could not find left shape");
        let right = tokens.next().expect("Could not find right shape");
        let left: Shape = left.parse()?;
        let right: Shape = right.parse()?;
        Ok(Round(left, right))
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lost,
    Draw,
    Won,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Won => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lost),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Won),
            _ => Err(format!("Invalid outcome {}", s)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(format!("Unrecognized input for shape '{}'", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc::Solver;

    use crate::day2;

    #[test]
    fn test_input() {
        let input = r#"
            A Y
            B X
            C Z
        "#;

        assert_eq!(String::from("15"), day2::Aoc::solve(input));
    }
}