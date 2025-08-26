use crate::days::base_day::BaseDay;

pub struct Day01;

impl BaseDay for Day01 {
    fn day_number(&self) -> u8 {
        1
    }

    fn description(&self) -> &'static str {
        "Chronal Calibration"
    }

    fn part1(&self, input: &str) -> String {
        input.lines().map(|line| line.parse::<i32>().unwrap()).sum::<i32>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut seen = std::collections::HashSet::new();
        let mut current_frequency = 0;
        seen.insert(current_frequency); // Start with the initial frequency

        for line in input.lines().cycle() {
            let change: i32 = line.parse::<i32>().unwrap();
            current_frequency += change;
            if seen.contains(&current_frequency) {
                return current_frequency.to_string();
            }
            seen.insert(current_frequency);
        }
        panic!("No frequency reached twice");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities;

    #[test]
    fn test_part1_examples() {
        let d = Day01;
        assert_eq!(d.part1("+1\n-2\n+3\n+1\n"), "3");
        assert_eq!(d.part1("+1\n+1\n+1\n"), "3");
        assert_eq!(d.part1("+1\n+1\n-2\n"), "0");
        assert_eq!(d.part1("-1\n-2\n-3\n"), "-6");

        let input = utilities::get_input_contents(2018, 1);
        assert_eq!(d.part1(input.as_str()), "513");
    }

    #[test]
    fn test_part2_examples() {
        let d = Day01;
        assert_eq!(d.part2("+1\n-2\n+3\n+1\n"), "2");
        assert_eq!(d.part2("+1\n-1\n"), "0");
        assert_eq!(d.part2("+3\n+3\n+4\n-2\n-4\n"), "10");
        assert_eq!(d.part2("-6\n+3\n+8\n+5\n-6\n"), "5");
        assert_eq!(d.part2("+7\n+7\n-2\n-7\n-4\n"), "14");

        let input = utilities::get_input_contents(2018, 1);
        assert_eq!(d.part2(input.as_str()), "287");
    }
}
