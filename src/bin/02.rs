enum Won {
    Me,
    Opponent,
    Tied,
}

#[derive(Clone, Copy)]
enum RockPapperScissor {
    Rock,
    Paper,
    Scissor,
}

#[inline]
fn who_won(opp: RockPapperScissor, me: RockPapperScissor) -> Won {
    match (opp, me) {
        (RockPapperScissor::Rock, RockPapperScissor::Rock) => Won::Tied,
        (RockPapperScissor::Rock, RockPapperScissor::Paper) => Won::Me,
        (RockPapperScissor::Rock, RockPapperScissor::Scissor) => Won::Opponent,
        (RockPapperScissor::Paper, RockPapperScissor::Rock) => Won::Opponent,
        (RockPapperScissor::Paper, RockPapperScissor::Paper) => Won::Tied,
        (RockPapperScissor::Paper, RockPapperScissor::Scissor) => Won::Me,
        (RockPapperScissor::Scissor, RockPapperScissor::Rock) => Won::Me,
        (RockPapperScissor::Scissor, RockPapperScissor::Paper) => Won::Opponent,
        (RockPapperScissor::Scissor, RockPapperScissor::Scissor) => Won::Tied,
    }
}

#[inline]
fn score_by_move(mov: RockPapperScissor) -> u32 {
    match mov {
        RockPapperScissor::Rock => 1,
        RockPapperScissor::Paper => 2,
        RockPapperScissor::Scissor => 3,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, round| {
        let mut iter = round.split_whitespace();

        let opp = match iter.next().unwrap() {
            "A" => RockPapperScissor::Rock,
            "B" => RockPapperScissor::Paper,
            "C" => RockPapperScissor::Scissor,
            _ => panic!("Mismatched value"),
        };

        let me = match iter.next().unwrap() {
            "X" => RockPapperScissor::Rock,
            "Y" => RockPapperScissor::Paper,
            "Z" => RockPapperScissor::Scissor,
            _ => panic!("Mismatched value"),
        };

        let score = match who_won(opp, me) {
            Won::Me => 6,
            Won::Opponent => 0,
            Won::Tied => 3,
        };

        acc + score + score_by_move(me)
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0_u32;

    input.lines().for_each(|round| {
        let mut iter = round.split_whitespace();

        let opp = match iter.next().unwrap() {
            "A" => RockPapperScissor::Rock,
            "B" => RockPapperScissor::Paper,
            "C" => RockPapperScissor::Scissor,
            _ => panic!("Mismatched value"),
        };

        match iter.next().unwrap() {
            "X" => match opp {
                RockPapperScissor::Rock => score += score_by_move(RockPapperScissor::Scissor),
                RockPapperScissor::Paper => score += score_by_move(RockPapperScissor::Rock),
                RockPapperScissor::Scissor => score += score_by_move(RockPapperScissor::Paper),
            },
            "Y" => score += score_by_move(opp) + 3,
            "Z" => {
                match opp {
                    RockPapperScissor::Rock => score += score_by_move(RockPapperScissor::Paper),
                    RockPapperScissor::Paper => score += score_by_move(RockPapperScissor::Scissor),
                    RockPapperScissor::Scissor => score += score_by_move(RockPapperScissor::Rock),
                };
                score += 6
            }
            _ => panic!("Mismatched value"),
        }
    });

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
