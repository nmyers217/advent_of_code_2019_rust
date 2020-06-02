#![warn(clippy::all)]

use colored::*;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn main() {
    println!("{}", "Day 01".green().bold().underline());
    day_01::solve();
    println!();

    println!("{}", "Day 02".green().bold().underline());
    day_02::solve();
    println!();

    println!("{}", "Day 03".green().bold().underline());
    day_03::solve();
    println!();

    println!("{}", "Day 04".green().bold().underline());
    day_04::solve();
    println!();

    println!("{}", "Day 05".green().bold().underline());
    day_05::solve();
    println!();

    println!("{}", "Day 06".green().bold().underline());
    day_06::solve();
    println!();
}
