use crate::parser::{ArithmeticOperator, Day21, MonkeyName, MonkeyType, ParsedOutput};
pub use crate::part1::get_monkey_shout;
use std::collections::HashMap;
use utils::problem::SolvePart2;

const HUMAN_NAME: &str = "humn";

impl SolvePart2 for Day21 {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, parsed_input: ParsedOutput) -> String {
        let mut has_human_map: HashMap<MonkeyName, bool> = HashMap::new();

        fill_has_human_hashmap(&parsed_input, &mut has_human_map, "root".to_string());

        let root_monkey = parsed_input
            .get(&"root".to_string())
            .unwrap_or_else(|| panic!("Root not found."));

        let res = match &root_monkey.monkey_type {
            MonkeyType::Number(n) => n.to_string(),
            MonkeyType::Other {
                operator: _,
                left,
                right,
            } => {
                let left_has_human = has_human_map.get(left).is_some_and(|c| c == &true);
                let right_has_human = has_human_map.get(right).is_some_and(|c| c == &true);

                if left_has_human && right_has_human {
                    println!("monkey: {:#?}", root_monkey);
                    panic!("Both left and right have human.");
                }

                if left_has_human {
                    let val = get_monkey_shout(&parsed_input, right.to_owned());
                    return correct_human(&parsed_input, &mut has_human_map, left.to_owned(), val)
                        .to_string();
                }

                if right_has_human {
                    let val = get_monkey_shout(&parsed_input, left.to_owned());
                    return correct_human(&parsed_input, &mut has_human_map, right.to_owned(), val)
                        .to_string();
                }

                "0".to_string()
            }
        };

        res
    }
}

pub fn correct_human(
    parsed_input: &ParsedOutput,
    has_human_map: &mut HashMap<MonkeyName, bool>,
    monkey_name: MonkeyName,
    equal_value: i64,
) -> i64 {
    let monkey = parsed_input
        .get(&monkey_name)
        .unwrap_or_else(|| panic!("Monkey with name {} not found.", { monkey_name.to_owned() }));

    match &monkey.monkey_type {
        MonkeyType::Number(_) => equal_value,
        MonkeyType::Other {
            operator,
            left,
            right,
        } => {
            let left_has_human = has_human_map.get(left).is_some_and(|c| c == &true);
            let right_has_human = has_human_map.get(right).is_some_and(|c| c == &true);

            if left_has_human && right_has_human {
                println!("monkey: {:#?}", monkey);
                panic!("Both left and right have human.");
            }

            if left_has_human {
                let right_value = get_monkey_shout(parsed_input, right.to_owned());

                let correction = match operator {
                    ArithmeticOperator::Add => equal_value - right_value,
                    ArithmeticOperator::Subtract => equal_value + right_value,
                    ArithmeticOperator::Multiply => equal_value / right_value,
                    ArithmeticOperator::Divide => equal_value * right_value,
                };
                return correct_human(parsed_input, has_human_map, left.to_owned(), correction);
            }

            if right_has_human {
                let left_value = get_monkey_shout(parsed_input, left.to_owned());

                let correction = match operator {
                    ArithmeticOperator::Add => equal_value - left_value,
                    ArithmeticOperator::Subtract => left_value - equal_value,
                    ArithmeticOperator::Multiply => equal_value / left_value,
                    ArithmeticOperator::Divide => left_value / equal_value,
                };
                return correct_human(parsed_input, has_human_map, right.to_owned(), correction);
            }
            0
        }
    }
}

pub fn fill_has_human_hashmap(
    parsed_input: &ParsedOutput,
    has_human_map: &mut HashMap<MonkeyName, bool>,
    monkey_name: MonkeyName,
) -> bool {
    let monkey = parsed_input
        .get(&monkey_name)
        .unwrap_or_else(|| panic!("Monkey with name {} not found.", { monkey_name.to_owned() }));

    if monkey.monkey_name == HUMAN_NAME {
        has_human_map.insert(monkey_name.to_owned(), true);
        return true;
    }

    match &monkey.monkey_type {
        MonkeyType::Number(_) => false,
        MonkeyType::Other {
            operator: _,
            left,
            right,
        } => {
            let left_has_human =
                fill_has_human_hashmap(parsed_input, has_human_map, left.to_owned());
            let right_has_human =
                fill_has_human_hashmap(parsed_input, has_human_map, right.to_owned());
            let has_human = left_has_human || right_has_human;
            has_human_map.insert(monkey_name.to_owned(), has_human);
            has_human
        }
    }
}
