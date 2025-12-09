use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}
impl Position {
    fn euclidian_distance_squared(&self, other: &Self) -> u64 {
        fn delta(a: i64, b: i64) -> u64 {
            a.abs_diff(b).pow(2)
        }

        let x_delta = delta(self.x, other.x);
        let y_delta = delta(self.y, other.y);
        let z_delta = delta(self.z, other.z);

        x_delta + y_delta + z_delta
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Graph(HashMap<usize, Vec<usize>>);
impl Graph {
    fn new(connections: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

        for (x, y) in connections {
            graph
                .entry(x)
                .and_modify(|x| x.push(y))
                .or_insert_with(|| vec![y]);

            graph
                .entry(y)
                .and_modify(|y| y.push(x))
                .or_insert_with(|| vec![x]);
        }

        Self(graph)
    }

    fn traverse(&self, visited: &mut HashSet<usize>, i: usize) -> Option<usize> {
        if visited.contains(&i) {
            return None;
        }

        let mut group = HashSet::new();
        let mut queue = VecDeque::from([i]);

        while let Some(x) = queue.pop_front() {
            group.insert(x);
            visited.insert(x);

            if let Some(neighbors) = self.0.get(&x) {
                for n in neighbors {
                    if !visited.contains(n) {
                        queue.push_back(*n);
                    }
                }
            }
        }

        Some(group.len())
    }

    fn groups(&self, n: usize) -> Vec<usize> {
        let mut visited = HashSet::new();

        (0..n)
            .filter_map(|i| self.traverse(&mut visited, i))
            .collect()
    }
}

type Distance = (usize, usize, u64);

fn get_distances(boxes: &[Position]) -> Vec<Distance> {
    let mut distances = vec![];

    for x in 0..boxes.len() {
        for y in x + 1..boxes.len() {
            let dist = boxes[x].euclidian_distance_squared(&boxes[y]);
            distances.push((x, y, dist));
        }
    }

    distances.sort_by_key(|(_, _, dist)| *dist);
    distances
}

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.trim().split(',');

            Some(Position {
                x: parts.next()?.parse().ok()?,
                y: parts.next()?.parse().ok()?,
                z: parts.next()?.parse().ok()?,
            })
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    parent: usize,
    size: usize,
}

#[derive(Debug)]
struct DisjointSet {
    nodes: HashMap<usize, Node>,
}
impl DisjointSet {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn make_set(&mut self, x: usize) {
        self.nodes.entry(x).or_insert(Node { parent: x, size: 1 });
    }

    fn find(&mut self, x: &usize) -> Option<usize> {
        let parent = self.nodes.get(x).map(|x| x.parent)?;

        if parent != *x {
            let new_parent = self.find(&parent)?;
            let node = self.nodes.get_mut(x)?;
            node.parent = new_parent;
            Some(new_parent)
        } else {
            Some(*x)
        }
    }

    fn union(&mut self, x: &usize, y: &usize) {
        let x_root = self.find(x).unwrap();
        let y_root = self.find(y).unwrap();

        if x_root == y_root {
            return;
        }

        let x_size = self.nodes.get(&x_root).unwrap().size;
        let y_size = self.nodes.get(&y_root).unwrap().size;

        let (new_root, other) = if x_size >= y_size {
            (x_root, y_root)
        } else {
            (y_root, x_root)
        };

        self.nodes.get_mut(&other).unwrap().parent = new_root;
        self.nodes.get_mut(&new_root).unwrap().size = x_size + y_size;
    }

    fn kruskal(&mut self, distances: &[Distance]) -> Vec<(usize, usize)> {
        for (x, y, _) in distances.iter().copied() {
            self.make_set(x);
            self.make_set(y);
        }

        let mut edges = vec![];

        for (x, y, _) in distances {
            let x_root = self.find(x).unwrap();
            let y_root = self.find(y).unwrap();

            if x_root != y_root {
                edges.push((*x, *y));
                self.union(x, y);
            }
        }

        edges
    }
}

fn part1(boxes: &[Position], connections: usize) -> usize {
    let distances = get_distances(boxes);

    let connections = distances
        .into_iter()
        .take(connections)
        .map(|(x, y, _)| (x, y));

    let graph = Graph::new(connections);

    let group_sizes = {
        let mut g = graph.groups(boxes.len());
        g.sort_by(|a, b| b.cmp(a));
        g
    };

    group_sizes.iter().take(3).fold(1, |total, x| total * *x)
}

fn part2(boxes: &[Position]) -> i64 {
    let distances = get_distances(boxes);
    let mut disjoint_set = DisjointSet::new();
    let edges = disjoint_set.kruskal(&distances);

    let last = *edges.last().unwrap();
    boxes[last.0].x * boxes[last.1].x
}

fn main() {
    let input = common::read_stdin();
    let input = parse_input(&input);

    let (time, result) = common::timed(|| part1(&input, 1000));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part2(&input));
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
            162,817,812
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
            425,690,689
        "#;

        let boxes = parse_input(input);
        assert_eq!(part1(&boxes, 10), 40);
        assert_eq!(part2(&boxes), 25272);
    }
}
