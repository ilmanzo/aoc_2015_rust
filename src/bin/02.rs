pub fn part_one(input: &str) -> Option<usize> {
    let mut sum: usize = 0;
    for line in input.lines() {
        let mut dims: Vec<usize> = line.split('x').map(|x| x.parse().unwrap()).collect();
        dims.sort();
        let a = dims[0] * dims[1];
        let b = dims[1] * dims[2];
        let c = dims[0] * dims[2];
        sum += 3 * a + 2 * b + 2 * c
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut ribbon: usize = 0;
    for line in input.lines() {
        let mut dims: Vec<usize> = line.split('x').map(|x| x.parse().unwrap()).collect();
        dims.sort();
        ribbon += 2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2]
    }
    Some(ribbon)
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
        use advent_of_code::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), Some(58));
    }

    #[test]
    fn test_part_two() {
        use advent_of_code::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), Some(34));
    }
}
