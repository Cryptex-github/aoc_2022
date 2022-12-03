#![feature(iter_array_chunks)]

use std::collections::HashMap;

fn gen_mapping() -> HashMap<char, u32> {
    let mut char_to_priority: HashMap<char, u32> = HashMap::with_capacity(26);

    for (i, v) in (1..=26).zip(97..=122) {
        unsafe {
            char_to_priority.insert(char::from_u32_unchecked(v), i as u32);
        }
    }

    char_to_priority
}

pub fn part_one(input: &str) -> Option<u32> {
    let char_to_priority = gen_mapping();

    let mut total_pri = 0_u32;

    input.lines().for_each(|e| {
        let (f, s) = e.split_at(e.len() / 2);

        for c in f.chars() {
            if s.contains(c) {
                let priority = char_to_priority.get(&c.to_ascii_lowercase()).expect("Character not in mapping.");

                if c.is_ascii_uppercase() {
                    total_pri += priority + 26;
                } else {
                    total_pri += priority;
                }
                break;
            }
        }
    });

    Some(total_pri)
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_to_priority = gen_mapping();

    let mut total_pri = 0_u32;

    for [x, y, z] in input.lines().array_chunks::<3>() {
        for c in x.chars() {
            if y.contains(c) && z.contains(c) {
                let priority = char_to_priority.get(&c.to_ascii_lowercase()).expect("Character not in mapping.");

                if c.is_ascii_uppercase() {
                    total_pri += priority + 26;
                } else {
                    total_pri += priority;
                }
                break;
            }
        }
    };

    Some(total_pri)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
