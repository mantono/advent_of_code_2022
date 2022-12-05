mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let answer: String = aoc::run::<day5::Aoc>().unwrap();
    println!("Answer: {}", answer);
}
