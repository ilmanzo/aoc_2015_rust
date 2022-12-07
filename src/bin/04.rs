extern crate md5;

use std::{fmt::Write, fs::OpenOptions};

/// Check if a byte array has at least the given number of zero nibbles
pub fn has_zero_nibbles(bytes: md5::Digest, num_zeroes: usize) -> bool {
    bytes.iter().take(num_zeroes / 2).all(|b| *b == 0)
        && (num_zeroes % 2 == 0 || *bytes.iter().nth(num_zeroes / 2).unwrap() < 0x10)
}

/// Finds the suffix that needs to be appended to the given prefix so that
/// the MD5 hash has at least the given number of leading zero nibbles
pub fn find_suffix(prefix: &str, num_zeroes: usize) -> usize {
    let mut suffix = 1;
    let mut full_str = String::with_capacity(20);
    let mut suffix_str = String::with_capacity(10);
    loop {
        full_str.clear();
        suffix_str.clear();
        suffix_str.write_fmt(format_args!("{}", suffix)).unwrap();
        full_str.write_str(prefix).unwrap();
        full_str.push_str(&suffix_str);
        let digest = md5::compute(&full_str);
        if has_zero_nibbles(digest, num_zeroes) {
            break;
        }
        suffix += 1;
    }
    suffix
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(find_suffix(input, 5))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(find_suffix(input, 6))
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
        use advent_of_code::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), Some(1048970));
    }

    #[test]
    fn test_part_two() {
        use advent_of_code::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), Some(5714438));
    }
}
