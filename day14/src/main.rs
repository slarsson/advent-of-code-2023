use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: HashMap<(i32, i32), char>,
    size: usize,
}

impl Grid {
    fn new(grid: HashMap<(i32, i32), char>, size: usize) -> Self {
        return Grid {
            grid: grid,
            size: size,
        };
    }

    pub fn calculate_north_load(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.size {
            for x in 0..self.size {
                if self.grid.get(&(x as i32, y as i32)).unwrap().eq(&'O') {
                    sum += self.size - y;
                }
            }
        }
        return sum;
    }

    fn can_move_to(&self, to: &(i32, i32)) -> bool {
        match self.grid.get(to) {
            Some(value) => value.eq(&'.'),
            None => false,
        }
    }

    pub fn move_to(&mut self, from: (i32, i32), to: (i32, i32)) {
        if from == to {
            return;
        }
        self.grid.insert(from, '.');
        self.grid.insert(to, 'O');
    }

    pub fn is_stone(&self, position: &(i32, i32)) -> bool {
        match self.grid.get(position) {
            Some(value) => value.eq(&'O'),
            None => false,
        }
    }

    pub fn to_string(&self) -> String {
        let mut s: String = String::new();
        for y in 0..self.size {
            for x in 0..self.size {
                s.push(*self.grid.get(&(x as i32, y as i32)).unwrap());
            }
        }
        return s;
    }
}

struct SpinCycle {
    direction: Direction,
}

// should use transpose here instead of this stupid thing....
impl SpinCycle {
    fn new(direction: Direction) -> Self {
        return SpinCycle {
            direction: direction,
        };
    }

    pub fn is_next_step_within_bounds(
        &self,
        current_position: (i32, i32),
        grid_size: usize,
    ) -> bool {
        match self.direction {
            Direction::North => return current_position.1 - 1 >= 0,
            Direction::South => return current_position.1 + 1 < grid_size as i32,
            Direction::East => return current_position.0 + 1 < grid_size as i32,
            Direction::West => return current_position.0 - 1 >= 0,
        }
    }

    pub fn next(&self, current_position: (i32, i32)) -> (i32, i32) {
        match self.direction {
            Direction::North => return (current_position.0, current_position.1 - 1),
            Direction::South => return (current_position.0, current_position.1 + 1),
            Direction::East => return (current_position.0 + 1, current_position.1),
            Direction::West => return (current_position.0 - 1, current_position.1),
        }
    }

    fn to_coord(&self, outer: usize, inner: usize) -> (i32, i32) {
        match self.direction {
            Direction::North | Direction::South => (inner as i32, outer as i32),
            Direction::East | Direction::West => (outer as i32, inner as i32),
        }
    }

    fn outer_range(&self, grid_size: usize) -> Vec<usize> {
        return match self.direction {
            Direction::North | Direction::West => (0..grid_size).collect(),
            Direction::South | Direction::East => (0..grid_size).rev().collect(),
        };
    }
}

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut data: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            data.insert((x as i32, y as i32), ch);
        }
    }

    let size = input.lines().count();

    let grid = Grid::new(data.clone(), size);

    println!("part1: {}", part1(grid.clone(), size));
    println!("part2: {}", part2(grid.clone(), size));
}

fn part1(mut grid: Grid, size: usize) -> usize {
    let cycle = SpinCycle::new(Direction::North);

    for y in 0..size {
        for x in 0..size {
            let position = (x as i32, y as i32);

            if !grid.is_stone(&position) {
                continue;
            }

            let mut cursor = position;
            loop {
                if !cycle.is_next_step_within_bounds(cursor, size) {
                    break;
                }

                let next = cycle.next(cursor);

                if !grid.can_move_to(&next) {
                    break;
                }

                cursor = next;
            }

            grid.move_to(position, cursor);
        }
    }

    return grid.calculate_north_load();
}

fn part2(mut grid: Grid, size: usize) -> usize {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut results: Vec<usize> = Vec::new();
    let mut incr: usize = 0;

    loop {
        for cycle in [
            SpinCycle::new(Direction::North),
            SpinCycle::new(Direction::West),
            SpinCycle::new(Direction::South),
            SpinCycle::new(Direction::East),
        ] {
            for &a in cycle.outer_range(size).iter() {
                for b in 0..size {
                    let position = cycle.to_coord(a, b);

                    if !grid.is_stone(&position) {
                        continue;
                    }

                    let mut cursor = position;
                    loop {
                        if !cycle.is_next_step_within_bounds(cursor, size) {
                            break;
                        }

                        let next = cycle.next(cursor);

                        if !grid.can_move_to(&next) {
                            break;
                        }

                        cursor = next;
                    }

                    grid.move_to(position, cursor);
                }
            }
        }

        incr += 1;

        let fingerprint = grid.to_string();

        match seen.get(&fingerprint) {
            Some(&index) => {
                // ((1B - <first hit>) % (<second hit> - <first hit>)) + <first hit>
                let want_index = (1000000000 - index) % (incr - index) + index;
                return *results.get(want_index - 1).unwrap();
            }
            None => {
                results.push(grid.calculate_north_load());
                seen.insert(fingerprint, incr);
            }
        };
    }
}
