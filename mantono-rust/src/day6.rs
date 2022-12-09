use aoc::Solver;

pub struct Aoc;

impl Aoc {
    fn solve_any<const N: usize>(inputs: &[char], mut stack: DropStack<N>) -> String {
        let mut counter: usize = 0;
        for c in inputs {
            stack.push(*c);
            counter += 1;

            if stack.distinct() {
                break;
            }
        }

        counter.to_string()
    }
}

impl Solver for Aoc {
    type Item = char;

    fn solve_one(inputs: &[Self::Item]) -> String {
        let stack: DropStack<4> = DropStack::new();
        Aoc::solve_any(inputs, stack)
    }

    fn puzzle() -> aoc::Puzzle {
        aoc::Puzzle::new(2022, 6)
    }

    fn solve_two(inputs: &[Self::Item]) -> String {
        let stack: DropStack<14> = DropStack::new();
        Aoc::solve_any(inputs, stack)
    }

    fn parse_input<T: Into<String>>(input: T) -> Vec<Self::Item> {
        input.into().chars().collect()
    }
}

struct DropStack<const N: usize> {
    data: [char; N],
    ptr: usize,
}

impl<const N: usize> DropStack<N> {
    fn new() -> DropStack<N> {
        DropStack {
            data: ['\0'; N],
            ptr: 0,
        }
    }

    fn distinct(&self) -> bool {
        let set: std::collections::HashSet<char> =
            self.data.iter().filter(|c| **c != '\0').copied().collect();
        set.len() == self.data.len()
    }

    fn push(&mut self, c: char) {
        self.data[self.index()] = c;
        self.ptr += 1;
    }

    fn index(&self) -> usize {
        self.ptr % self.data.len()
    }
}
