use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let mut state = HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for c in input.chars() {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            'v' => y += 1,
            '^' => y -= 1,
            _ => (),
        }
        state.insert((x, y));
    }
    Some(state.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut state = HashSet::new();
    let mut x: [i32; 2] = [0; 2];
    let mut y: [i32; 2] = [0; 2];
    let mut turn = 0;
    for c in input.chars() {
        match c {
            '>' => x[turn] += 1,
            '<' => x[turn] -= 1,
            'v' => y[turn] += 1,
            '^' => y[turn] -= 1,
            _ => (),
        }
        state.insert((x[turn], y[turn]));
        turn = 1 - turn;
    }
    Some(state.len())
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
        use advent_of_code::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        use advent_of_code::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), Some(10));
    }
}
