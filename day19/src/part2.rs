use crate::{parser::BluePrint, part1};
use crate::{parser::ParsedOutput, Problem};
use utils::problem::SolvePart2;

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, blueprints: ParsedOutput) -> String {
        let mut new_blueprint_vec: Vec<BluePrint> = Vec::with_capacity(3);

        for (i, blueprint) in blueprints.iter().enumerate() {
            new_blueprint_vec.push(blueprint.to_owned());
            if i == 2 {
                break;
            }
        }

        let max_geodes = part1::get_max_geodes_per_blue_print(self, new_blueprint_vec);

        let product_of_max_geodes = max_geodes.iter().fold(1, |acc, (_, f)| acc * f);

        product_of_max_geodes.to_string()
    }
}
