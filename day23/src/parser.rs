use std::collections::BTreeSet;

use utils::parser::ParseInput;

pub type ParsedOutput = ((isize, isize), (isize, isize), BTreeSet<(isize, isize)>);

#[derive(Default)]
pub enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

#[derive(Default)]
pub struct Problem {
    pub checking_order: [Direction; 4],
    pub checking_order_start_index: usize,
}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, _: bool) -> ParsedOutput {
        let mut elv_pos: BTreeSet<(isize, isize)> = BTreeSet::new();
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;

        self.checking_order = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        self.checking_order_start_index = 0;

        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    min_x = min_x.min(x as isize);
                    min_y = min_y.min(y as isize);
                    max_x = max_x.max(x as isize);
                    max_y = max_y.max(y as isize);
                    elv_pos.insert((x as isize, y as isize));
                }
            })
        });

        ((min_x, min_y), (max_x, max_y), elv_pos)
    }
}
