pub trait BaseDay {
    fn day_number(&self) -> u8;
    fn description(&self) -> &'static str;
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}