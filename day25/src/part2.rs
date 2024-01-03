use utils::problem::SolvePart2;

use crate::{parser::ParsedOutput, Problem};

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, _: ParsedOutput) -> String {
        "No part 2!\n".to_string()
    }
}
