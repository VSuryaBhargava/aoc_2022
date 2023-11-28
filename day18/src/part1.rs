use crate::parser::{Cube, ParsedOutput, Point, Problem, SIDES};
use std::collections::HashMap;
use utils::problem::SolvePart1;

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, (coords, mut input, _): ParsedOutput) -> String {
        let mut total_surface_area: u32 = 0;

        for point in coords {
            total_surface_area += get_surface_area_of_cube(&point, &mut input);
        }

        total_surface_area.to_string()
    }
}

fn get_surface_area_of_cube(point: &Point, data: &mut HashMap<Point, Cube>) -> u32 {
    SIDES.iter().fold(
        0,
        |acc, ((side, side_offset), (opposite_side, _opposite_side_offset))| {
            let opposite_point = *point + *side_offset;

            let has_opposite_cube = data.get(&opposite_point).is_some();

            if let Some(cube) = data.get_mut(point) {
                if cube.checked_sides & *side == *side {
                    return acc;
                }
                cube.checked_sides |= *side;
                if has_opposite_cube {
                    cube.surface_area -= 1;
                }
            }
            if let Some(opposite_cube) = data.get_mut(&opposite_point) {
                opposite_cube.checked_sides |= *opposite_side;
                opposite_cube.surface_area -= 1;
            }

            if has_opposite_cube {
                acc
            } else {
                acc + 1
            }
        },
    )
}
