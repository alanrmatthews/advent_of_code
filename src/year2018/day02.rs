// TODO: Part 2 can be improved by going through each string and creating a substring with one letter removed.
// Add all of these to a set and when a duplicate is found, return it.
// This would reduce the time complexity significantly.
// The current implementation is O(n^2) due to the nested loops.
// The improved version would be O(n * m) where n is the number of strings and m is the length of the strings.

pub fn part1(input: &str) -> i32 {
    let mut count_two = 0;
    let mut count_three = 0;

    for line in input.lines() {
        let (two, three) = check_box_id(line);
        count_two += two;
        count_three += three;
    }

    return count_two * count_three;
}

pub fn part2(input: &str) -> String {
    let ids: Vec<&str> = input.lines().collect();
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            let id1 = ids[i];
            let id2 = ids[j];
            let mut diff_count = 0;
            let mut diff_index = None;

            for (k, (c1, c2)) in id1.chars().zip(id2.chars()).enumerate() {
                if c1 != c2 {
                    diff_count += 1;
                    diff_index = Some(k);
                }
                if diff_count > 1 {
                    break;
                }
            }

            if diff_count == 1 {
                if let Some(diff_idx) = diff_index {
                    let common_id = id1.chars().enumerate()
                        .filter_map(|(k, c)| if k != diff_idx { Some(c) } else { None })
                        .collect();
                    return common_id;
                }
            }
        }
    }
    String::new()
}

fn check_box_id(input: &str) -> (i32, i32) {
    let mut counts = std::collections::HashMap::new();
    for ch in input.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    let has_two = counts.values().any(|&count| count == 2) as i32;
    let has_three = counts.values().any(|&count| count == 3) as i32;

    (has_two, has_three)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\naabcde\nababab"), 12);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), "fgij");
    }

    #[test]
    fn test_part1_input() {
        let input = include_str!("inputs/day2.txt");
        assert_eq!(part1(input), 7872);
    }

    #[test]
    fn test_part2_input() {
        let input = include_str!("inputs/day2.txt");
        assert_eq!(part2(input), "tjxmoewpdkyaihvrndfluwbzc");
    }
}
