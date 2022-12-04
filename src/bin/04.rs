fn get_idx(pair: &str) -> (u32, u32, u32, u32) {
    let mut splitted = pair.split(',');
    let first = splitted.next().unwrap();
    let second = splitted.next().unwrap();

    let mut first_splitted = first.split('-').map(|x| x.parse::<u32>().unwrap());
    let mut sec_splitted = second.split('-').map(|x| x.parse::<u32>().unwrap());

    (
        first_splitted.next().unwrap(),
        first_splitted.next().unwrap(),
        sec_splitted.next().unwrap(),
        sec_splitted.next().unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, pair| {
        let (first_idx_begin, first_idx_end, sec_idx_begin, sec_idx_end) = get_idx(pair);

        if (first_idx_begin <= sec_idx_begin && first_idx_end >= sec_idx_end)
            || (sec_idx_begin <= first_idx_begin && sec_idx_end >= first_idx_end)
        {
            acc + 1
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, pair| {
        let (first_idx_begin, first_idx_end, sec_idx_begin, sec_idx_end) = get_idx(pair);

        if first_idx_begin <= sec_idx_end && first_idx_end >= sec_idx_begin {
            acc + 1
        } else {
            acc
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
