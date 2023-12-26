use utils::problem::SolvePart2;

use crate::{parser::ParsedOutput, part1::solve, Problem};

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, parsed_input: ParsedOutput) -> String {
        let (_, rounds) = solve(self, parsed_input, usize::MAX);
        rounds.to_string()
    }
}
