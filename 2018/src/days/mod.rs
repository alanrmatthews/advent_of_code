use crate::utilities;
use crate::days::base_day::BaseDay;
use crate::days::day01::Day01;
use crate::days::day02::Day02;

mod base_day;
mod day01;
mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

pub fn run_2018() {
    println!("\n=== 2018 Solutions ===");

    println!("Day 1: Chronal Calibration");
    let day1_input = utilities::get_input_contents(2018, 1);
    let day1 = Day01;
    println!("Part 1: {}", day1.part1(day1_input.as_str()));
    println!("Part 2: {}", day1.part2(day1_input.as_str()));
    println!();

    println!("Day 2: Inventory Management System");
    let day2_input = utilities::get_input_contents(2018, 2);
    let day2 = Day02;
    println!("Part 1: {}", day2.part1(day2_input.as_str()));
    println!("Part 2: {}", day2.part2(day2_input.as_str()));
    println!();

    // day03::run();
    // day04::run();
    // day05::run();
    // day06::run();
    // day07::run();
    // day08::run();
    // day09::run();
    // day10::run();
    // day11::run();
    // day12::run();
    // day13::run();
    // day14::run();
    // day15::run();
    // day16::run();
    // day17::run();
    // day18::run();
    // day19::run();
    // day20::run();
    // day21::run();
    // day22::run();
    // day23::run();
    // day24::run();
    // day25::run();
}
