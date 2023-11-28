pub use nom;
use nom::{
    bytes::complete::tag,
    combinator::{map_res, opt, recognize},
    sequence::preceded,
    IResult,
};
use std::fs;

pub trait ParseInput {
    type ParsedType;
    fn parse(&mut self, input: String, part2: bool) -> Self::ParsedType;
}

pub fn read_file(my_input: bool, day: u8) -> String {
    let day_str = if day < 10 {
        format!("0{}", day)
    } else {
        format!("{}", day)
    };
    let file_path = if my_input {
        format!("./day{}/input/input.txt", day_str)
    } else {
        format!("./day{}/input/sample-input.txt", day_str)
    };

    fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Failed to read file: {}", file_path))
}

pub fn parse_number_i16(input: &str) -> IResult<&str, i16> {
    map_res(
        recognize(preceded(opt(tag("-")), nom::character::complete::digit1)),
        |num_str: &str| num_str.parse::<i16>(),
    )(input)
}

pub fn parse_number_i32(input: &str) -> IResult<&str, i32> {
    map_res(
        recognize(preceded(opt(tag("-")), nom::character::complete::digit1)),
        |num_str: &str| num_str.parse::<i32>(),
    )(input)
}

pub fn parse_number_i64(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(preceded(opt(tag("-")), nom::character::complete::digit1)),
        |num_str: &str| num_str.parse::<i64>(),
    )(input)
}

pub fn parse_number_i128(input: &str) -> IResult<&str, i128> {
    map_res(
        recognize(preceded(opt(tag("-")), nom::character::complete::digit1)),
        |num_str: &str| num_str.parse::<i128>(),
    )(input)
}

pub fn parse_number_u8(input: &str) -> IResult<&str, u8> {
    map_res(nom::character::complete::digit1, |num_str: &str| {
        num_str.parse::<u8>()
    })(input)
}

pub fn parse_number_u32(input: &str) -> IResult<&str, u32> {
    map_res(nom::character::complete::digit1, |num_str: &str| {
        num_str.parse::<u32>()
    })(input)
}
