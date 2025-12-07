use std::collections::{HashMap, HashSet, VecDeque};

use aoc2025::read_input;

type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);

fn parse_grid(input: &str) -> Grid {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_start(grid: &Grid) -> Option<Pos> {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|j| (i, j))
        })
}

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);
    let rows = grid.len();

    let start = find_start(&grid).unwrap();

    let mut queue = VecDeque::from([start]);
    let mut seen = HashSet::from([start]);
    let mut res = 0;

    while let Some((row, col)) = queue.pop_front() {
        match grid[row][col] {
            '^' => {
                res += 1;
                [col.checked_sub(1), Some(col + 1)]
                    .into_iter()
                    .flatten()
                    .map(|c| (row, c))
                    .filter(|pos| seen.insert(*pos))
                    .for_each(|pos| queue.push_back(pos));
            }
            _ => {
                if row + 1 < rows && seen.insert((row + 1, col)) {
                    queue.push_back((row + 1, col));
                }
            }
        }
    }

    res
}

fn count_ways(pos: Pos, grid: &Grid, dp: &mut HashMap<Pos, i64>) -> i64 {
    let (row, col) = pos;
    let (rows, cols) = (grid.len(), grid[0].len());

    if row >= rows || col >= cols {
        return 1;
    }

    if let Some(&res) = dp.get(&pos) {
        return res;
    }

    let res = match grid[row][col] {
        '^' => {
            let left = col.checked_sub(1)
                .map(|c| count_ways((row, c), grid, dp))
                .unwrap_or(1);
            let right = count_ways((row, col + 1), grid, dp);
            left + right
        }
        _ => {
            count_ways((row + 1, col), grid, dp)
        }
    };

    dp.insert(pos, res);
    res
}

fn part2(input: &str) -> i64 {
    let grid = parse_grid(input);
    let start = find_start(&grid).unwrap();

    let mut dp = HashMap::new();
    count_ways(start, &grid, &mut dp)
}

fn main() {
    let input = read_input(7);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 40);
    }
}
