pub mod args;
pub mod parser;
pub mod problem;
pub use nom;

pub fn wait_for_input() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}
