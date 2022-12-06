use gcollections::ops::*;
use interval::ops::*;
use interval::Interval;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut ranges = l.split_terminator(',').map(|r| {
                    let mut range = r.split_terminator('-');
                    Interval::new(
                        range.next().unwrap().parse::<u32>().unwrap(),
                        range.next().unwrap().parse::<u32>().unwrap(),
                    )
                });
                let first = ranges.next().unwrap();
                let second = ranges.next().unwrap();
                first.is_subset(&second) || second.is_subset(&first)
            })
            .filter(|v| *v)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut ranges = l.split_terminator(',').map(|r| {
                    let mut range = r.split_terminator('-');
                    Interval::new(
                        range.next().unwrap().parse::<u32>().unwrap(),
                        range.next().unwrap().parse::<u32>().unwrap(),
                    )
                });
                let first = &ranges.next().unwrap();
                let second = &ranges.next().unwrap();
                first.overlap(second)
            })
            .filter(|v| *v)
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
