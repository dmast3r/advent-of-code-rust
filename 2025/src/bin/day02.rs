
use aoc2025::read_input;

fn parse_input(input: &str) -> Vec<(i128, i128)> {
    input
        .trim()
        .split(',')
        .filter_map(|range| {
            let mut parts = range.split('-');
            Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
        })
        .collect()
}

fn is_palindrome_half(s: &str) -> bool {
    let n = s.len();
    n % 2 == 0 && s[..n / 2] == s[n / 2..]
}

fn is_repeating_pattern(s: &str) -> bool {
    let n = s.len();
    (1..=n / 2).any(|pattern_len| {
        if n % pattern_len != 0 {
            return false;
        }
        let pattern = &s[..pattern_len];
        s.as_bytes().chunks(pattern_len).all(|chunk| chunk == pattern.as_bytes())
    })
}

fn part1(input: &str) -> i128 {
    parse_input(input)
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&val| is_palindrome_half(&val.to_string()))
        .sum()
}

fn part2(input: &str) -> i128 {
    parse_input(input)
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&val| {
            let s = val.to_string();
            s.len() >= 2 && is_repeating_pattern(&s)
        })
        .sum()
}

fn main() {
    let input = read_input(2);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
  "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}