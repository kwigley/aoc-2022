use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 3;
    for g in input.as_bytes().windows(4) {
        count += 1;
        let set: HashSet<u8> = HashSet::from_iter(g.iter().cloned());
        if set.len() == 4 {
            break;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 13;
    for g in input.as_bytes().windows(14) {
        count += 1;
        let set: HashSet<u8> = HashSet::from_iter(g.iter().cloned());
        if set.len() == 14 {
            break;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
