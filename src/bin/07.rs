use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut fs: Vec<(&str, u32)> = Vec::new();
    let mut stack = vec![("/", 0)];

    for line in input.lines() {
        let (first, rest) = line.split_once(' ').unwrap();
        match first {
            "$" => {
                let mut command = rest.split_terminator(' ');
                match command.next() {
                    Some("cd") => match command.next() {
                        Some("/") => continue,
                        Some("..") => fs.push(stack.pop().unwrap()),
                        Some(dir) => stack.push((dir, 0)),
                        _ => panic!("Invalid command"),
                    },
                    _ => continue,
                }
            }
            "dir" => continue,
            size => {
                for item in stack.iter_mut() {
                    let (_, cur_size) = item;
                    *cur_size += size.parse::<u32>().unwrap();
                }
            }
        }
    }
    fs.append(&mut stack);
    Some(
        fs.iter()
            .filter(|(_, size)| *size < 100_000)
            .map(|(_, size)| size)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut fs: Vec<(&str, u32)> = Vec::new();
    let mut stack = vec![("/", 0)];
    let mut total_size = 0;

    for line in input.lines() {
        let (first, rest) = line.split_once(' ').unwrap();
        match first {
            "$" => {
                let mut command = rest.split_terminator(' ');
                match command.next() {
                    Some("cd") => match command.next() {
                        Some("/") => continue,
                        Some("..") => fs.push(stack.pop().unwrap()),
                        Some(dir) => stack.push((dir, 0)),
                        _ => panic!("Invalid command"),
                    },
                    _ => continue,
                }
            }
            "dir" => continue,
            size => {
                let size = size.parse::<u32>().unwrap();
                for item in stack.iter_mut() {
                    let (_, cur_size) = item;
                    *cur_size += size;
                }
                total_size += size;
            }
        }
    }
    fs.append(&mut stack);
    let free_space = 70000000 - total_size;
    fs.iter()
        .map(|(_, size)| *size)
        .sorted()
        .find(|&size| free_space + size >= 30_000_000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
