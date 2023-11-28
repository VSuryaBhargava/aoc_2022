use utils::{
    nom::combinator::{all_consuming, map},
    parser::{parse_number_i64, ParseInput},
};

pub type ParsedOutput = (Vec<NumericalMessage>, i64);

#[derive(Default)]
pub struct Problem {
    pub number_of_mixes: u8,
    pub multiplication_factor: i64,
}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, is_part2: bool) -> ParsedOutput {
        let mut data_arr: Vec<NumericalMessage> = Vec::new();
        let mut index_of_0 = 0;

        self.number_of_mixes = if is_part2 { 10 } else { 1 };
        self.multiplication_factor = if is_part2 { 811589153 } else { 1 };

        input.lines().enumerate().for_each(|(index, l)| {
            let _ = map(all_consuming(parse_number_i64), |data| {
                data_arr.push(NumericalMessage {
                    data,
                    index: index as i64,
                });
                if data == 0 {
                    index_of_0 = index as i64;
                }
            })(l);
        });

        (data_arr, index_of_0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumericalMessage {
    pub data: i64,
    pub index: i64,
}
