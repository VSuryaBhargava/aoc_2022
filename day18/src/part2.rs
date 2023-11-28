use crate::{
    parser::{Cube, ParsedOutput, Point, SIDES},
    Problem,
};
use std::collections::HashMap;
use utils::problem::SolvePart2;

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, (coords, input, (min_point, max_point)): ParsedOutput) -> String {
        let mut total_surface_area: u32 = 0;

        let min_point = Point {
            x: min_point.x - 1,
            y: min_point.y - 1,
            z: min_point.z - 1,
        };
        let max_point = Point {
            x: max_point.x + 2,
            y: max_point.y + 2,
            z: max_point.z + 2,
        };

        let x_delta = max_point.x - min_point.x;
        let y_delta = max_point.y - min_point.y;
        let z_delta = max_point.z - min_point.z;

        let mut grid: Vec<Vec<Vec<bool>>> =
            vec![vec![vec![false; z_delta as usize]; y_delta as usize]; x_delta as usize];

        fill_empty_space(&min_point, &min_point, &max_point, &input, &mut grid);

        for point in coords {
            total_surface_area += get_surface_area_of_cube(&point, &min_point, &grid);
        }

        total_surface_area.to_string()
    }
}

fn fill_empty_space(
    current_point: &Point,
    min_point: &Point,
    max_point: &Point,
    data: &HashMap<Point, Cube>,
    grid: &mut Vec<Vec<Vec<bool>>>,
) {
    if data.get(current_point).is_some()
        || current_point.x < min_point.x
        || current_point.y < min_point.y
        || current_point.z < min_point.z
        || current_point.x >= max_point.x
        || current_point.y >= max_point.y
        || current_point.z >= max_point.z
    {
        return;
    }

    let grid_x: usize = (current_point.x - min_point.x) as usize;
    let grid_y = (current_point.y - min_point.y) as usize;
    let grid_z = (current_point.z - min_point.z) as usize;

    if grid[grid_x][grid_y][grid_z] {
        return;
    }

    grid[grid_x][grid_y][grid_z] = true;

    fill_empty_space(
        &Point {
            x: current_point.x + 1,
            y: current_point.y,
            z: current_point.z,
        },
        min_point,
        max_point,
        data,
        grid,
    );
    fill_empty_space(
        &Point {
            x: current_point.x - 1,
            y: current_point.y,
            z: current_point.z,
        },
        min_point,
        max_point,
        data,
        grid,
    );
    fill_empty_space(
        &Point {
            x: current_point.x,
            y: current_point.y + 1,
            z: current_point.z,
        },
        min_point,
        max_point,
        data,
        grid,
    );
    fill_empty_space(
        &Point {
            x: current_point.x,
            y: current_point.y - 1,
            z: current_point.z,
        },
        min_point,
        max_point,
        data,
        grid,
    );
    fill_empty_space(
        &Point {
            x: current_point.x,
            y: current_point.y,
            z: current_point.z + 1,
        },
        min_point,
        max_point,
        data,
        grid,
    );
    fill_empty_space(
        &Point {
            x: current_point.x,
            y: current_point.y,
            z: current_point.z - 1,
        },
        min_point,
        max_point,
        data,
        grid,
    );
}

fn get_surface_area_of_cube(point: &Point, min_point: &Point, grid: &Vec<Vec<Vec<bool>>>) -> u32 {
    SIDES.iter().fold(
        0,
        |acc, ((_side, side_offset), (_opposite_side, _opposite_side_offset))| {
            let opposite_point = *point + *side_offset;
            let grid_x: usize = (opposite_point.x - min_point.x) as usize;
            let grid_y = (opposite_point.y - min_point.y) as usize;
            let grid_z = (opposite_point.z - min_point.z) as usize;

            if !grid[grid_x][grid_y][grid_z] {
                return acc;
            }

            acc + 1
        },
    )
}
