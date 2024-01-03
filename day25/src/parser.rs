use utils::{nom::AsChar, parser::ParseInput};

pub type ParsedOutput = Vec<i128>;

#[derive(Debug)]
pub struct SNAFUParseError {}

pub trait SNAFUTrait {
    fn from_snafu(str: &str) -> Result<Self, SNAFUParseError>
    where
        Self: Sized;

    fn to_snafu(&self) -> String;
}

impl SNAFUTrait for i128 {
    fn from_snafu(s: &str) -> Result<Self, SNAFUParseError> {
        s.bytes().rev().enumerate().try_fold(0, |value, (i, c)| {
            if c == b'0' {
                return Ok(value);
            }

            let char_val: i128 = match c {
                b'1' => 1,
                b'2' => 2,
                b'-' => -1,
                b'=' => -2,
                _ => {
                    println!(
                        "Error parsing: {}\nUnable failed at char {}",
                        s,
                        c.as_char()
                    );
                    return Result::Err(SNAFUParseError {});
                }
            };

            Ok(value + (char_val * 5_i128.pow(i as u32)))
        })
    }
    fn to_snafu(&self) -> String {
        let mut value = *self;
        let mut result = String::new();

        // println!("Convert {} to SNAFU", value);

        while value != 0 {
            let digit = (value) % 5;
            value /= 5;

            let digit_char = match digit {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => {
                    value += 1;
                    '='
                }
                4 => {
                    value += 1;
                    '-'
                }
                _ => panic!("Invalid digit: {}", digit),
            };

            // println!(
            //     "Digit: {}, remaining: {}, char {}, carry {}",
            //     digit, value, digit_char, carry_over
            // );

            result.push(digit_char);
        }

        result.chars().rev().collect()
    }
}

#[derive(Default)]
pub struct Problem {}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, _: bool) -> ParsedOutput {
        let mut parsed_input: Vec<i128> = vec![];

        input
            .lines()
            .for_each(|l| parsed_input.push(i128::from_snafu(l).unwrap()));

        parsed_input
    }
}
