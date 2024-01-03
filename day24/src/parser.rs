use std::collections::HashMap;

use utils::{clear_screen, parser::ParseInput, wait_for_input};

pub const EMPTY: u8 = 0;
pub const UP: u8 = 1 << 0;
pub const DOWN: u8 = 1 << 1;
pub const LEFT: u8 = 1 << 2;
pub const RIGHT: u8 = 1 << 3;
pub const WALL: u8 = 1 << 4;

pub type Grid = Vec<Vec<u8>>;

#[allow(dead_code)]
pub fn print_grid(grid: &Vec<Vec<u8>>, current_pos: (usize, usize)) {
    clear_screen();
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if current_pos.0 == x && current_pos.1 == y {
                print!("@");
            } else {
                match *c {
                    EMPTY => print!("."),
                    WALL => print!("#"),
                    LEFT => print!("<"),
                    RIGHT => print!(">"),
                    UP => print!("^"),
                    DOWN => print!("v"),
                    _ => print!("*"),
                }
            }
        });
        println!();
    });
    wait_for_input();
}

#[derive(Debug)]
pub struct ParsedOutput {
    pub start_pos: (usize, usize),
    pub end_pos: (usize, usize),
    pub grid: Grid,
}

pub fn move_blizzard_up(grid: &Grid, pos: (usize, usize)) -> (usize, usize) {
    let (x, y) = pos;
    if y == 0 {
        return pos;
    }
    let mut new_pos = (x, y - 1);
    while grid[new_pos.1][new_pos.0] & WALL != 0 {
        new_pos.1 = grid.len() - 2;
    }
    new_pos
}
pub fn move_blizzard_down(grid: &Grid, pos: (usize, usize)) -> (usize, usize) {
    let (x, y) = pos;
    if y == grid.len() - 1 {
        return pos;
    }
    let mut new_pos = (x, y + 1);
    while grid[new_pos.1][new_pos.0] & WALL != 0 {
        new_pos.1 = 1;
    }
    new_pos
}
pub fn move_blizzard_right(grid: &Grid, pos: (usize, usize)) -> (usize, usize) {
    let (x, y) = pos;
    if x == grid[0].len() - 1 {
        return pos;
    }
    let mut new_pos = (x + 1, y);
    while grid[new_pos.1][new_pos.0] & WALL != 0 {
        new_pos.0 = 1;
    }
    new_pos
}
pub fn move_blizzard_left(grid: &Grid, pos: (usize, usize)) -> (usize, usize) {
    let (x, y) = pos;
    if x == 0 {
        return pos;
    }
    let mut new_pos = (x - 1, y);
    while grid[new_pos.1][new_pos.0] & WALL != 0 {
        new_pos.0 = grid[0].len() - 2;
    }
    new_pos
}

#[derive(Default)]
pub struct Problem {
    pub grid_time_cache: HashMap<u32, Grid>,
    pub start_min_elapsed: u32,
    pub grid_size: u32,
}

impl Problem {
    pub fn get_grid(&self, min: u32) -> Option<&Grid> {
        self.grid_time_cache.get(&(min % self.grid_size))
    }

    fn run_round(&mut self, grid: &Grid) -> Grid {
        let mut new_grid: Grid = vec![vec![EMPTY; grid[0].len()]; grid.len()];

        grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, c)| {
                match *c {
                    WALL => new_grid[y][x] = *c,
                    _ => {
                        if *c & LEFT > 0 {
                            let (newx, newy) = move_blizzard_left(grid, (x, y));
                            new_grid[newy][newx] |= LEFT;
                        };

                        if *c & RIGHT > 0 {
                            let (newx, newy) = move_blizzard_right(grid, (x, y));
                            new_grid[newy][newx] |= RIGHT;
                        };

                        if *c & UP > 0 {
                            let (newx, newy) = move_blizzard_up(grid, (x, y));
                            new_grid[newy][newx] |= UP;
                        };

                        if *c & DOWN > 0 {
                            let (newx, newy) = move_blizzard_down(grid, (x, y));
                            new_grid[newy][newx] |= DOWN;
                        };
                    }
                };
            });
        });

        new_grid
    }
}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, _: bool) -> ParsedOutput {
        let mut grid: Vec<Vec<u8>> = Vec::new();
        let mut start_pos = (0, 0);
        let mut end_pos_pos = (0, 0);

        input.lines().enumerate().for_each(|(y, l)| {
            let mut grid_row = Vec::new();
            l.chars().enumerate().for_each(|(x, c)| {
                if c == '.' {
                    if y == 0 {
                        start_pos = (x, y);
                    } else {
                        end_pos_pos = (x, y);
                    }
                }

                grid_row.push(match c {
                    '#' => WALL,
                    '<' => LEFT,
                    '>' => RIGHT,
                    'v' => DOWN,
                    '^' => UP,
                    _ => EMPTY,
                })
            });
            grid.push(grid_row);
        });

        self.grid_size = ((grid.len() - 2) * (grid[0].len() - 2)) as u32;
        self.start_min_elapsed = 0;

        let mut grid_pre_cache = grid.clone();

        for i in 0..self.grid_size {
            self.grid_time_cache.insert(i, grid_pre_cache.clone());
            grid_pre_cache = self.run_round(&grid_pre_cache);
        }

        ParsedOutput {
            start_pos,
            end_pos: end_pos_pos,
            grid,
        }
    }
}
