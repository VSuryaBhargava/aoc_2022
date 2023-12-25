use utils::problem::SolvePart2;

use crate::{parser::ParsedOutput, Problem};

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, _parsed_input: ParsedOutput) -> String {
        "Implement part two".to_string()
    }
}
