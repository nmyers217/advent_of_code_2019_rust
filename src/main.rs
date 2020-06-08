#![warn(clippy::all)]

use colored::*;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod intcode;
mod linear_algebra;

macro_rules! solve {
    ($title: literal, $module:tt) => {
        println!("{}", $title.green().bold().underline());
        $module::solve();
        println!();
    };
}

fn main() {
    solve!("Day 01", day_01);
    solve!("Day 02", day_02);
    solve!("Day 03", day_03);
    solve!("Day 04", day_04);
    solve!("Day 05", day_05);
    solve!("Day 06", day_06);
    solve!("Day 07", day_07);
    solve!("Day 08", day_08);
    solve!("Day 09", day_09);
}
