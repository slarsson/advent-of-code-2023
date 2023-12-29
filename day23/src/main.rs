use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut path: HashMap<(i32, i32), char> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                continue;
            }
            path.insert((x as i32, y as i32), ch);
        }
    }

    let size = input.lines().count();

    let start = (1, 0);
    let end = (size as i32 - 2, size as i32 - 1);

    println!("part1: {}", part1(&path, start, end));
    println!("part2: {}", part2(&path, start, end));
}

fn part1(path: &HashMap<(i32, i32), char>, start: (i32, i32), end: (i32, i32)) -> usize {
    let mut queue: VecDeque<(i32, i32, HashSet<(i32, i32)>)> = VecDeque::new();

    let mut start_set: HashSet<(i32, i32)> = HashSet::new();
    start_set.insert(start);
    queue.push_back((start.0, start.1, HashSet::new()));

    let mut max = 0;
    loop {
        if queue.len() == 0 {
            break;
        }

        let head = queue.pop_front().unwrap();

        if head.0 == end.0 && head.1 == end.1 {
            if head.2.len() > max {
                max = head.2.len();
            }
            continue;
        }

        for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x = head.0 + delta.0;
            let y = head.1 + delta.1;

            if head.2.contains(&(x, y)) {
                continue;
            }

            match path.get(&(x, y)) {
                Some(&value) => {
                    let mut next = head.clone();

                    match value {
                        '.' => {
                            next.0 = x;
                            next.1 = y;
                            next.2.insert((x, y));
                        }
                        '>' => {
                            if delta == (-1, 0) {
                                continue;
                            }

                            next.0 = x + 1;
                            next.1 = y;
                            next.2.insert((x, y));
                            next.2.insert((x + 1, y));
                        }
                        'v' => {
                            if delta == (0, -1) {
                                continue;
                            }

                            next.0 = x;
                            next.1 = y + 1;
                            next.2.insert((x, y));
                            next.2.insert((x, y + 1));
                        }
                        _ => {}
                    }

                    queue.push_back(next);
                }
                None => {}
            }
        }
    }

    return max;
}

fn part2(path: &HashMap<(i32, i32), char>, start: (i32, i32), end: (i32, i32)) -> i32 {
    let edges = extract_edges(path, start, end);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    return dfs(&edges, &mut seen, start, end);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Edge {
    from: (i32, i32),
    to: (i32, i32),
    length: i32,
}

impl Edge {
    fn new(start: (i32, i32)) -> Self {
        return Edge {
            from: start,
            to: start,
            length: 0,
        };
    }

    fn increment(&mut self, x: i32, y: i32) {
        if (x - self.to.0).abs() > 1 {
            panic!("invalid x");
        }

        if (y - self.to.1).abs() > 1 {
            panic!("invalid y");
        }

        self.length += 1;
        self.to = (x, y);
    }

    fn sort(&mut self) {
        if self.to < self.from {
            (self.from, self.to) = (self.to, self.from);
        }
    }

    fn connects(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        if self.from == (x, y) {
            return Some(self.to);
        }

        if self.to == (x, y) {
            return Some(self.from);
        }

        return None;
    }
}

fn extract_edges(
    path: &HashMap<(i32, i32), char>,
    start: (i32, i32),
    end: (i32, i32),
) -> Vec<Edge> {
    let mut intersections: Vec<(i32, i32)> = Vec::new();
    intersections.push(start);
    intersections.push(end);

    for (step, _) in path {
        let x = step.0;
        let y = step.1;

        let mut count = 0;
        for next in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if path.get(&next).is_some() {
                count += 1;
            }
        }
        if count > 2 {
            intersections.push(step.clone());
        }
    }

    let mut edges: Vec<Edge> = Vec::new();
    let mut seen_edges: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    for (x, y) in intersections.clone() {
        for next in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if path.get(&next).is_none() {
                continue;
            }

            let mut edge = Edge::new((x, y));

            let mut seen: HashSet<(i32, i32)> = HashSet::new();
            seen.insert((x, y));

            let mut cursor = next;
            loop {
                seen.insert(cursor);
                edge.increment(cursor.0, cursor.1);

                if intersections.contains(&cursor) {
                    break;
                }

                for next_cursor in [
                    (cursor.0 + 1, cursor.1),
                    (cursor.0 - 1, cursor.1),
                    (cursor.0, cursor.1 + 1),
                    (cursor.0, cursor.1 - 1),
                ] {
                    if path.contains_key(&next_cursor) && !seen.contains(&next_cursor) {
                        cursor = next_cursor;
                        break;
                    }
                }
            }

            edge.sort();

            let edge_key = (edge.from, edge.to);
            if !seen_edges.contains(&edge_key) {
                edges.push(edge);
                seen_edges.insert(edge_key);
            }
        }
    }

    return edges;
}

fn dfs(
    edges: &Vec<Edge>,
    seen: &mut HashSet<(i32, i32)>,
    current: (i32, i32),
    end: (i32, i32),
) -> i32 {
    if current == end {
        return 0;
    }

    seen.insert(current);

    let mut sum = -i32::MAX;

    for edge in edges {
        match edge.connects(current.0, current.1) {
            Some(next) => {
                if !seen.contains(&next) {
                    sum = sum.max(edge.length + dfs(edges, seen, next, end));
                }
            }
            None => {}
        }
    }

    seen.remove(&current);

    return sum;
}
