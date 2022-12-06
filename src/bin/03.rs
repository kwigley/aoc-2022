use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                let half = s.len() / 2;
                let (l, r) = s.split_at(half);
                l.chars()
                    .collect::<HashSet<char>>()
                    .intersection(&r.chars().collect::<HashSet<char>>())
                    .next()
                    .map_or_else(
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
    Some(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|g| {
                let g = g.collect::<Vec<_>>();
                let l = g[0].chars().collect::<HashSet<char>>();
                let r = g[1].chars().collect::<HashSet<char>>();
                let u = g[2].chars().collect::<HashSet<char>>();
                l.iter()
                    .find(|k| [&r, &u].iter().all(|s| s.contains(k)))
                    .map_or_else(
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
