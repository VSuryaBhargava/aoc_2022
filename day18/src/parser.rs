use std::collections::HashMap;
use std::ops::Add;
use utils::{
    nom::{
        bytes::complete::tag,
        combinator::{all_consuming, map},
        IResult,
    },
    parser::{parse_number_i32, ParseInput},
};

const TOP_SIDE: u8 = 0b00000001;
const BOTTOM_SIDE: u8 = 0b00000010;
const LEFT_SIDE: u8 = 0b00000100;
const RIGHT_SIDE: u8 = 0b00001000;
const FRONT_SIDE: u8 = 0b00010000;
const BACK_SIDE: u8 = 0b00100000;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

pub const SIDES: [((u8, Point), (u8, Point)); 6] = [
    (
        (TOP_SIDE, Point { x: 0, y: 1, z: 0 }),
        (BOTTOM_SIDE, Point { x: 0, y: -1, z: 0 }),
    ),
    (
        (BOTTOM_SIDE, Point { x: 0, y: -1, z: 0 }),
        (TOP_SIDE, Point { x: 0, y: 1, z: 0 }),
    ),
    (
        (LEFT_SIDE, Point { x: -1, y: 0, z: 0 }),
        (RIGHT_SIDE, Point { x: 1, y: 0, z: 0 }),
    ),
    (
        (RIGHT_SIDE, Point { x: 1, y: 0, z: 0 }),
        (LEFT_SIDE, Point { x: -1, y: 0, z: 0 }),
    ),
    (
        (FRONT_SIDE, Point { x: 0, y: 0, z: -1 }),
        (BACK_SIDE, Point { x: 0, y: 0, z: 1 }),
    ),
    (
        (BACK_SIDE, Point { x: 0, y: 0, z: 1 }),
        (FRONT_SIDE, Point { x: 0, y: 0, z: -1 }),
    ),
];

#[derive(Debug, PartialEq, Eq)]
pub struct Cube {
    pub point: Point,
    pub x_len: i32,
    pub y_len: i32,
    pub z_len: i32,
    pub checked_sides: u8,
    pub surface_area: u8,
}

pub type ParsedOutput = (Vec<Point>, HashMap<Point, Cube>, (Point, Point));

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    // 6,18,9
    let (input, x) = parse_number_i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = parse_number_i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = parse_number_i32(input)?;

    Ok((
        input,
        Cube {
            point: Point { x, y, z },
            x_len: 1,
            y_len: 1,
            z_len: 1,
            checked_sides: 0,
            surface_area: 6,
        },
    ))
}

#[derive(Default)]
pub struct Problem {}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, _: bool) -> ParsedOutput {
        let mut cubes = HashMap::new();
        let mut coords = Vec::new();

        let mut min_point = Point {
            x: i32::MAX,
            y: i32::MAX,
            z: i32::MAX,
        };
        let mut max_point = Point {
            x: i32::MIN,
            y: i32::MIN,
            z: i32::MIN,
        };

        input.lines().for_each(|l| {
            let _ = map(all_consuming(parse_cube), |cube| {
                min_point.x = std::cmp::min(min_point.x, cube.point.x);
                min_point.y = std::cmp::min(min_point.y, cube.point.y);
                min_point.z = std::cmp::min(min_point.z, cube.point.z);

                max_point.x = std::cmp::max(max_point.x, cube.point.x);
                max_point.y = std::cmp::max(max_point.y, cube.point.y);
                max_point.z = std::cmp::max(max_point.z, cube.point.z);

                coords.push(cube.point);
                cubes.insert(cube.point, cube);
            })(l);
        });

        (coords, cubes, (min_point, max_point))
    }
}
