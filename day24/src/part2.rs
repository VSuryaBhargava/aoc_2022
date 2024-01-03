use utils::problem::{SolvePart1, SolvePart2};

use crate::{parser::ParsedOutput, Problem};

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, parsed_input: ParsedOutput) -> String {
        let start_pos = parsed_input.start_pos;
        let end_pos = parsed_input.end_pos;

        let start_to_end = self
            .solve_part_one(ParsedOutput {
                end_pos,
                start_pos,
                grid: parsed_input.grid,
            })
            .parse::<u32>()
            .unwrap();

        println!("start_to_end: {}", start_to_end);

        self.start_min_elapsed = start_to_end;
        let end_to_start = self
            .solve_part_one(ParsedOutput {
                end_pos: start_pos,
                start_pos: end_pos,
                grid: self.get_grid(self.start_min_elapsed).unwrap().clone(),
            })
            .parse::<u32>()
            .unwrap();

        println!("end_to_start: {}", end_to_start - start_to_end);

        self.start_min_elapsed = end_to_start;
        let start_to_end2 = self
            .solve_part_one(ParsedOutput {
                end_pos,
                start_pos,
                grid: self.get_grid(self.start_min_elapsed).unwrap().clone(),
            })
            .parse::<u32>()
            .unwrap();

        println!("start_to_end2: {}", start_to_end2 - end_to_start);
        (start_to_end2).to_string()
    }
}
