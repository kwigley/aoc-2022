use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(4, input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(14, input))
}

fn solve(window_size: u32, input: &str) -> u32 {
    window_size
        + input
            .as_bytes()
            .windows(window_size as usize)
            .position(|w| {
                let set: HashSet<&u8> = HashSet::from_iter(w.iter());
                set.len() == window_size as usize
            })
            .expect("No solution found") as u32
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
        assert_eq!(part_two(&input), Some(19));
    }
}
