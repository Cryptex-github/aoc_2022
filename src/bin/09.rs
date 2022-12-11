#![feature(portable_simd)]

use std::simd::{Simd, SimdInt};

use ahash::{HashSet, HashSetExt};

fn get_directions(input: &str) -> impl Iterator<Item = (&str, u32)> + '_ {
    input.lines().map(|l| {
        let (d, c) = l.split_once(' ').unwrap();

        (d, c.parse::<u32>().unwrap())
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hx @ mut hy @ mut tx @ mut ty = 0_i32;
    let mut seen = HashSet::with_capacity(5);
    seen.insert((0, 0));

    for (d, c) in get_directions(input) {
        for _ in 0..c {
            match d {
                "L" => hx -= 1,
                "R" => hx += 1,
                "U" => hy += 1,
                "D" => hy -= 1,
                _ => unreachable!(),
            }            

            while (tx - hx).abs() > 1 || (ty - hy).abs() > 1 {
                let lanes = Simd::from_array([tx - hx, ty - hy]).abs();

                if lanes[0] > 0 {
                    if hx > tx { tx += 1 } else { tx -= 1 }
                }

                if lanes[1] > 0 {
                    if hy > ty { ty += 1 } else { ty -= 1 }
                }

                seen.insert((tx, ty));
            }
        }
    }

    Some(seen.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = [(0_i32, 0_i32); 10];
    let mut seen = HashSet::with_capacity(10);

    for (d, c) in get_directions(input) {
        for _ in 0..c {
            let (mut hx, mut hy) = rope[0];

            match d {
                "L" => hx -= 1,
                "R" => hx += 1,
                "U" => hy += 1,
                "D" => hy -= 1,
                _ => unreachable!(),
            }

            rope[0] = (hx, hy);

            for i in 1..rope.len() {
                let (prev_x, prev_y) = rope[i - 1];
                let (mut curr_x, mut curr_y) = rope[i];

                while (curr_x - prev_x).abs() > 1 || (curr_y - prev_y).abs() > 1 {
                    let lanes = Simd::from_array([curr_x - prev_x, curr_y - prev_y]).abs();

                    if lanes[0] > 0 {
                        if prev_x > curr_x { curr_x += 1 } else { curr_x -= 1 }
                    }
    
                    if lanes[1] > 0 {
                        if prev_y > curr_y { curr_y += 1 } else { curr_y -= 1 }
                    }
                }

                rope[i] = (curr_x, curr_y);
            }

            seen.insert(rope.last().copied().unwrap());
        }
    }

    dbg!(rope);

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
