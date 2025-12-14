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
    let (i, j) = (i as i32, j as i32);

    if !is_paper(i, j, grid) {
        return false;
    }

    let neighboring_paper_count: i32 = (-1..=1)
        .flat_map(|di| (-1..=1).map(move |dj| (di, dj)))
        .filter(|&(di, dj)| di != 0 || dj != 0)
        .map(|(di, dj)| (i + di, j + dj))
        .map(|(adj_i, adj_j)| is_paper(adj_i, adj_j, &grid) as i32)
        .sum();

    neighboring_paper_count < 4
}

fn part1(input: &str) -> i64 {
    let grid = parse_input(input);
    let (m, n) = (grid.len(), grid[0].len());

    (0..m)
        .flat_map(|i| {
            let grid_ref = &grid;
            (0..n).map(move |j| is_valid_paper(i, j, grid_ref) as i64)
        })
        .sum()
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