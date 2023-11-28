use crate::parser::{BluePrint, Resources, RobotCount};
use crate::parser::{ParsedOutput, Problem};
use rayon::prelude::*;
use std::collections::HashMap;
use utils::problem::SolvePart1;

impl SolvePart1 for Problem {
    type ParsedType = ParsedOutput;

    fn solve_part_one(&mut self, blueprints: ParsedOutput) -> String {
        let sol: u32 = get_max_geodes_per_blue_print(self, blueprints)
            .iter()
            .map(|(blueprint_id, max_geode)| blueprint_id * max_geode)
            .sum();
        sol.to_string()
    }
}

pub fn get_max_geodes_per_blue_print(
    problem: &mut Problem,
    blueprints: ParsedOutput,
) -> Vec<(u32, u32)> {
    let time = problem.time;
    blueprints
        .par_iter()
        .map(|b| {
            let max_robots = b.get_max_robots();

            let mut cur_max_per_time = vec![0_u32; time as usize + 1];

            let geode_count = run_minute(
                b,
                Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                RobotCount::init(),
                time,
                &mut HashMap::new(),
                &max_robots,
                true,
                true,
                true,
                true,
                &mut cur_max_per_time,
            )
            .0
            .geode;

            println!("geode_count: {:#?}", geode_count);

            (b.blueprint_id as u32, geode_count)
        })
        .collect()
}

fn get_cache_key(resources: Resources, robot_count: RobotCount, time: u8) -> u128 {
    let mut key: u128 = 0;
    key |= resources.ore as u128;
    key |= (resources.clay as u128) << 8;
    key |= (resources.obsidian as u128) << (8 * 2);
    key |= (resources.geode as u128) << (8 * 3);
    key |= (robot_count.ore as u128) << (8 * 4);
    key |= (robot_count.clay as u128) << (8 * 5);
    key |= (robot_count.obsidian as u128) << (8 * 6);
    key |= (robot_count.geode as u128) << (8 * 7);
    key |= (time as u128) << (8 * 8);

    key
}

