use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let moves = input.lines().map(|line| {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let num = chars.as_str().trim().parse::<u32>().unwrap();
        (dir, num)
    });
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let (mut hx, mut hy, mut tx, mut ty) = (1, 1, 0, 0);
    for mv in moves {
        for _ in 0..mv.1 {
            match mv.0 {
                'U' => hy += 1,
                'D' => hy -= 1,
                'R' => hx += 1,
                'L' => hx -= 1,
                _ => unreachable!(),
            };
            if hx - tx > 1 {
                tx += 1;
            }
            if hx - tx < -1 {
                tx -= 1;
            }
            if hy - ty > 1 {
                ty += 1;
            }
            if hy - ty < -1 {
                ty -= 1;
            }
            visited.insert((tx, ty));
        }
    }
    dbg!(&visited);
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
