use utils::{problem::SolvePart1, wait_for_input};

use crate::parser::{Direction, Instructions, ParsedOutput, Points, Problem};

pub type WrapFunction = Box<dyn Fn((usize, usize), &ParsedOutput, Direction) -> WrapPointState>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WrapPointState(pub (usize, usize), pub Direction);

#[allow(dead_code)]
pub fn print_map(map: &[Vec<Points>], problem: &Problem, string: String) {
    print!("\x1B[2J\x1B[1;1H");
    println!("Position: {:?}", problem.position);
    for (y, row) in map.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            match point {
                Points::Empty => {
                    if problem.position.0 == x && problem.position.1 == y {
                        match problem.direction {
                            Direction::North => print!("^"),
                            Direction::South => print!("v"),
                            Direction::East => print!(">"),
                            Direction::West => print!("<"),
                        }
                    } else {
                        print!(".")
                    }
                }
                Points::Wall => print!("#"),
                Points::OutOfBounds => print!(" "),
            }
        }
        println!();
    }
    println!("{}", string);

    wait_for_input();
}

impl Problem {
    pub fn solve(&mut self, parsed_input: ParsedOutput, get_warped_point: &WrapFunction) -> String {
        parsed_input.instructions.iter().for_each(|instruction| {
            // print_map(&parsed_input.map, self, format!("{:?}", instruction));

            match instruction {
                Instructions::Left => {
                    self.direction = match self.direction {
                        Direction::North => Direction::West,
                        Direction::South => Direction::East,
                        Direction::East => Direction::North,
                        Direction::West => Direction::South,
                    }
                }
                Instructions::Right => {
                    self.direction = match self.direction {
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                    }
                }
                Instructions::Move(value) => {
                    let new_data = get_new_pos(self, &parsed_input, *value, get_warped_point);
                    self.position = new_data.0;
                    self.direction = new_data.1;
                }
            };
        });

        format!(
            "{}",
            1000 * (self.position.1) + 4 * (self.position.0) + self.direction.get_direction_value()
        )
    }
}

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, parsed_input: ParsedOutput) -> String {
        self.solve(parsed_input, &(Box::new(get_warped_point) as WrapFunction))
    }
}

pub fn get_new_pos(
    problem: &Problem,
    parsed_input: &ParsedOutput,
    move_distance: u32,
    get_warped_point: &WrapFunction,
) -> WrapPointState {
    let mut return_pos = problem.position;
    let mut direction = problem.direction;
    for _ in 0..move_distance {
        let new_pos = match direction {
            Direction::North => (return_pos.0, return_pos.1 - 1),
            Direction::South => (return_pos.0, return_pos.1 + 1),
            Direction::East => (return_pos.0 + 1, return_pos.1),
            Direction::West => (return_pos.0 - 1, return_pos.1),
        };

        let val = parsed_input.get_point(new_pos);
        match val {
            Points::OutOfBounds => {
                let new_data = get_warped_point(return_pos, parsed_input, direction);

                if matches!(parsed_input.get_point(new_data.0), Points::Wall) {
                    break;
                }

                return_pos = new_data.0;
                direction = new_data.1;
            }
            Points::Wall => {
                break;
            }
            Points::Empty => {
                return_pos = new_pos;
            }
        }
    }

    WrapPointState(return_pos, direction)
}

fn get_warped_point(
    pos: (usize, usize),
    parsed_input: &ParsedOutput,
    direction: Direction,
) -> WrapPointState {
    let move_dir: (isize, isize) = match direction {
        Direction::North => (0, 1),
        Direction::South => (0, -1),
        Direction::East => (-1, 0),
        Direction::West => (1, 0),
    };

    // println!("Warping from {:?} in direction {:?}", pos, move_dir);

    let mut new_pos = pos;

    loop {
        let pos_to_check: (isize, isize) = (
            new_pos.0 as isize + move_dir.0,
            new_pos.1 as isize + move_dir.1,
        );

        let point = parsed_input.get_point((pos_to_check.0 as usize, pos_to_check.1 as usize));

        if let Points::OutOfBounds = point {
            break;
        }

        new_pos = (pos_to_check.0 as usize, pos_to_check.1 as usize);
    }

    if let Points::Wall = parsed_input.get_point(new_pos) {
        return WrapPointState(pos, direction);
    }

    WrapPointState(new_pos, direction)
}
