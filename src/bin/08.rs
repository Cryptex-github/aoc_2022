use std::ops::ControlFlow;

#[inline]
fn get_trees(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|s| s.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = get_trees(input);

    let mut visible_count = 0;

    for (x, row) in trees.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            if x == 0 || y == 0 {
                visible_count += 1;

                continue;
            }

            let tree = *tree;

            if trees[..x].iter().all(|row| tree > row[y]) // up
                || trees[x + 1..].iter().all(|row| tree > row[y]) // down
                || row[..y].iter().all(|t| tree > *t) // left
                || row[y + 1..].iter().all(|t| tree > *t) // right
            {
                visible_count += 1;
            }
        }
    }

    Some(visible_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = get_trees(input);
    let mut highest_score = 0;

    for (x, row) in trees.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            if x == 0 || y == 0 || x == trees.len() - 1 || y == row.len() - 1 {
                continue;
            }

            let tree = *tree;

            let up = trees[..x].iter().rev().try_fold(0, |acc, row| {
                if tree > row[y] {
                    ControlFlow::Continue(acc + 1)
                } else {
                    ControlFlow::Break(acc + 1)
                }
            });

            let down = trees[x + 1..].iter().try_fold(0, |acc, row| {
                if tree > row[y] {
                    ControlFlow::Continue(acc + 1)
                } else {
                    ControlFlow::Break(acc + 1)
                }
            });

            let left = row[..y].iter().rev().try_fold(0, |acc, t| {
                if tree > *t {
                    ControlFlow::Continue(acc + 1)
                } else {
                    ControlFlow::Break(acc + 1)
                }
            });

            let right = row[y + 1..].iter().try_fold(0, |acc, t| {
                if tree > *t {
                    ControlFlow::Continue(acc + 1)
                } else {
                    ControlFlow::Break(acc + 1)
                }
            });

            match (up, down, left, right) {
                (
                    ControlFlow::Break(up) | ControlFlow::Continue(up),
                    ControlFlow::Break(down) | ControlFlow::Continue(down),
                    ControlFlow::Break(left) | ControlFlow::Continue(left),
                    ControlFlow::Break(right) | ControlFlow::Continue(right),
                ) => {
                    highest_score = highest_score.max(up * down * left * right);
                }
            }
        }
    }

    Some(highest_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
