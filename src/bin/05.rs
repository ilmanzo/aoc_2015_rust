use std::str;

fn vowels(input: &str) -> usize {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    input.chars().filter(|c| vowels.contains(c)).count()
}

fn twice(input: &str) -> bool {
    for i in 0..(input.len() - 1) {
        if input.chars().nth(i) == input.chars().nth(i + 1) {
            return true;
        }
    }

    false
}

fn banned(input: &str) -> bool {
    input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
}

fn isnice_pt1(input: &str) -> bool {
    vowels(input) >= 3 && twice(input) && !banned(input)
}

fn pairs(input: &str) -> bool {
    // check for pair of any two letters that appears at least twice
    let indices: Vec<_> = input.char_indices().map(|(i, _)| i).collect();

    for i in 0..=(indices.len() - 4) {
        let pair = &input[indices[i]..indices[i + 2]];
        let tail = &input[indices[i + 2]..];

        if tail.contains(pair) {
            return true;
        }
    }

    false
}
fn repeat_between(input: &str) -> bool {
    input
        .chars()
        .zip(input.chars().skip(2))
        .any(|(a, b)| a == b)
}

fn isnice_pt2(input: &str) -> bool {
    pairs(input) && repeat_between(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().filter(|x| isnice_pt1(x)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().filter(|x| isnice_pt2(x)).count())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use advent_of_code::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), Some(1));
    }

    #[test]
    fn test_part_two() {
        use advent_of_code::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), Some(0));
    }
}
