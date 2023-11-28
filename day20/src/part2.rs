use utils::problem::SolvePart2;

use crate::{parser::ParsedOutput, Problem};

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, parsed_input: ParsedOutput) -> String {
        crate::part1::solve_part_one(self, parsed_input)
    }
}
