use utils::{
    args::Args,
    parser::ParseInput,
    problem::{SolvePart1, SolvePart2},
};

fn main() {
    let args = utils::args::get_args();
    let input = utils::parser::read_file(args.my_input, args.day);

    let value = match args.day {
        17 => run(day17::Problem::default(), args, input),
        18 => run(day18::Problem::default(), args, input),
        19 => run(day19::Problem::default(), args, input),
        20 => run(day20::Problem::default(), args, input),
        21 => run(day21::Problem::default(), args, input),
        22 => run(day22::Problem::default(), args, input),
        23 => run(day23::Problem::default(), args, input),
        24 => run(day24::Problem::default(), args, input),
        25 => run(day25::Problem::default(), args, input),
        _ => panic!("Invalid day."),
    };

    println!("Solution: {}", value);
}

fn run<T>(
    mut problem: impl ParseInput<ParsedType = T>
        + SolvePart1<ParsedType = T>
        + SolvePart2<ParsedType = T>,
    args: Args,
    input: String,
) -> String {
    let parsed_input = problem.parse(input, args.part2);
    if args.part2 {
        problem.solve_part_two(parsed_input)
    } else {
        problem.solve_part_one(parsed_input)
    }
}
