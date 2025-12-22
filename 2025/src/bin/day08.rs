use std::cmp::Reverse;

use aoc2025::read_input;

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn get_distance(&self, other: &Coordinate) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}
struct UnionFind {
    rank: Vec<usize>,
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            rank: vec![1; n],
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if u != self.parent[u] {
            self.parent[u] = self.find(self.parent[u]);
        }
        self.parent[u]
    }

    fn join(&mut self, u: usize, v: usize) -> bool {
        let ru = self.find(u);
        let rv = self.find(v);

        if ru == rv {
            return false;
        }

        let (ru_idx, rv_idx) = (ru as usize, rv as usize);

        if self.rank[ru_idx] < self.rank[rv_idx] {
            self.parent[ru_idx] = rv;
            self.rank[rv_idx] += self.rank[ru_idx];
        } else {
            self.parent[rv_idx] = ru;
            self.rank[ru_idx] += self.rank[rv_idx];
        }

        true
    }
}

fn get_junction_boxes(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').filter_map(|s| s.parse().ok());
            Coordinate {
                x: coords.next().unwrap(),
                y: coords.next().unwrap(),
                z: coords.next().unwrap(),
            }
        })
        .collect()
}

fn get_sorted_all_pair_distances(junction_boxes: &Vec<Coordinate>) -> Vec<(usize, usize, i64)> {
    let mut all_pair_distances: Vec<_> = (0..junction_boxes.len())
        .flat_map(|i| {
            (i + 1..junction_boxes.len())
                .map(move |j| (i, j, junction_boxes[i].get_distance(&junction_boxes[j])))
        })
        .collect();

    all_pair_distances.sort_unstable_by_key(|&(_, _, dist)| dist);
    all_pair_distances
}

fn part1(input: &str, join_count: usize) -> i64 {
    let junction_boxes = get_junction_boxes(input);
    let sorted_all_pair_distances = get_sorted_all_pair_distances(&junction_boxes);

    let mut uf = UnionFind::new(junction_boxes.len());

    for &(u, v, _) in &sorted_all_pair_distances[..join_count] {
        uf.join(u, v);
    }

    let mut group_sizes = vec![0; junction_boxes.len()];
    (0..junction_boxes.len()).for_each(|u| group_sizes[uf.find(u)] += 1);

    group_sizes.sort_unstable_by_key(|&n| Reverse(n));
    group_sizes.iter().take(3).product()
}

fn part2(input: &str) -> i64 {
    let junction_boxes = get_junction_boxes(input);
    let sorted_all_pair_distances = get_sorted_all_pair_distances(&junction_boxes);

    let mut uf = UnionFind::new(junction_boxes.len());
    let mut components = junction_boxes.len();

    for &(u, v, _) in &sorted_all_pair_distances {
        if uf.join(u, v) {
            components -= 1;
            if components == 1 {
                return junction_boxes[u].x * junction_boxes[v].x;
            }
        }
    }

    unreachable!("Should always connect to single component")
}

fn main() {
    let input = read_input(8);

    println!("Part 1: {}", part1(&input, 1000));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 25272);
    }
}