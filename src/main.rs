use clap::Parser;
use sortingalgo::UserInput;

pub mod sorting;

fn main() {
    let mut _args = UserInput::parse();
    sorting::sort(&mut _args)
}