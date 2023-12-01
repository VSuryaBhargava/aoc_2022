use utils::problem::SolvePart1;

use crate::parser::{ParsedOutput, Problem};

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, parsed_input: ParsedOutput) -> String {
        println!("{:?}", parsed_input);
        todo!("Implement part one");
    }
}
