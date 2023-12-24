use utils::{
    nom::{
        branch::alt,
        character::complete::char,
        combinator::{map, opt},
        multi::many1,
        IResult,
    },
    parser::{parse_number_u32, ParseInput},
};

#[derive(Debug)]
pub struct ParsedOutput {
    pub instructions: Vec<Instructions>,
    pub map: Vec<Vec<Points>>,
}

impl ParsedOutput {
    pub fn get_point(&self, position: (usize, usize)) -> Points {
        if let Some(row) = self.map.get(position.1) {
            if let Some(val) = row.get(position.0) {
                return *val;
            }
        }
        Points::OutOfBounds
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Direction {
    North,
    South,
    #[default]
    East,
    West,
}

#[derive(Default)]
pub struct Problem {
    pub position: (usize, usize),
    pub direction: Direction,
}

impl Direction {
    pub fn get_direction_value(&self) -> usize {
        match self {
            Direction::North => 3,
            Direction::South => 1,
            Direction::East => 0,
            Direction::West => 2,
        }
    }
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    pub fn get_new_point(&self, (x, y): (usize, usize), distance: usize) -> (usize, usize) {
        match self {
            Direction::North => (x, y - distance),
            Direction::South => (x, y + distance),
            Direction::East => (x + distance, y),
            Direction::West => (x - distance, y),
        }
    }
}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, _: bool) -> ParsedOutput {
        self.position.0 = 0;
        self.position.1 = 0;

        let mut parsed_input = ParsedOutput {
            instructions: Vec::new(),
            map: Vec::new(),
        };

        parsed_input.map.push(Vec::new());

        input.lines().enumerate().for_each(|(line_no, l)| {
            let _ = map(parse_line, |(ins, points)| {
                if let Some(ins) = ins {
                    parsed_input.instructions = ins;
                }
                if let Some(mut points) = points {
                    points.insert(0, Points::OutOfBounds);
                    if line_no == 0 {
                        for (i, point) in points.iter().enumerate() {
                            if let Points::Empty = point {
                                self.position.0 = i;
                                self.position.1 = parsed_input.map.len();
                                break;
                            }
                        }
                    }
                    parsed_input.map.push(points);
                }
            })(l);
        });

        parsed_input
    }
}

#[derive(Debug)]
pub enum Instructions {
    Move(u32),
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub enum Points {
    OutOfBounds,
    Wall,
    Empty,
}

fn parse_map(input: &str) -> IResult<&str, Vec<Points>> {
    many1(alt((
        map(char(' '), |_| Points::OutOfBounds),
        map(char('.'), |_| Points::Empty),
        map(char('#'), |_| Points::Wall),
    )))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instructions>> {
    many1(alt((
        map(parse_number_u32, Instructions::Move),
        map(char('L'), |_| Instructions::Left),
        map(char('R'), |_| Instructions::Right),
    )))(input)
}

fn parse_line(input: &str) -> IResult<&str, (Option<Vec<Instructions>>, Option<Vec<Points>>)> {
    let (input, instructions) = opt(parse_instructions)(input)?;
    if let Some(instructions) = instructions {
        return Ok((input, (Some(instructions), None)));
    }

    let (input, points) = opt(parse_map)(input)?;
    if let Some(points) = points {
        return Ok((input, (None, Some(points))));
    }

    Ok((input, (None, None)))
}
