pub trait SolvePart1 {
    type ParsedType;
    fn solve_part_one(&mut self, parsed_input: Self::ParsedType) -> String;
}

pub trait SolvePart2 {
    type ParsedType;
    fn solve_part_two(&mut self, parsed_input: Self::ParsedType) -> String;
}
