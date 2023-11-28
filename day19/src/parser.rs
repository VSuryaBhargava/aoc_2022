use utils::{
    nom::{
        branch::permutation,
        bytes::complete::tag,
        character::complete::space0,
        combinator::{all_consuming, map, opt},
        multi::separated_list0,
        sequence::tuple,
        IResult,
    },
    parser::{parse_number_u32, parse_number_u8, ParseInput},
};

#[derive(Default)]
pub struct Problem {
    pub time: u8,
}

impl ParseInput for Problem {
    type ParsedType = ParsedOutput;
    fn parse(&mut self, input: String, part2: bool) -> ParsedOutput {
        let mut blueprints = Vec::new();

        self.time = if part2 { 32 } else { 24 };

        input.lines().for_each(|l| {
            let _ = map(all_consuming(parse_line), |cube| {
                blueprints.push(cube);
            })(l);
        });

        blueprints
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resources {
    pub ore: u32,
    pub clay: u32,
    pub obsidian: u32,
    pub geode: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BluePrint {
    pub ore_robot_cost: Resources,
    pub clay_robot_cost: Resources,
    pub obsidian_robot_cost: Resources,
    pub geode_robot_cost: Resources,
    pub blueprint_id: u8,
}

impl BluePrint {
    pub fn get_max_robots(&self) -> RobotCount {
        let mut robot_count = RobotCount {
            clay: 0,
            geode: 0,
            obsidian: 0,
            ore: 0,
        };

        robot_count.ore = std::cmp::max(robot_count.ore as u32, self.ore_robot_cost.ore) as u8;
        robot_count.ore = std::cmp::max(robot_count.ore as u32, self.clay_robot_cost.ore) as u8;
        robot_count.ore = std::cmp::max(robot_count.ore as u32, self.obsidian_robot_cost.ore) as u8;
        robot_count.ore = std::cmp::max(robot_count.ore as u32, self.geode_robot_cost.ore) as u8;

        robot_count.clay = std::cmp::max(robot_count.clay as u32, self.ore_robot_cost.clay) as u8;
        robot_count.clay = std::cmp::max(robot_count.clay as u32, self.clay_robot_cost.clay) as u8;
        robot_count.clay =
            std::cmp::max(robot_count.clay as u32, self.obsidian_robot_cost.clay) as u8;
        robot_count.clay = std::cmp::max(robot_count.clay as u32, self.geode_robot_cost.clay) as u8;

        robot_count.obsidian =
            std::cmp::max(robot_count.obsidian as u32, self.ore_robot_cost.obsidian) as u8;
        robot_count.obsidian =
            std::cmp::max(robot_count.obsidian as u32, self.clay_robot_cost.obsidian) as u8;
        robot_count.obsidian = std::cmp::max(
            robot_count.obsidian as u32,
            self.obsidian_robot_cost.obsidian,
        ) as u8;
        robot_count.obsidian =
            std::cmp::max(robot_count.obsidian as u32, self.geode_robot_cost.obsidian) as u8;

        robot_count.geode = u8::MAX;

        robot_count
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RobotCount {
    pub ore: u8,
    pub clay: u8,
    pub obsidian: u8,
    pub geode: u8,
}

impl RobotCount {
    pub fn init() -> RobotCount {
        RobotCount {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

pub type ParsedOutput = Vec<BluePrint>;

fn parse_blueprint_id(input: &str) -> IResult<&str, u8> {
    let (input, (_, blueprint_id, _)) =
        tuple((tag("Blueprint "), parse_number_u8, tag(":")))(input)?;
    Ok((input, blueprint_id))
}

fn parse_ore_cost(input: &str) -> IResult<&str, u32> {
    // costs 2 ore
    let (input, (ore_cost, _)) = tuple((parse_number_u32, tag(" ore")))(input)?;
    Ok((input, ore_cost))
}

fn parse_clay_cost(input: &str) -> IResult<&str, u32> {
    // costs 2 ore
    let (input, (ore_cost, _)) = tuple((parse_number_u32, tag(" clay")))(input)?;
    Ok((input, ore_cost))
}

fn parse_obsidian_cost(input: &str) -> IResult<&str, u32> {
    // costs 2 ore
    let (input, (ore_cost, _)) = tuple((parse_number_u32, tag(" obsidian")))(input)?;
    Ok((input, ore_cost))
}

fn parse_cost(input: &str) -> IResult<&str, Resources> {
    // costs 2 ore
    // costs 3 ore and 14 clay
    let (input, costs) = separated_list0(
        tag(" and "),
        permutation((
            opt(parse_ore_cost),
            opt(parse_clay_cost),
            opt(parse_obsidian_cost),
        )),
    )(input)?;

    let mut resource_cost = Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    costs.iter().for_each(|cost| {
        if let Some(ore_cost) = cost.0 {
            resource_cost.ore = ore_cost;
        }
        if let Some(clay_cost) = cost.1 {
            resource_cost.clay = clay_cost;
        }
        if let Some(obsidian_cost) = cost.2 {
            resource_cost.obsidian = obsidian_cost;
        }
    });

    Ok((input, resource_cost))
}

fn parse_ore_robot_cost(input: &str) -> IResult<&str, Resources> {
    // Each ore robot costs 2 ore.
    let (input, (_, _, robot_cost, _)) =
        tuple((space0, tag("Each ore robot costs "), parse_cost, tag(".")))(input)?;
    Ok((input, robot_cost))
}

fn parse_clay_robot_cost(input: &str) -> IResult<&str, Resources> {
    // Each clay robot costs 2 ore.
    let (input, (_, _, robot_cost, _)) =
        tuple((space0, tag("Each clay robot costs "), parse_cost, tag(".")))(input)?;
    Ok((input, robot_cost))
}

fn parse_obsidian_robot_cost(input: &str) -> IResult<&str, Resources> {
    // Each obsidian robot costs 2 ore.
    let (input, (_, _, robot_cost, _)) = tuple((
        space0,
        tag("Each obsidian robot costs "),
        parse_cost,
        tag("."),
    ))(input)?;
    Ok((input, robot_cost))
}

fn parse_geode_robot_cost(input: &str) -> IResult<&str, Resources> {
    // Each geode robot costs 2 ore.
    let (input, (_, _, robot_cost, _)) =
        tuple((space0, tag("Each geode robot costs "), parse_cost, tag(".")))(input)?;
    Ok((input, robot_cost))
}

fn parse_line(input: &str) -> IResult<&str, BluePrint> {
    // Blueprint 3: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 14 obsidian.
    let (input, blueprint_id) = parse_blueprint_id(input)?;

    let (input, (ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost)) =
        permutation((
            parse_ore_robot_cost,
            parse_clay_robot_cost,
            parse_obsidian_robot_cost,
            parse_geode_robot_cost,
        ))(input)?;

    let blueprint = BluePrint {
        ore_robot_cost,
        clay_robot_cost,
        obsidian_robot_cost,
        geode_robot_cost,
        blueprint_id,
    };

    Ok((input, blueprint))
}
