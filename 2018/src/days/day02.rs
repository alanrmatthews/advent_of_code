// TODO: Part 2 can be improved by going through each string and creating a substring with one letter removed.
// Add all of these to a set and when a duplicate is found, return it.
// This would reduce the time complexity significantly.
// The current implementation is O(n^2) due to the nested loops.
// The improved version would be O(n * m) where n is the number of strings and m is the length of the strings.

use crate::days::base_day::BaseDay;

pub struct Day02;

impl BaseDay for Day02 {
    fn day_number(&self) -> u8 {
        2
    }

    fn description(&self) -> &'static str {
        "Inventory Management System"
    }

    fn part1(&self, input: &str) -> String {
        let mut count_two = 0;
        let mut count_three = 0;

        for line in input.lines() {
            let (two, three) = self.check_box_id(line);
            count_two += two;
            count_three += three;
        }

        (count_two * count_three).to_string()
    }


   fn part2(&self, input: &str) -> String {
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

                if diff_count == 1
                    && let Some(diff_idx) = diff_index {
                        let common_id = id1.chars().enumerate()
                            .filter_map(|(k, c)| if k != diff_idx { Some(c) } else { None })
                            .collect();
                        return common_id;
                    }
            }
        }
        String::new()
    }
}

impl Day02 {
    fn check_box_id(&self, input: &str) -> (i32, i32) {
        let mut counts = std::collections::HashMap::new();
        for ch in input.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        let has_two = counts.values().any(|&count| count == 2) as i32;
        let has_three = counts.values().any(|&count| count == 3) as i32;

        (has_two, has_three)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utilities;

    const D: Day02 = Day02;

    #[test]
    fn test_part1() {
        assert_eq!(D.part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\naabcde\nababab"), "12");

        let input = utilities::get_input_contents(2018, 2);
        assert_eq!(D.part1(input.as_str()), "7872");
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(D.part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), "fgij");

        let input = utilities::get_input_contents(2018, 2);
        assert_eq!(D.part2(input.as_str()), "tjxmoewpdkyaihvrndfluwbzc");
    }
}
