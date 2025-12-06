use aoc2025::read_input;

fn parse_line(line: &str) -> (char, i64) {
    let dir = line.chars().next().unwrap();
    let value = line[1..].parse().unwrap();
    (dir, value)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .fold((50, 0), |(pos, count), (dir, value)| {
            let delta = if dir == 'R' { value } else { -value };
            let new_pos = (pos + delta).rem_euclid(100);
            let new_count = count + (new_pos == 0) as i64;
            (new_pos, new_count)
        })
        .1
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .fold((50, 0), |(pos, count), (dir, value)| {
            let full_rotations = value / 100;
            let extra = value % 100;

            let distance_to_zero = if dir == 'R' {
                100 - pos
            } else {
                if pos == 0 { 100 } else { pos }
            };

            let crosses = full_rotations + (extra >= distance_to_zero) as i64;
            let delta = if dir == 'R' { value } else { -value };
            let new_pos = (pos + delta).rem_euclid(100);

            (new_pos, count + crosses)
        })
        .1
}

fn main() {
    let input = read_input(1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}