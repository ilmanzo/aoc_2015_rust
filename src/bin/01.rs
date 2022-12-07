pub fn part_one(input: &str) -> Option<usize> {
    let a = input.matches('(').count();
    let b = input.matches(')').count();
    Some(a - b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => result += 1,
            ')' => result -= 1,
            _ => (),
        }
        if result == -1 {
            return Some(1 + i);
        };
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use advent_of_code::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        use advent_of_code::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), Some(1));
    }
}
