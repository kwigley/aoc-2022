pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|s| get_score(s.trim())).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|s| get_score(get_move(s.trim()))).sum())
}

fn get_score(s: &str) -> u32 {
    match s {
        "A Y" => 8,
        "A Z" => 3,
        "A X" => 4,
        "B X" => 1,
        "B Z" => 9,
        "B Y" => 5,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => {
            println!("Unknown line: {}", s);
            0
        }
    }
}

fn get_move(s: &str) -> &str {
    match s {
        "A Y" => "A X",
        "A Z" => "A Y",
        "A X" => "A Z",
        "B X" => "B X",
        "B Z" => "B Z",
        "B Y" => "B Y",
        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
        _ => {
            println!("Unknown line: {}", s);
            "?"
        }
    }
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
