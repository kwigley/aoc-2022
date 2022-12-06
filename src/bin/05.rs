use regex::Regex;

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}
type CrateStacks = Vec<Vec<char>>;
type Moves = Vec<Move>;

struct ShipCargo {
    stacks: CrateStacks,
    moves: Moves,
}

fn parse_input(input: &str) -> ShipCargo {
    let mut split = input.split_terminator("\n\n");
    let (stacks, moves) = (split.next().unwrap(), split.next().unwrap());
    let ship = stacks
        .lines()
        .map(|l| {
            l.as_bytes()
                .chunks(4)
                .map(|s| {
                    match std::str::from_utf8(s)
                        .unwrap()
                        .trim()
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                    {
                        "" => None,
                        c => Some(c.chars().next().unwrap()),
                    }
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<_>>();
    let num_of_stacks = ship[0].len();
    let mut stacks = vec![vec![]; num_of_stacks];
    for crates in &ship[..num_of_stacks] {
        for (i, c) in crates.iter().enumerate() {
            if let Some(c) = c {
                stacks[i].push(*c);
            }
        }
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = moves
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Move {
                amount: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                from: caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                to: caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            }
        })
        .collect();
    ShipCargo { stacks, moves }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut ship = parse_input(input);
    for mv in ship.moves {
        let crates = ship.stacks[mv.from]
            .drain(..mv.amount)
            .rev()
            .collect::<Vec<_>>();
        ship.stacks[mv.to].splice(0..0, crates.clone());
    }
    ship.stacks
        .iter()
        .map(|s| s.first())
        .collect::<Option<String>>()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut ship = parse_input(input);
    for mv in ship.moves {
        let crates = ship.stacks[mv.from].drain(..mv.amount).collect::<Vec<_>>();
        ship.stacks[mv.to].splice(0..0, crates.clone());
    }
    ship.stacks
        .iter()
        .map(|s| s.first())
        .collect::<Option<String>>()
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
