use utils::problem::SolvePart1;

use crate::parser::{ArithmeticOperator, Day21, MonkeyName, MonkeyType, ParsedOutput};

impl SolvePart1 for Day21 {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, parsed_input: ParsedOutput) -> String {
        get_monkey_shout(&parsed_input, "root".to_string()).to_string()
    }
}

pub fn get_monkey_shout(parsed_input: &ParsedOutput, monkey_name: MonkeyName) -> i64 {
    let monkey = parsed_input
        .get(&monkey_name)
        .unwrap_or_else(|| panic!("Monkey with name {} not found.", { monkey_name.to_owned() }));

    match &monkey.monkey_type {
        MonkeyType::Number(n) => *n,
        MonkeyType::Other {
            right: right_monkey_name,
            operator,
            left: left_monkey_name,
        } => {
            let right_monkey_name = right_monkey_name.to_owned();
            let left_monkey_name = left_monkey_name.to_owned();
            let operator = operator.to_owned();
            let left = get_monkey_shout(parsed_input, left_monkey_name);
            let right = get_monkey_shout(parsed_input, right_monkey_name);
            match operator {
                ArithmeticOperator::Add => left + right,
                ArithmeticOperator::Subtract => left - right,
                ArithmeticOperator::Multiply => left * right,
                ArithmeticOperator::Divide => left / right,
            }
        }
    }
}
