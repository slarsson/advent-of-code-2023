use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Tile {
    x: i32,
    y: i32,
    value: char,
}

impl Tile {
    fn new(x: i32, y: i32, ch: char) -> Self {
        Tile {
            x: x,
            y: y,
            value: ch,
        }
    }

    fn next(&self, speed: (i32, i32)) -> Vec<((i32, i32), (i32, i32))> {
        match self.value {
            '|' => match speed {
                (0, 1) => return vec![((self.x, self.y + 1), (0, 1))],
                (0, -1) => return vec![((self.x, self.y - 1), (0, -1))],
                (1, 0) | (-1, 0) => {
                    return vec![
                        ((self.x, self.y + 1), (0, 1)),
                        ((self.x, self.y - 1), (0, -1)),
                    ]
                }
                _ => panic!("noop"),
            },
            '-' => match speed {
                (1, 0) => return vec![((self.x + 1, self.y), (1, 0))],
                (-1, 0) => return vec![((self.x - 1, self.y), (-1, 0))],
                (0, 1) | (0, -1) => {
                    return vec![
                        ((self.x + 1, self.y), (1, 0)),
                        ((self.x - 1, self.y), (-1, 0)),
                    ]
                }
                _ => panic!("noop"),
            },
            '/' => match speed {
                (1, 0) => return vec![((self.x, self.y - 1), (0, -1))],
                (-1, 0) => return vec![((self.x, self.y + 1), (0, 1))],
                (0, 1) => return vec![((self.x - 1, self.y), (-1, 0))],
                (0, -1) => return vec![((self.x + 1, self.y), (1, 0))],
                _ => panic!("noop"),
            },
            '\\' => match speed {
                (1, 0) => return vec![((self.x, self.y + 1), (0, 1))],
                (-1, 0) => return vec![((self.x, self.y - 1), (0, -1))],
                (0, 1) => return vec![((self.x + 1, self.y), (1, 0))],
                (0, -1) => return vec![((self.x - 1, self.y), (-1, 0))],
                _ => panic!("noop"),
            },
            _ => return vec![((self.x + speed.0, self.y + speed.1), speed.clone())],
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: HashMap<(i32, i32), Tile>,
    size: i32,
}

impl Grid {
    fn new(grid: HashMap<(i32, i32), Tile>, size: usize) -> Self {
        return Grid {
            grid: grid,
            size: size as i32,
        };
    }

    fn get(&self, position: (i32, i32)) -> Option<&Tile> {
        return self.grid.get(&position);
    }

    fn is_inside(&self, position: (i32, i32)) -> bool {
        return position.0 >= 0
            && position.0 < self.size
            && position.1 >= 0
            && position.1 < self.size;
    }
}

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut data: HashMap<(i32, i32), Tile> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            data.insert((x as i32, y as i32), Tile::new(x as i32, y as i32, ch));
        }
    }

    let size = input.lines().count();

    let grid = Grid::new(data.clone(), size);

    println!("part1: {}", part1(grid.clone()));
    println!("part2: {}", part2(grid.clone()));
}

fn part1(grid: Grid) -> usize {
    let start = ((0, 0), (1, 0));
    return solve(&grid, start);
}

fn part2(grid: Grid) -> usize {
    let mut max = 0;
    for x in 0..grid.size {
        let top = solve(&grid, ((x, 0), (0, 1)));
        if top > max {
            max = top;
        }

        let bottom = solve(&grid, ((x, grid.size - 1), (0, -1)));
        if bottom > max {
            max = bottom;
        }
    }

    for y in 0..grid.size {
        let left = solve(&grid, ((0, y), (1, 0)));
        if left > max {
            max = left;
        }

        let right = solve(&grid, ((grid.size - 1, y), (-1, 0)));
        if right > max {
            max = right;
        }
    }

    return max;
}

fn solve(grid: &Grid, start: ((i32, i32), (i32, i32))) -> usize {
    let mut queue: VecDeque<((i32, i32), (i32, i32))> = VecDeque::new();
    let mut seen_tiles: HashSet<(i32, i32)> = HashSet::new();
    let mut seen_beams: HashSet<Vec<(i32, i32)>> = HashSet::new();

    queue.push_back(start);

    loop {
        if queue.is_empty() {
            break;
        }

        let mut head = queue.pop_front().unwrap();
        let mut beam: Vec<(i32, i32)> = vec![head.0];

        loop {
            if !grid.is_inside(head.0) || seen_beams.get(&beam).is_some() {
                break;
            }

            let tile = grid.get(head.0).unwrap();
            seen_tiles.insert(head.0);

            let next_items = tile.next(head.1);

            if next_items.len() == 1 {
                beam.push(next_items[0].0);
                head = next_items[0];
                continue;
            }

            for item in next_items {
                queue.push_back(item);
            }

            break;
        }

        seen_beams.insert(beam.clone());
    }

    return seen_tiles.len();
}
