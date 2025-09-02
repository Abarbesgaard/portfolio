pub mod commands;
pub mod cv;
pub mod start;

fn main() {
    commands::commands::Args::execute();
}
