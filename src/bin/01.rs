pub fn part_one(_input: &str) -> Option<u32> {
    None // did not save part 1 solution
}

pub fn part_two(input: &str) -> Option<u32> {
    let [mut current, mut prev, mut biggest, mut sec_biggest, mut third_biggest] = [0_u32; 5];

    input
        .lines()
        .chain(std::iter::once(""))
        .for_each(|e| {
            if !e.is_empty() {
                current += e.parse::<u32>().unwrap();
            } else {
                if current > biggest {
                    biggest = current;
                }

                if current > sec_biggest && current < biggest {
                    sec_biggest = current;
                }

                if current > third_biggest && current < sec_biggest {
                    third_biggest = current;
                }

                prev = current;
                current = 0;
            }
        });
    
    Some(biggest + sec_biggest + third_biggest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 1);
    //     assert_eq!(part_two(&input), Some(45000));
    // }
}
