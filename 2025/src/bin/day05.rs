use aoc2025::read_input;

fn get_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn get_ids(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn get_ranges_and_ids(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let split_result = input.split_once("\n\n");
    let (range_input, ids_input) = split_result.unwrap();
    (get_ranges(range_input), get_ids(ids_input))
}

fn part1(input: &str) -> i64 {
    let (ranges, ids) = get_ranges_and_ids(input);

    let is_id_in_ranges = |id| ranges.iter().any(|&(start, end)| start <= id && id <= end);

    ids.into_iter().filter(|&id| is_id_in_ranges(id)).count() as i64
}

fn part2(input: &str) -> i64 {
    let (mut ranges, _) = get_ranges_and_ids(input);
    ranges.sort_unstable();

    let (mut last_start, mut max_end, mut res) = (1, 0, 0);
    for &(start, end) in ranges.iter() {
        if start > max_end {
            res += max_end - last_start + 1;
            last_start = start;
            max_end = end;
        } else {
            max_end = max_end.max(end);
        }
    }

    res += max_end - last_start + 1;
    res
}

fn main() {
    let input = read_input(5);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 14);
    }
}
