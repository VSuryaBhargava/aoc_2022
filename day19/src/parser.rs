use utils::{
    nom::combinator::{all_consuming, map},
    parser::{parse_number_i32, ParseInput},
};

pub type ParsedOutput = Vec<i32>;

#[derive(Default)]
pub struct Problem {}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, _: bool) -> ParsedOutput {
        let mut parsed_input: Vec<i32> = vec![];

        input.lines().for_each(|l| {
            let _ = map(all_consuming(parse_number_i32), |data| {
                parsed_input.push(data);
            })(l);
        });

        parsed_input
    }
}
