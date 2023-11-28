use utils::parser::ParseInput;

pub type ParsedOutput = Vec<u8>;

#[derive(Default)]
pub struct Problem {
    pub total_number_of_rocks: u64,
}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, part2: bool) -> ParsedOutput {
        self.total_number_of_rocks = if part2 { 1000000000000 } else { 2022 };
        input.as_bytes().to_vec()
    }
}
