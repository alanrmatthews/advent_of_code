pub fn part1(input: &str) -> i32 {
    for line in input.lines() {
        let mut counts = std::collections::HashMap::new();
        for ch in line.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
    }
    return 0;
}

pub fn part2(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\naabcde\nababab"), 12);
    }

    // #[test]
    // fn test_part2_examples() {
    //     assert_eq!(part2("+1\n-2\n+3\n+1\n"), 2);
    //     assert_eq!(part2("+1\n-1\n"), 0);
    //     assert_eq!(part2("+3\n+3\n+4\n-2\n-4\n"), 10);
    //     assert_eq!(part2("-6\n+3\n+8\n+5\n-6\n"), 5);
    //     assert_eq!(part2("+7\n+7\n-2\n-7\n-4\n"), 14);
    // }

    #[test]
    fn test_part1_input() {
        let input = include_str!("inputs/day1.txt");
        assert_eq!(part1(input), 0);
    }

    // #[test]
    // fn test_part2_input() {
    //     let input = include_str!("inputs/day1.txt");
    //     assert_eq!(part2(input), 0);
    // }
}
