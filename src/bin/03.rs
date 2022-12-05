use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                let half = s.len() / 2;
                let (l, r) = s.split_at(half);
                let l: HashSet<char> = l
                    .trim()
                    .chars()
                    .filter(|v| v.is_ascii_alphanumeric())
                    .collect();
                let r: HashSet<char> = r
                    .trim()
                    .chars()
                    .filter(|v| v.is_ascii_alphanumeric())
                    .collect();
                l.intersection(&r).next().map_or_else(
                    || 0,
                    |v| {
                        if v.is_uppercase() {
                            *v as u32 - 'A' as u32 + 27
                        } else {
                            *v as u32 - 'a' as u32 + 1
                        }
                    },
                ) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut answers: Vec<u32> = Vec::new();
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next())
    {
        let first: HashSet<char> = first
            .trim()
            .chars()
            .filter(|v| v.is_ascii_alphanumeric())
            .collect();
        let second: HashSet<char> = second
            .trim()
            .chars()
            .filter(|v| v.is_ascii_alphanumeric())
            .collect();
        let third: HashSet<char> = third
            .trim()
            .chars()
            .filter(|v| v.is_ascii_alphanumeric())
            .collect();

        let other = [&second, &third];
        answers.push(
            first
                .iter()
                .find(|k| other.iter().all(|s| s.contains(k)))
                .map_or_else(
                    || 0,
                    |v| {
                        if v.is_uppercase() {
                            *v as u32 - 'A' as u32 + 27
                        } else {
                            *v as u32 - 'a' as u32 + 1
                        }
                    },
                ) as u32,
        );
    }
    Some(answers.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
