#![feature(get_many_mut)]
#![feature(iter_array_chunks)]

use std::collections::VecDeque;

fn get_cargo(crates: &str) -> Vec<VecDeque<char>> {
    let mut cargo = Vec::<VecDeque<char>>::new();
    let mut exists = false;

    for c in crates.lines() {
        for (stack, [b, item, _, _]) in c
            .chars()
            .chain(std::iter::once(' '))
            .array_chunks::<4>()
            .enumerate()
        {
            if b == '[' {
                if !exists {
                    cargo.push(VecDeque::from([item]));
                } else {
                    cargo
                        .get_mut(stack)
                        .expect("Stack doesn't exist")
                        .push_back(item)
                }
            } else if item == ' ' {
                cargo.push(VecDeque::new());
            }
        }

        if !exists {
            exists = true;
        }
    }

    cargo
}

pub fn part_one(input: &str) -> Option<String> {
    let (crates, instrs) = input.split_once("\n\n").unwrap();

    let mut cargo = get_cargo(crates);

    for instr in instrs.lines().map(|instr| {
        instr
            .replace("move", "")
            .replace("from", "")
            .replace("to", "")
    }) {
        let mut iter = instr
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let count = iter.next().unwrap();
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        let [from, to] = cargo.get_many_mut([from - 1, to - 1]).unwrap();

        for _ in 0..count {
            to.push_front(from.pop_front().unwrap())
        }
    }

    Some(
        cargo
            .iter_mut()
            .filter_map(|x| x.pop_front())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (crates, instrs) = input.split_once("\n\n").unwrap();

    let mut cargo = get_cargo(crates);

    for instr in instrs.lines().map(|instr| {
        instr
            .replace("move", "")
            .replace("from", "")
            .replace("to", "")
    }) {
        let mut iter = instr
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let count = iter.next().unwrap();
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        let [from, to] = cargo.get_many_mut([from - 1, to - 1]).unwrap();

        let mut temp = VecDeque::with_capacity(count);

        for _ in 0..count {
            temp.push_back(from.pop_front().unwrap());
        }

        for item in temp.into_iter().rev() {
            to.push_front(item)
        }
    }

    Some(
        cargo
            .iter_mut()
            .filter_map(|x| x.pop_front())
            .collect::<String>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
