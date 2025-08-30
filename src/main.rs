pub mod commands;
pub mod cv;
pub mod ui;

fn main() {
    commands::commands::Args::execute();
}

