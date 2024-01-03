use utils::problem::SolvePart1;

use crate::parser::{ParsedOutput, Problem, SNAFUTrait};

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, parsed_input: ParsedOutput) -> String {
        parsed_input.iter().sum::<i128>().to_snafu()
    }
}