fn run_minute(
    blueprint: &BluePrint,
    resources: Resources,
    robot_count: RobotCount,
    time: u8,
    cache: &mut HashMap<u128, (Resources, RobotCount)>,
    max_robots: &RobotCount,
    should_build_geode_robot: bool,
    should_build_obsidian_robot: bool,
    should_build_clay_robot: bool,
    should_build_ore_robot: bool,
    cur_max_per_time: &mut Vec<u32>,
) -> (Resources, RobotCount) {
    if time == 0 {
        return (resources, robot_count);
    }

    let cache_key = get_cache_key(resources, robot_count, time);

    // println!("{}", cache_key);

    if let Some(data) = cache.get(&cache_key) {
        return *data;
    }

    if resources.geode
        + (robot_count.geode as u32 * time as u32)
        + ((time as u32 * (time as u32 + 1)) / 2)
        <= cur_max_per_time[time as usize]
    {
        let mut new_resources = resources;
        new_resources.geode = 0;
        cache.insert(cache_key, (new_resources, robot_count));
        return (new_resources, robot_count);
    }

    let time = time - 1;

    let mut resource_results: Vec<(Resources, RobotCount)> = vec![];

    let geode_robots_maxed_out = robot_count.geode >= max_robots.geode;
    let obsidian_robots_maxed_out = robot_count.obsidian >= max_robots.obsidian;
    let clay_robots_maxed_out = robot_count.clay >= max_robots.clay;
    let ore_robots_maxed_out = robot_count.ore >= max_robots.ore;

    let can_build_geode_robot = can_build_geode_robot(blueprint, &resources)
        && !geode_robots_maxed_out
        && should_build_geode_robot;
    let can_build_obsidian_robot = can_build_obsidian_robot(blueprint, &resources)
        && !obsidian_robots_maxed_out
        && should_build_obsidian_robot;
    let can_build_clay_robot = can_build_clay_robot(blueprint, &resources)
        && !clay_robots_maxed_out
        && should_build_clay_robot;
    let can_build_ore_robot = can_build_ore_robot(blueprint, &resources)
        && !ore_robots_maxed_out
        && should_build_ore_robot;

    if can_build_geode_robot {
        let mut new_resources = resources;

        new_resources.ore -= blueprint.geode_robot_cost.ore;
        new_resources.clay -= blueprint.geode_robot_cost.clay;
        new_resources.obsidian -= blueprint.geode_robot_cost.obsidian;

        new_resources.ore += robot_count.ore as u32;
        new_resources.clay += robot_count.clay as u32;
        new_resources.obsidian += robot_count.obsidian as u32;
        new_resources.geode += robot_count.geode as u32;

        let mut new_robot_count = robot_count;
        new_robot_count.geode += 1;

        resource_results.push(run_minute(
            blueprint,
            new_resources,
            new_robot_count,
            time,
            cache,
            max_robots,
            true,
            true,
            true,
            true,
            cur_max_per_time,
        ));
    }

    if can_build_obsidian_robot {
        let mut new_resources = resources;

        new_resources.ore -= blueprint.obsidian_robot_cost.ore;
        new_resources.clay -= blueprint.obsidian_robot_cost.clay;
        new_resources.obsidian -= blueprint.obsidian_robot_cost.obsidian;

        new_resources.ore += robot_count.ore as u32;
        new_resources.clay += robot_count.clay as u32;
        new_resources.obsidian += robot_count.obsidian as u32;
        new_resources.geode += robot_count.geode as u32;

        let mut new_robot_count = robot_count;
        new_robot_count.obsidian += 1;

        resource_results.push(run_minute(
            blueprint,
            new_resources,
            new_robot_count,
            time,
            cache,
            max_robots,
            true,
            true,
            true,
            true,
            cur_max_per_time,
        ));
    }

    if can_build_clay_robot {
        let mut new_resources = resources;

        new_resources.ore -= blueprint.clay_robot_cost.ore;
        new_resources.clay -= blueprint.clay_robot_cost.clay;
        new_resources.obsidian -= blueprint.clay_robot_cost.obsidian;

        new_resources.ore += robot_count.ore as u32;
        new_resources.clay += robot_count.clay as u32;
        new_resources.obsidian += robot_count.obsidian as u32;
        new_resources.geode += robot_count.geode as u32;

        let mut new_robot_count = robot_count;
        new_robot_count.clay += 1;

        resource_results.push(run_minute(
            blueprint,
            new_resources,
            new_robot_count,
            time,
            cache,
            max_robots,
            true,
            true,
            true,
            true,
            cur_max_per_time,
        ));
    }

    if can_build_ore_robot {
        let mut new_resources = resources;

        new_resources.ore -= blueprint.ore_robot_cost.ore;
        new_resources.clay -= blueprint.ore_robot_cost.clay;
        new_resources.obsidian -= blueprint.ore_robot_cost.obsidian;

        new_resources.ore += robot_count.ore as u32;
        new_resources.clay += robot_count.clay as u32;
        new_resources.obsidian += robot_count.obsidian as u32;
        new_resources.geode += robot_count.geode as u32;

        let mut new_robot_count = robot_count;
        new_robot_count.ore += 1;

        resource_results.push(run_minute(
            blueprint,
            new_resources,
            new_robot_count,
            time,
            cache,
            max_robots,
            true,
            true,
            true,
            true,
            cur_max_per_time,
        ));
    }

    let mut new_resources = resources;

    new_resources.ore += robot_count.ore as u32;
    new_resources.clay += robot_count.clay as u32;
    new_resources.obsidian += robot_count.obsidian as u32;
    new_resources.geode += robot_count.geode as u32;

    resource_results.push(run_minute(
        blueprint,
        new_resources,
        robot_count,
        time,
        cache,
        max_robots,
        !can_build_geode_robot,
        !can_build_obsidian_robot,
        !can_build_clay_robot,
        !can_build_ore_robot,
        cur_max_per_time,
    ));

    resource_results.sort_by(|a, b| a.0.geode.cmp(&b.0.geode));
    let res = resource_results.pop().unwrap();

    cache.insert(cache_key, res);
    cur_max_per_time[time as usize] = std::cmp::max(cur_max_per_time[time as usize], res.0.geode);
    res
}

fn can_build_ore_robot(blueprint: &BluePrint, resources: &Resources) -> bool {
    blueprint.ore_robot_cost.ore <= resources.ore
        && blueprint.ore_robot_cost.clay <= resources.clay
        && blueprint.ore_robot_cost.obsidian <= resources.obsidian
}

fn can_build_clay_robot(blueprint: &BluePrint, resources: &Resources) -> bool {
    blueprint.clay_robot_cost.ore <= resources.ore
        && blueprint.clay_robot_cost.clay <= resources.clay
        && blueprint.clay_robot_cost.obsidian <= resources.obsidian
}

fn can_build_obsidian_robot(blueprint: &BluePrint, resources: &Resources) -> bool {
    blueprint.obsidian_robot_cost.ore <= resources.ore
        && blueprint.obsidian_robot_cost.clay <= resources.clay
        && blueprint.obsidian_robot_cost.obsidian <= resources.obsidian
}

fn can_build_geode_robot(blueprint: &BluePrint, resources: &Resources) -> bool {
    blueprint.geode_robot_cost.ore <= resources.ore
        && blueprint.geode_robot_cost.clay <= resources.clay
        && blueprint.geode_robot_cost.obsidian <= resources.obsidian
}
