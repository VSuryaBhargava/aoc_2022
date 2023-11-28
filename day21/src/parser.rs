use std::{collections::HashMap};
use utils::{
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, space1},
        combinator::{all_consuming, map},
        sequence::tuple,
        IResult,
    },
    parser::{parse_number_i64, ParseInput},
};

pub type ParsedOutput = HashMap<MonkeyName, Monkey>;

pub struct Day21;

impl ParseInput for Day21 {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, _: bool) -> ParsedOutput {
        let mut parsed_input: HashMap<MonkeyName, Monkey> = HashMap::new();

        input.lines().for_each(|l| {
            let _ = map(all_consuming(parse_line), |data| {
                parsed_input.insert(data.monkey_name.to_owned(), data);
            })(l);
        });

        parsed_input
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MonkeyType {
    Number(i64),
    Other {
        operator: ArithmeticOperator,
        left: MonkeyName,
        right: MonkeyName,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Monkey {
    pub monkey_name: MonkeyName,
    pub monkey_type: MonkeyType,
}

pub type MonkeyName = String;

fn parse_operation(input: &str) -> IResult<&str, ArithmeticOperator> {
    let (input, operator) = alt((
        map(tag("+"), |_| ArithmeticOperator::Add),
        map(tag("-"), |_| ArithmeticOperator::Subtract),
        map(tag("*"), |_| ArithmeticOperator::Multiply),
        map(tag("/"), |_| ArithmeticOperator::Divide),
    ))(input)?;

    Ok((input, operator))
}

fn parse_arithmetic_operation(input: &str) -> IResult<&str, MonkeyType> {
    let (input, (left, _, operator, _, right)) =
        tuple((alpha1, space1, parse_operation, space1, alpha1))(input)?;

    Ok((
        input,
        MonkeyType::Other {
            operator,
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}

fn parse_monkey_type(input: &str) -> IResult<&str, MonkeyType> {
    let (input, monkey_type) = alt((
        map(parse_number_i64, MonkeyType::Number),
        parse_arithmetic_operation,
    ))(input)?;

    Ok((input, monkey_type))
}

fn parse_line(input: &str) -> IResult<&str, Monkey> {
    // root: pppw + sjmn
    // dvpt: 3

    let (input, (monkey_name, _, monkey_type)) =
        tuple((alpha1, tag(": "), parse_monkey_type))(input)?;

    Ok((
        input,
        Monkey {
            monkey_name: monkey_name.to_string(),
            monkey_type,
        },
    ))
}
