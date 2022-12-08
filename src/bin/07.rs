use ahash::{HashMap, HashMapExt};

use relative_path::RelativePathBuf;

fn get_size_map(input: &str) -> HashMap<RelativePathBuf, u64> {
    let mut current_path = RelativePathBuf::new();
    let mut sizes = HashMap::<RelativePathBuf, u64>::with_capacity(3);

    for line in input.lines() {
        if line.starts_with("$") {
            let mut args = line.split(' ');
            args.next();

            let cmd = args.next().unwrap();
            let arg = args.next().unwrap_or_default();

            match cmd {
                "cd" => match arg {
                    "/" => current_path = RelativePathBuf::new(),
                    _ => {
                        current_path = current_path.join_normalized(arg);
                    }
                },
                "ls" => continue,
                _ => panic!("Unknown instruction"),
            }
        }

        let (pre, _) = line.split_once(' ').unwrap();

        if let Ok(size) = pre.parse::<u64>() {
            sizes
                .entry(current_path.clone())
                .and_modify(|x| *x += size)
                .or_insert(size);

            let mut current = current_path.as_relative_path();

            loop {
                if let Some(parent) = current.parent() {
                    sizes
                        .entry(parent.to_owned())
                        .and_modify(|x| *x += size)
                        .or_insert(size);

                    current = parent;
                } else {
                    break;
                }
            }
        }
    }

    sizes
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        get_size_map(input)
            .into_iter()
            .filter_map(|(_, s)| if s <= 100_000 { Some(s) } else { None })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let sizes = get_size_map(input);
    let unused = 70_000_000 - sizes.get(&RelativePathBuf::from("")).unwrap();
    let needs = 30_000_000;
    let mut min = u64::MAX;

    for (_, s) in sizes {
        if s + unused >= needs {
            min = min.min(s);
        }
    }

    Some(min)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
