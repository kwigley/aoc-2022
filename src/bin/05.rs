use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
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
    for crates in ship.split_at(num_of_stacks).0 {
        for (i, c) in crates.iter().enumerate() {
            if let Some(c) = c {
                stacks[i].push(*c);
            }
        }
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = moves.lines().map(|l| {
        let caps = re.captures(l).unwrap();
        (
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        )
    });
    for mv in moves {
        // count, from, to
        let elements = stacks[mv.1 - 1].drain(..mv.0).collect::<Vec<_>>();
        for element in &elements {
            stacks[mv.2 - 1].insert(0, *element)
        }
    }
    Some(
        stacks
            .iter()
            .map(|s| s.first().unwrap())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
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
    for crates in ship.split_at(num_of_stacks).0 {
        for (i, c) in crates.iter().enumerate() {
            if let Some(c) = c {
                stacks[i].push(*c);
            }
        }
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = moves.lines().map(|l| {
        let caps = re.captures(l).unwrap();
        (
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        )
    });
    for mv in moves {
        // count, from, to
        let elements = stacks[mv.1 - 1].drain(..mv.0).rev().collect::<Vec<_>>();
        for element in &elements {
            stacks[mv.2 - 1].insert(0, *element)
        }
    }
    Some(
        stacks
            .iter()
            .map(|s| s.first().unwrap())
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
