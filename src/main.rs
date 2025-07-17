mod y2018 {
    pub mod day1;
}

fn main() {
    println!("🎄 Advent of Code Solutions 🎄");

    // 2018 Day 1
    let day1_input: &'static str = include_str!("y2018/inputs/day1.txt");
    println!("\n=== 2018 Day 1 ===");
    println!("Part 1: {}", y2018::day1::part1(day1_input));
    println!("Part 2: {}", y2018::day1::part2(day1_input));
}
