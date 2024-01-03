use std::collections::{HashMap, HashSet};

use utils::problem::SolvePart1;

use crate::parser::{ParsedOutput, Problem, EMPTY};

// the key is (elapsed_time % grid_size, current_pos, elapsed_time)
pub type Cache = HashMap<(u32, (usize, usize), u32), u32>;

// the key is (elapsed_time % grid_size, current_pos)
pub type LoopCache = HashSet<(u32, (usize, usize))>;

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, parsed_input: ParsedOutput) -> String {
        let mut current_min = u32::MAX;
        let mut cache: Cache = HashMap::new();

        let mut loop_grid_cache: LoopCache = HashSet::new();

        solve(
            self,
            parsed_input.start_pos,
            parsed_input.end_pos,
            self.start_min_elapsed,
            &mut current_min,
            &mut cache,
            &mut loop_grid_cache,
        )
        .to_string()
    }
}

pub fn solve(
    problem: &mut Problem,
    (x, y): (usize, usize),
    end_pos: (usize, usize),
    minutes_elapsed: u32,
    current_min: &mut u32,
    cache: &mut Cache,
    loop_grid_cache: &mut LoopCache,
) -> u32 {
    let grid_cache_key = minutes_elapsed % problem.grid_size;

    if loop_grid_cache.contains(&(grid_cache_key, (x, y))) {
        return u32::MAX;
    }

    loop_grid_cache.insert((grid_cache_key, (x, y)));

    if cache
        .get(&(grid_cache_key, (x, y), minutes_elapsed))
        .is_some()
    {
        return *cache
            .get(&(grid_cache_key, (x, y), minutes_elapsed))
            .unwrap();
    }

    // println!("Elapsed time: {}", minutes_elapsed);
    // print_grid(problem.get_grid(minutes_elapsed).unwrap(), (x, y));

    if x == end_pos.0 && y == end_pos.1 {
        if minutes_elapsed < *current_min {
            *current_min = minutes_elapsed;
        }
        cache.insert((grid_cache_key, (x, y), minutes_elapsed), minutes_elapsed);
        return minutes_elapsed;
    }

    let next_grid = problem.get_grid(minutes_elapsed + 1).unwrap().clone();
    let max_x = next_grid[0].len() - 1;
    let min_y = next_grid.len() - 1;

    if (*current_min as usize)
        < (minutes_elapsed as usize + end_pos.0.abs_diff(x) + end_pos.1.abs_diff(y))
    {
        cache.insert((grid_cache_key, (x, y), minutes_elapsed), u32::MAX);
        return u32::MAX;
    }

    let mut new_positions: Vec<(usize, usize)> = (vec![(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)])
        .into_iter()
        .filter(|(delta_x, delta_y)| {
            if x == 0 && *delta_x == -1 {
                return false;
            }
            if y == 0 && *delta_y == -1 {
                return false;
            }
            if x == max_x && *delta_x == 1 {
                return false;
            }
            if y == min_y && *delta_y == 1 {
                return false;
            }
            let next_y = (*delta_y + (y as i32)) as usize;
            let next_x = ((x as i32) + *delta_x) as usize;
            if next_grid[next_y][next_x] != EMPTY {
                return false;
            }

            // This was done assuming that if a blizzard is heading towards you, you can't move in that direction
            // but the sample input shows that you can move in that direction
            // so I'm commenting this out (in case it's required for part 2)
            // if *delta_x == 1 && grid[next_y][next_x] & LEFT > 0 {
            //     return false;
            // }
            // if *delta_x == -1 && grid[next_y][next_x] & RIGHT > 0 {
            //     return false;
            // }
            // if *delta_y == -1 && grid[next_y][next_x] & DOWN > 0 {
            //     return false;
            // }
            // if *delta_y == 1 && grid[next_y][next_x] & UP > 0 {
            //     return false;
            // }

            true
        })
        .map(|(delta_x, delta_y)| {
            (
                ((x as i32) + delta_x) as usize,
                (delta_y + (y as i32)) as usize,
            )
        })
        .collect();

    if new_positions.is_empty() {
        cache.insert((grid_cache_key, (x, y), minutes_elapsed), u32::MAX);
        return u32::MAX;
    }

    new_positions.sort_by(|a, b| {
        (end_pos.0.abs_diff(a.0) + end_pos.1.abs_diff(a.1))
            .cmp(&(end_pos.0.abs_diff(b.0) + end_pos.1.abs_diff(b.1)))
    });

    let mut min_time = u32::MAX;
    for (new_x, new_y) in new_positions {
        let time = solve(
            problem,
            (new_x, new_y),
            end_pos,
            minutes_elapsed + 1,
            current_min,
            cache,
            loop_grid_cache,
        );
        if time < min_time {
            min_time = time;
        }
    }

    cache.insert((grid_cache_key, (x, y), minutes_elapsed), min_time);
    min_time
}
