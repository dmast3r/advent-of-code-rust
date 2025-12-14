use aoc2025::read_input;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_paper(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);

    if i < 0 || j < 0 || i >= m || j >= n {
        return false;
    }

    grid[i as usize][j as usize] == '@'
}

fn is_valid_paper(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if !is_paper(i as i32, j as i32, grid) {
        return false;
    }

    let mut count_neighboring_papers = 0;

    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }

            if is_paper(i as i32 + di, j as i32 + dj, &grid) {
                count_neighboring_papers += 1;
                if count_neighboring_papers == 4 {
                    return false;
                }
            }
        }
    }

    true
}

fn part1(input: &str) -> i64 {
    let grid = parse_input(input);
    let (m, n) = (grid.len(), grid[0].len());
    let mut res = 0;

    for i in 0..m {
        for j in 0..n {
            if is_valid_paper(i, j, &grid) {
                res += 1;
            }
        }
    }

    res
}

fn part2(input: &str) -> i64 {
    let mut grid = parse_input(input);
    let (m, n) = (grid.len(), grid[0].len());

    let mut should_continue_iteration = true;
    let mut res = 0;

    while should_continue_iteration {
        should_continue_iteration = false;

        for i in 0..m {
            for j in 0..n {
                if is_valid_paper(i, j, &grid) {
                    res += 1;
                    grid[i][j] = '.';
                    should_continue_iteration = true;
                }
            }
        }
    }

    res
}

fn main() {
    let input = read_input(4);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 43);
    }
}
