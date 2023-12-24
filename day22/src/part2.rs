use std::collections::HashMap;
use utils::problem::SolvePart2;

use crate::{
    parser::{Direction, ParsedOutput, Points},
    part1::{WrapFunction, WrapPointState},
    Problem,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct StitchEdge {
    point: (usize, usize),
    stitch_direction: Direction,
    move_direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct StitchSection {
    edge1: StitchEdge,
    edge2: StitchEdge,
}

impl SolvePart2 for Problem {
    type ParsedType = ParsedOutput;
    fn solve_part_two(&mut self, parsed_input: ParsedOutput) -> String {
        let stitch_data = stitch_cube(&parsed_input);

        self.solve(
            parsed_input,
            &(Box::new(
                move |point: (usize, usize),
                      _parsed_output: &ParsedOutput,
                      direction: Direction| {
                    let a = stitch_data
                        .get(&WrapPointState(point, direction))
                        .expect("Unable to find stitch point");

                    *a
                },
            ) as WrapFunction),
        )
    }
}

fn stitch_cube(parsed_input: &ParsedOutput) -> HashMap<WrapPointState, WrapPointState> {
    let mut stitch: HashMap<WrapPointState, WrapPointState> = HashMap::new();

    let max_x = parsed_input.map.iter().map(|f| f.len()).max().unwrap();
    let max_y = parsed_input.map.len();

    let max_length = std::cmp::max(max_x - 1, max_y - 1);
    let min_length = std::cmp::min(max_x - 1, max_y - 1);
    let cube_edge_size = if (min_length / 3) * 4 == max_length {
        max_length / 4
    } else {
        max_length / 5
    };

    let stitch_points = get_stitch_points(parsed_input, cube_edge_size, max_x, max_y);

    // println!("Stitch points: {:?}", stitch_points);
    stitch_points.into_iter().for_each(|stitch_point| {
        stitch_patch(stitch_point, parsed_input, &mut stitch, cube_edge_size);
    });

    stitch
}

fn stitch_patch(
    stitch_section: StitchSection,
    parsed_input: &ParsedOutput,
    stitch_map: &mut HashMap<WrapPointState, WrapPointState>,
    cube_edge_size: usize,
) {
    // if this was already stitched, then skip
    if stitch_map.contains_key(&WrapPointState(
        stitch_section.edge1.point,
        stitch_section.edge1.move_direction,
    )) {
        return;
    }

    let mut end_edge_point1 = (0, 0);
    let mut end_edge_point2 = (0, 0);

    for i in 0..cube_edge_size {
        let point1 = stitch_section
            .edge1
            .move_direction
            .get_new_point(stitch_section.edge1.point, i);
        let point2 = stitch_section
            .edge2
            .move_direction
            .get_new_point(stitch_section.edge2.point, i);

        stitch_map.insert(
            WrapPointState(point1, stitch_section.edge1.stitch_direction),
            WrapPointState(point2, stitch_section.edge2.stitch_direction.opposite()),
        );

        stitch_map.insert(
            WrapPointState(point2, stitch_section.edge2.stitch_direction),
            WrapPointState(point1, stitch_section.edge1.stitch_direction.opposite()),
        );
        end_edge_point1 = point1;
        end_edge_point2 = point2;
    }

    let new_move_point1 = stitch_section
        .edge1
        .move_direction
        .get_new_point(end_edge_point1, 1);
    let new_stitch_point1 = stitch_section
        .edge1
        .stitch_direction
        .get_new_point(new_move_point1, 1);

    let new_move_point2 = stitch_section
        .edge2
        .move_direction
        .get_new_point(end_edge_point2, 1);
    let new_stitch_point2 = stitch_section
        .edge2
        .stitch_direction
        .get_new_point(new_move_point2, 1);

    if !matches!(
        parsed_input.get_point(new_stitch_point1),
        Points::OutOfBounds
    ) || !matches!(
        parsed_input.get_point(new_stitch_point2),
        Points::OutOfBounds
    ) {
        return;
    }

    let can_go_forward1 = !matches!(parsed_input.get_point(new_move_point1), Points::OutOfBounds);
    let can_go_forward2 = !matches!(parsed_input.get_point(new_move_point2), Points::OutOfBounds);

    if can_go_forward1 == can_go_forward2 {
        return;
    }

    // Move direction becomes the new stitch direction
    // The opposite of the stitch direction becomes the new move direction
    if !can_go_forward1 {
        let new_move_direction = stitch_section.edge1.stitch_direction.opposite();
        stitch_patch(
            StitchSection {
                // Here edge 1 is at a corner and the new direction is still in the same face and starts
                // at the same corner so we don't need to increase move the start point
                edge1: StitchEdge {
                    point: end_edge_point1,
                    stitch_direction: stitch_section.edge1.move_direction,
                    move_direction: new_move_direction,
                },
                // Here edge 2 is straight and is moving into another face of the cube so we increase the start point
                edge2: StitchEdge {
                    point: stitch_section
                        .edge2
                        .move_direction
                        .get_new_point(end_edge_point2, 1),
                    ..stitch_section.edge2
                },
            },
            parsed_input,
            stitch_map,
            cube_edge_size,
        )
    }
    if !can_go_forward2 {
        let _new_move_direction = stitch_section.edge2.stitch_direction.opposite();
        stitch_patch(
            StitchSection {
                // Here edge 1 is straight and is moving into another face of the cube so we increase the start point
                edge1: StitchEdge {
                    point: stitch_section
                        .edge1
                        .move_direction
                        .get_new_point(end_edge_point1, 1),
                    ..stitch_section.edge1
                },
                // Here edge 2 is at a corner and the new direction is still in the same face and starts
                // at the same corner so we don't need to increase move the start point
                edge2: StitchEdge {
                    point: end_edge_point2,
                    stitch_direction: stitch_section.edge2.move_direction,
                    move_direction: stitch_section.edge2.stitch_direction.opposite(),
                },
            },
            parsed_input,
            stitch_map,
            cube_edge_size,
        )
    }
}

fn get_stitch_points(
    parsed_input: &ParsedOutput,
    cube_edge_size: usize,
    max_x: usize,
    max_y: usize,
) -> Vec<StitchSection> {
    let mut stitch_points: Vec<StitchSection> = vec![];

    let mut i = 1;
    loop {
        i += cube_edge_size;
        if i > max_x {
            break;
        }
        let mut j = 1;
        loop {
            j += cube_edge_size;
            if j > max_y {
                break;
            }

            // Which one of these quadrant are empty, will determine the direction of the stitch
            let (empty_faces_at_corner, point, quadrant) = [
                // (empty_point, corner_point)
                ((i, j - 1), (i - 1, j)), // quadrant 1, index 0, Stitch top and right together
                ((i - 1, j - 1), (i, j)), // quadrant 2, index 1, Stitch top and left together
                ((i - 1, j), (i, j - 1)), // quadrant 3, index 2, Stitch bottom and left together
                ((i, j), (i - 1, j - 1)), // quadrant 4, index 3, Stitch bottom and right together
            ]
            .into_iter()
            .enumerate()
            .fold(
                (0, (0, 0), 1),
                |a, (quadrant, (p, corner_point))| match parsed_input.get_point(p) {
                    Points::OutOfBounds => (a.0 + 1, corner_point, quadrant + 1),
                    _ => a,
                },
            );

            if empty_faces_at_corner == 1 {
                let ((move_dir1, stitch_dir1), (move_dir2, stitch_dir2)) = match quadrant {
                    1 => (
                        (Direction::North, Direction::East),
                        (Direction::East, Direction::North),
                    ),
                    2 => (
                        (Direction::North, Direction::West),
                        (Direction::West, Direction::North),
                    ),
                    3 => (
                        (Direction::South, Direction::West),
                        (Direction::West, Direction::South),
                    ),
                    4 => (
                        (Direction::South, Direction::East),
                        (Direction::East, Direction::South),
                    ),
                    _ => panic!("Invalid quadrant"),
                };
                // stitch_points.push((point, vert_dir, hor_dir));
                stitch_points.push(StitchSection {
                    edge1: StitchEdge {
                        point: move_dir1.get_new_point(point, 1),
                        stitch_direction: stitch_dir1,
                        move_direction: move_dir1,
                    },
                    edge2: StitchEdge {
                        point: move_dir2.get_new_point(point, 1),
                        stitch_direction: stitch_dir2,
                        move_direction: move_dir2,
                    },
                });
            }
        }
    }
    stitch_points
}
