mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let answer: String = aoc::run::<day7::Aoc>().unwrap();
    println!("Answer: {}", answer);
}
