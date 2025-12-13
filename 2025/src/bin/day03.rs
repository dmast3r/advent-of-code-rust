use aoc2025::read_input;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|row| {
            row.chars()
                .filter_map(|ch| ch.to_digit(10))
                .map(i64::from)
                .collect()
        }).collect()
}

fn find_max_joltage(batteries: &[i64], count: usize) -> i64 {
    let mut stack = Vec::new();
    let n = batteries.len();

    for (i, &battery) in batteries.iter().enumerate() {
        while stack.last().is_some_and(|&val| val < battery && stack.len() + n - i > count) {
            stack.pop();
        }

        if stack.len() < count {
            stack.push(battery);
        }
    }

    stack.iter().fold(0, |acc, &curr| acc * 10 + curr)
}

fn solve(input: &str, count: usize) -> i64 {
    let input = parse_input(input);
    input.iter()
        .map(|batteries| find_max_joltage(batteries, count))
        .sum()
}

fn part1(input: &str) -> i64 {
    solve(input, 2)
}

fn part2(input: &str) -> i64 {
    solve(input, 12)
}

fn main() {
    let input = read_input(3);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
    }
}