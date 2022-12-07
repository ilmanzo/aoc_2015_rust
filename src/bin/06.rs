type Lights = Vec<usize>;

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Point {
    x: usize,
    y: usize,
}

struct Instruction {
    command: Command,
    from: Point,
    to: Point,
}

fn turn(lights: &mut Lights, value: u8, from: Point, to: Point) {
    for y in from.y..=to.y {
        for x in from.x..=to.x {
            lights[x + y * 1000] = value as usize;
        }
    }
}

fn toggle(lights: &mut Lights, from: Point, to: Point) {
    for y in from.y..=to.y {
        for x in from.x..=to.x {
            lights[x + y * 1000] = 1 - lights[x + y * 1000];
        }
    }
}

fn brightness(lights: &mut Lights, value: i8, from: Point, to: Point) {
    for y in from.y..=to.y {
        for x in from.x..=to.x {
            let i = x + y * 1000;
            if value < 0 {
                if lights[i] > 0 {
                    lights[i] -= 1;
                }
            } else {
                lights[i] += value as usize
            }
        }
    }
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let len = tokens.len();
        let command = match tokens[0..2] {
            ["turn", "on"] => Command::TurnOn,
            ["turn", "off"] => Command::TurnOff,
            ["toggle", _] => Command::Toggle,
            _ => panic!("invalid command {:?}", tokens),
        };
        let (from, to) = match tokens[len - 3..] {
            [a, "through", b] => (parse_point(a), parse_point(b)),
            _ => panic!("Unrecognized args {:?}", tokens),
        };
        Self { command, from, to }
    }
}

// parses a string from coord x,y to a Point
fn parse_point(s: &str) -> Point {
    let coords: Vec<_> = s
        .split(',')
        .map(|d| d.parse::<usize>().expect("can't parse integer"))
        .collect();
    match coords.as_slice() {
        [a, b] => Point { x: *a, y: *b },
        _ => panic!("Unrecognized coordinate {:?}", s),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lights = vec![0; 1000 * 1000];
    for line in input.lines() {
        let instr: Instruction = line.into();
        match instr.command {
            Command::TurnOn => turn(&mut lights, 1, instr.from, instr.to),
            Command::TurnOff => turn(&mut lights, 0, instr.from, instr.to),
            Command::Toggle => toggle(&mut lights, instr.from, instr.to),
        }
    }
    Some(lights.iter().filter(|&v| *v == 1).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lights = vec![0; 1000 * 1000];
    for line in input.lines() {
        let instr: Instruction = line.into();
        match instr.command {
            Command::TurnOn => brightness(&mut lights, 1, instr.from, instr.to),
            Command::TurnOff => brightness(&mut lights, -1, instr.from, instr.to),
            Command::Toggle => brightness(&mut lights, 2, instr.from, instr.to),
        }
    }
    Some(lights.iter().sum())
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
        use advent_of_code::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        use advent_of_code::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), Some(0));
    }
}
