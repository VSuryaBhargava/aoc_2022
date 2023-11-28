use crate::parser::{ParsedOutput, Problem};
use std::collections::HashMap;
use utils::problem::SolvePart1;

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, jet_order: ParsedOutput) -> String {
        let mut algo = Algo::new(jet_order);

        let mut rock_number = 0;

        let mut max_rocks = self.total_number_of_rocks;
        let mut skipped_rocks_height = 0;

        // println!("Total number of rocks: {}", total_number_of_rocks);

        loop {
            rock_number += 1;
            if rock_number > max_rocks {
                println!("Breaking on rock {}", rock_number);
                break;
            }

            // println!("For rock {}", rock_no + 1);
            algo.spawn_rock_and_run_till_stationary();
            // println!("Max y: {}", algo.max_y + 1);
            // println!("Rest pos : {:?}", algo.current_rock);
            // wait_for_input();

            let state_id = algo.get_state_id();

            // println!(
            //     "Rock no: {} out of {}. Max y index is {}. State id: {}",
            //     rock_number, max_rocks, algo.max_y, state_id
            // );

            if let Some((prev_rock_no, prev_max_y)) = algo.state_id_map.get(&state_id) {
                // println!(
                //     "Found repeat: length = {}, height = {}",
                //     rock_number - prev_rock_no,
                //     algo.max_y - prev_max_y
                // );

                let delta_rock_no = rock_number - prev_rock_no;
                let delta_max_y = algo.max_y - prev_max_y;

                let deltas = (max_rocks - rock_number) / delta_rock_no;

                if deltas > 0 {
                    println!("current max y: {}", algo.max_y);

                    println!("Delta rock no: {}", delta_rock_no);
                    println!("Delta max y: {}", delta_max_y);

                    println!("Deltas: {}", deltas);

                    skipped_rocks_height += deltas as i128 * delta_max_y;

                    println!("Skipped rocks height: {}", skipped_rocks_height);

                    max_rocks -= deltas * delta_rock_no;

                    println!("New total rocks: {}", max_rocks);
                    continue;
                }
            }

            algo.state_id_map
                .insert(state_id.clone(), (rock_number, algo.max_y));
        }

        format!("{}", algo.max_y + skipped_rocks_height + 1)
    }
}

#[derive(Eq, Hash, Debug, PartialEq, Copy, Clone)]
pub struct Point(i128, i128);

pub const DASH: [Point; 4] = [Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0)];

pub const PLUS: [Point; 4] = [Point(1, 0), Point(0, 1), Point(2, 1), Point(1, 2)];

pub const L: [Point; 5] = [
    Point(0, 0),
    Point(1, 0),
    Point(2, 0),
    Point(2, 1),
    Point(2, 2),
];

pub const PIPE: [Point; 4] = [Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3)];

pub const SQUARE: [Point; 4] = [Point(0, 0), Point(0, 1), Point(1, 0), Point(1, 1)];

#[derive(Eq, Debug, PartialEq, Copy, Clone)]
// Root point in the bottom left point
pub enum Rock {
    Dash { point: Point },
    Plus { point: Point },
    L { point: Point },
    Pipe { point: Point },
    Square { point: Point },
}

impl Rock {
    pub fn new_dash() -> Rock {
        Rock::Dash { point: Point(0, 0) }
    }
    pub fn new_plus() -> Rock {
        Rock::Plus { point: Point(0, 0) }
    }
    pub fn new_l() -> Rock {
        Rock::L { point: Point(0, 0) }
    }
    pub fn new_pipe() -> Rock {
        Rock::Pipe { point: Point(0, 0) }
    }
    pub fn new_square() -> Rock {
        Rock::Square { point: Point(0, 0) }
    }

    pub fn get_rock_coordinates(&self, point: Point) -> Vec<Point> {
        let x_index = point.0;
        let y_index = point.1;

        let points = self.get_coordinate_offsets();

        let mut new_points: Vec<Point> = vec![Point(0, 0); points.len()];

        points
            .iter()
            .enumerate()
            .for_each(|(i, p)| new_points[i] = Point(p.0 + x_index, p.1 + y_index));

        new_points
    }

    pub fn get_point(&self) -> Point {
        match self {
            Rock::Dash { point }
            | Rock::Plus { point }
            | Rock::L { point }
            | Rock::Pipe { point }
            | Rock::Square { point } => *point,
        }
    }

    pub fn set_point(&mut self, p: Point) {
        match self {
            Rock::Dash { point }
            | Rock::Plus { point }
            | Rock::L { point }
            | Rock::Pipe { point }
            | Rock::Square { point } => {
                point.0 = p.0;
                point.1 = p.1;
            }
        }
    }

