use std::collections::{BTreeSet, HashMap};

use utils::{clear_screen, problem::SolvePart1, wait_for_input};

use crate::parser::{Direction, ParsedOutput, Problem};

// (bool, Vec<(isize, isize)>) => (cant_move, old_positions)
type NewPositionToOldPositionsMap = HashMap<(isize, isize), (bool, Vec<(isize, isize)>)>;

#[allow(dead_code)]
pub fn print_map(min: (isize, isize), max: (isize, isize), elv_pos: &BTreeSet<(isize, isize)>) {
    clear_screen();
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if elv_pos.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    wait_for_input();
}

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, parsed_input: ParsedOutput) -> String {
        let (empty_spaces, _) = solve(self, parsed_input, 10);
        empty_spaces.to_string()
    }
}

pub fn get_new_position_for_elf(
    (x, y): (isize, isize),
    elv_pos: &BTreeSet<(isize, isize)>,
    new_positions: &mut NewPositionToOldPositionsMap,
    problem: &Problem,
) {
    let ne = (x + 1, y - 1);
    let n = (x, y - 1);
    let nw = (x - 1, y - 1);
    let w = (x - 1, y);
    let sw = (x - 1, y + 1);
    let s = (x, y + 1);
    let se = (x + 1, y + 1);
    let e = (x + 1, y);

    let ne_present = elv_pos.contains(&ne);
    let n_present = elv_pos.contains(&n);
    let nw_present = elv_pos.contains(&nw);
    let w_present = elv_pos.contains(&w);
    let sw_present = elv_pos.contains(&sw);
    let s_present = elv_pos.contains(&s);
    let se_present = elv_pos.contains(&se);
    let e_present = elv_pos.contains(&e);

    if !ne_present
        && !n_present
        && !nw_present
        && !w_present
        && !sw_present
        && !s_present
        && !se_present
        && !e_present
    {
        new_positions.insert((x, y), (true, vec![(x, y)]));
        return;
    }

    for i in 0..4 {
        let direction = problem
            .checking_order
            .get((problem.checking_order_start_index + i) % problem.checking_order.len())
            .unwrap();
        let (can_move_in_dir, new_point) = match direction {
            Direction::North => (!ne_present && !n_present && !nw_present, n),
            Direction::South => (!sw_present && !s_present && !se_present, s),
            Direction::East => (!ne_present && !e_present && !se_present, e),
            Direction::West => (!nw_present && !w_present && !sw_present, w),
        };

        if can_move_in_dir {
            match new_positions.get_mut(&new_point) {
                Some((_, v)) => v.push((x, y)),
                None => {
                    new_positions.insert(new_point, (false, vec![(x, y)]));
                }
            };
            return;
        }
    }
    new_positions.insert((x, y), (true, vec![(x, y)]));
}

pub fn solve(
    problem: &mut Problem,
    (mut min_point, mut max_point, mut elv_pos): ParsedOutput,
    run_till_index: usize,
    // (empty_positions, round)
) -> (isize, usize) {
    let mut new_positions: NewPositionToOldPositionsMap = HashMap::new();
    let mut round = 0;
    loop {
        elv_pos.iter().for_each(|a| {
            let x = a.0;
            let y = a.1;

            get_new_position_for_elf((x, y), &elv_pos, &mut new_positions, problem);
        });

        problem.checking_order_start_index = (problem.checking_order_start_index + 1) % 4;

        let mut something_moved = false;

        new_positions
            .iter()
            .for_each(|(new_point, (cant_move, old_positions))| {
                something_moved = !cant_move || something_moved;
                if old_positions.len() == 1 && !cant_move {
                    elv_pos.remove(&old_positions[0]);
                    elv_pos.insert(*new_point);

                    min_point.0 = min_point.0.min(new_point.0);
                    min_point.1 = min_point.1.min(new_point.1);
                    max_point.0 = max_point.0.max(new_point.0);
                    max_point.1 = max_point.1.max(new_point.1);
                }
            });

        // print_map(min_point, max_point, &elv_pos);

        round += 1;

        if !something_moved || run_till_index == round {
            break;
        }

        new_positions.drain();
    }

    // println!("{:#?}", elv_pos);

    let total_empty_positions = ((max_point.0 - min_point.0 + 1) * (max_point.1 - min_point.1 + 1))
        - elv_pos.len() as isize;

    (total_empty_positions, round)
}
