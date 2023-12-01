use utils::{
    nom::{
        branch::alt,
        character::complete::char,
        combinator::{map, opt},
        multi::many1,
        IResult,
    },
    parser::{parse_number_u32, ParseInput},
};

#[derive(Debug)]
pub struct ParsedOutput {
    pub instructions: Vec<Instructions>,
    pub map: Vec<Vec<Points>>,
}

#[derive(Default)]
pub struct Problem {}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, _: bool) -> ParsedOutput {
        let mut parsed_input = ParsedOutput {
            instructions: Vec::new(),
            map: Vec::new(),
        };

        input.lines().for_each(|l| {
            let _ = map(parse_line, |(ins, points)| {
                if let Some(ins) = ins {
                    parsed_input.instructions = ins;
                }
                if let Some(points) = points {
                    parsed_input.map.push(points);
                }
            })(l);
        });

        parsed_input
    }
}

#[derive(Debug)]
pub enum Instructions {
    Move(u32),
    Right,
    Left,
}

#[derive(Debug)]
pub enum Points {
    OutOfBounds,
    Wall,
    Empty,
}

fn parse_map(input: &str) -> IResult<&str, Vec<Points>> {
    many1(alt((
        map(char(' '), |_| Points::OutOfBounds),
        map(char('.'), |_| Points::Empty),
        map(char('#'), |_| Points::Wall),
    )))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instructions>> {
    many1(alt((
        map(parse_number_u32, Instructions::Move),
        map(char('L'), |_| Instructions::Left),
        map(char('R'), |_| Instructions::Right),
    )))(input)
}

fn parse_line(input: &str) -> IResult<&str, (Option<Vec<Instructions>>, Option<Vec<Points>>)> {
    let (input, instructions) = opt(parse_instructions)(input)?;
    if let Some(instructions) = instructions {
        return Ok((input, (Some(instructions), None)));
    }

    let (input, points) = opt(parse_map)(input)?;
    if let Some(points) = points {
        return Ok((input, (None, Some(points))));
    }

    Ok((input, (None, None)))
}