    fn get_coordinate_offsets(&self) -> &'static [Point] {
        match self {
            Rock::Dash { point: _ } => &DASH,
            Rock::Plus { point: _ } => &PLUS,
            Rock::L { point: _ } => &L,
            Rock::Pipe { point: _ } => &PIPE,
            Rock::Square { point: _ } => &SQUARE,
        }
    }
}

pub struct Algo {
    pub jet_order: Vec<u8>,
    pub rocks_order: [Rock; 5],
    pub max_y: i128,
    pub stationary_rocks: HashMap<Point, bool>,
    pub current_rock_index: i8,
    pub current_jet_index: usize,
    pub max_x: u8,
    pub current_rock: Rock,
    // (length, height)
    pub state_id_map: HashMap<String, (u64, i128)>,
}

impl Algo {
    pub fn new(jet_order: Vec<u8>) -> Algo {
        let mut stationary_rocks = HashMap::new();
        stationary_rocks.insert(Point(0, -1), true);
        stationary_rocks.insert(Point(1, -1), true);
        stationary_rocks.insert(Point(2, -1), true);
        stationary_rocks.insert(Point(3, -1), true);
        stationary_rocks.insert(Point(4, -1), true);
        stationary_rocks.insert(Point(5, -1), true);
        stationary_rocks.insert(Point(6, -1), true);

        Algo {
            max_y: -1,
            stationary_rocks,
            current_rock: Rock::new_dash(),
            current_jet_index: 0,
            current_rock_index: 0,
            max_x: 6,
            jet_order,
            rocks_order: [
                Rock::new_dash(),
                Rock::new_plus(),
                Rock::new_l(),
                Rock::new_pipe(),
                Rock::new_square(),
            ],
            state_id_map: HashMap::new(),
        }
    }

    pub fn spawn_rock(&mut self) {
        self.current_rock = self.rocks_order[self.current_rock_index as usize];
        self.current_rock_index = (self.current_rock_index + 1) % self.rocks_order.len() as i8;

        let x_index = 2;
        let y_index = 4 + self.max_y;

        self.current_rock.set_point(Point(x_index, y_index));
    }

    pub fn is_point_full(&self, point: &Point) -> bool {
        point.0 < 0
            || point.0 > self.max_x as i128
            || point.1 < 0
            || self.stationary_rocks.get(point).is_some()
    }

    pub fn move_horizontal(&mut self) {
        let point = self.current_rock.get_point();

        let new_point = match self.jet_order[self.current_jet_index] {
            b'<' => Point(point.0 - 1, point.1),
            b'>' => Point(point.0 + 1, point.1),
            _ => panic!("Invalid move"),
        };

        self.current_jet_index = (self.current_jet_index + 1) % self.jet_order.len();

        let is_any_point_full = self
            .current_rock
            .get_rock_coordinates(new_point)
            .iter()
            .any(|p| self.is_point_full(p));

        if !is_any_point_full {
            self.current_rock.set_point(new_point);
        }
    }

    // Returns true if rock has is at bottom and can't move anymore
    pub fn move_vertical(&mut self) -> bool {
        let point = self.current_rock.get_point();
        let new_point = Point(point.0, point.1 - 1);

        let rock_new_co_ordinates = self.current_rock.get_rock_coordinates(new_point);

        let is_any_point_full = rock_new_co_ordinates.iter().any(|p| self.is_point_full(p));

        if !is_any_point_full {
            self.current_rock.set_point(new_point);
            // println!("Rest pos after vertical movement: {:?}", self.current_rock);
            false
        } else {
            let rock_new_co_ordinates = self.current_rock.get_rock_coordinates(point);
            rock_new_co_ordinates.iter().for_each(|p| {
                self.stationary_rocks.insert(*p, true);

                self.max_y = std::cmp::max(self.max_y, p.1);
            });
            // println!("Can't move down: {:?}", self.current_rock);
            true
        }
    }

    pub fn spawn_rock_and_run_till_stationary(&mut self) {
        self.spawn_rock();

        // println!("Rest pos : {:?}", self.current_rock);

        loop {
            self.move_horizontal();
            // println!(
            //     "Rest pos after horizontal movement: {:?}",
            //     self.current_rock
            // );
            if self.move_vertical() {
                break;
            }
        }
    }

    pub fn get_state_id(&self) -> String {
        let empty_sots_on_top = [self.max_y; 7];
        let empty_sots_on_top = empty_sots_on_top
            .iter()
            .enumerate()
            .map(|(x, _)| {
                for i in 0..self.max_y {
                    // println!("Checking for {:?}", Point(x as i128, self.max_y - i));
                    if self
                        .stationary_rocks
                        .get(&Point(x as i128, self.max_y - i))
                        .is_some()
                    {
                        return i.to_string();
                    }
                }
                self.max_y.to_string()
            })
            .collect::<Vec<String>>()
            .join("-");
        format!(
            "{}-{}-{}",
            self.current_rock_index, self.current_jet_index, empty_sots_on_top
        )
    }
}
