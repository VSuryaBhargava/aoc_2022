use clap::Parser;
use clap_num::number_range;

fn valid_day(s: &str) -> Result<u8, String> {
    number_range(s, 17, 25)
}

/// Problem in aoc 2022
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Run part 2
    #[arg(long, default_value_t = false)]
    pub part2: bool,

    /// Run using sample input
    #[arg(long, default_value_t = false)]
    pub my_input: bool,

    /// Day
    #[arg(long, value_parser=valid_day)]
    pub day: u8,
}

pub fn get_args() -> Args {
    Args::parse()
}
