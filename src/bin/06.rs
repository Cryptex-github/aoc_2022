use std::collections::VecDeque;

// What I would do if I can do it differently
fn _get_unique(input: &str, n: usize) -> Option<u32> {
    for (i, slice) in input.chars().collect::<Vec<_>>().windows(n).enumerate() {
        if !(1..n).any(|i| slice[i..].contains(&slice[i - 1])) {
            return Some((i + n) as u32);
        }
    }

    None
}

fn get_unique(input: &str, n: usize) -> Option<u32> {
    let mut queue = VecDeque::with_capacity(n);

    for (i, c) in (1..).zip(input.chars()) {
        if i <= n {
            queue.push_back(c)
        } else {
            queue.pop_front();
            queue.push_back(c)
        }

        if queue.len() == n {
            queue.make_contiguous();

            if !(1..n).any(|i| queue.as_slices().0[i..].contains(&queue.get(i - 1).unwrap())) {
                return Some(i as u32);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    get_unique(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    get_unique(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
