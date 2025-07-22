pub fn part1(input: &str) -> i32 {
    input.lines().map(|line| line.parse::<i32>().unwrap()).sum()
}

pub fn part2(input: &str) -> i32 {
    let mut seen = std::collections::HashSet::new();
    let mut current_frequency = 0;
    seen.insert(current_frequency); // Start with the initial frequency

    for line in input.lines().cycle() {
        let change: i32 = line.parse::<i32>().unwrap();
        current_frequency += change;
        if seen.contains(&current_frequency) {
            return current_frequency;
        }
        seen.insert(current_frequency);
    }
    panic!("No frequency reached twice");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("+1\n-2\n+3\n+1\n"), 3);
        assert_eq!(part1("+1\n+1\n+1\n"), 3);
        assert_eq!(part1("+1\n+1\n-2\n"), 0);
        assert_eq!(part1("-1\n-2\n-3\n"), -6);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2("+1\n-2\n+3\n+1\n"), 2);
        assert_eq!(part2("+1\n-1\n"), 0);
        assert_eq!(part2("+3\n+3\n+4\n-2\n-4\n"), 10);
        assert_eq!(part2("-6\n+3\n+8\n+5\n-6\n"), 5);
        assert_eq!(part2("+7\n+7\n-2\n-7\n-4\n"), 14);
    }

    #[test]
    fn test_part1_input() {
        let input = utilities::get_input_contents(2018, 1);
        assert_eq!(part1(input.as_str()), 513);
    }

    #[test]
    fn test_part2_input() {
        let input = utilities::get_input_contents(2018, 1);
        assert_eq!(part2(input.as_str()), 287);
    }
}
