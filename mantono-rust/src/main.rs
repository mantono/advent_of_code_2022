mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let answer: String = aoc::run::<day6::Aoc>().unwrap();
    println!("Answer: {}", answer);
}
