pub mod args;
pub mod parser;
pub mod problem;
pub use nom;
pub use rayon;

pub fn wait_for_input() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

pub fn clear_screen() {
    // print!("{}[2J", 27 as char);
    print!("\x1B[2J\x1B[1;1H");
}
