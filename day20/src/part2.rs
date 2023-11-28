
use crate::{parser::ParsedOutput, Problem};
use utils::problem::{SolvePart1, SolvePart2};

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, parsed_input: ParsedOutput) -> String {
        self.solve_part_one(parsed_input)
    }
}
