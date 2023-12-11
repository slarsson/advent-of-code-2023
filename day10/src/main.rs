use std::collections::{HashMap, HashSet};

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), ch);
        }
    }

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    println!("part1: {:?}", part1(&grid));
    println!("part2: {:?}", part2(&grid, width, height));
}

fn part1(grid: &HashMap<(i32, i32), char>) -> i32 {
    let path = resolve_path(grid);
    return (path.len() as i32 + 1) / 2;
}

fn part2(grid: &HashMap<(i32, i32), char>, width: usize, height: usize) -> i32 {
    let path = resolve_path(grid);

    let start = grid.iter().find(|x| *x.1 == 'S').unwrap().0;

    let mut border: HashSet<(i32, i32)> = path.iter().map(|x| x.0).collect();
    border.insert(start.clone());

    let (first_coord, first_value) = path.get(0).unwrap();
    let (last_coord, last_value) = path.iter().last().unwrap();

    let to_first = (first_coord.0 - start.0, first_coord.1 - start.1);
    let to_last = (last_coord.0 - start.0, last_coord.1 - start.1);

    let mut start_tile = 'S';
    for v in ['|', '-', 'F', 'L', 'J', '7'] {
        if is_connected(to_first, v, *first_value) && is_connected(to_last, v, *last_value) {
            start_tile = v;
            break;
        }
    }

    let mut sum = 0;

    for y in 0..height {
        for x in 0..width {
            // ignore if border
            if border.get(&(x as i32, y as i32)).is_some() {
                continue;
            }

            // use point-in-polygon algorithm
            let mut collisions = 0;
            for p in (0..x).rev() {
                let coord = (p as i32, y as i32);

                if border.get(&coord).is_none() {
                    continue;
                }

                let mut tile = grid.get(&coord).unwrap().to_owned();

                // replace start tile with border part
                if tile == 'S' {
                    tile = start_tile;
                }

                // avoid "top" border
                if tile == 'F' || tile == '-' || tile == '7' {
                    continue;
                }

                collisions += 1;
            }

            if collisions % 2 == 1 {
                sum += 1;
            }
        }
    }

    return sum;
}

fn resolve_path(grid: &HashMap<(i32, i32), char>) -> Vec<((i32, i32), char)> {
    let start = grid.iter().find(|x| *x.1 == 'S').unwrap();

    let mut cursor: ((i32, i32), char) = (start.0.clone(), start.1.clone());
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut path: Vec<((i32, i32), char)> = Vec::new();

    'outer: loop {
        seen.insert(cursor.0);

        for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (cursor.0 .0 + x, cursor.0 .1 + y);

            if !seen.get(&next).is_none() {
                continue;
            }

            match grid.get(&next) {
                None => continue,
                Some(value) => {
                    if is_connected((x, y), cursor.1, *value) {
                        cursor = (next, *value);
                        path.push(cursor);
                        continue 'outer;
                    }
                }
            }
        }

        // no more connections exists...
        break;
    }

    return path;
}

fn is_connected(change: (i32, i32), from: char, to: char) -> bool {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    return match change {
        (1, 0) => {
            return match (from, to) {
                ('S', '-') => true,
                ('S', 'J') => true,
                ('S', '7') => true,
                ('-', '-') => true,
                ('-', 'J') => true,
                ('-', '7') => true,
                ('L', '-') => true,
                ('L', 'J') => true,
                ('L', '7') => true,
                ('F', '-') => true,
                ('F', 'J') => true,
                ('F', '7') => true,
                _ => false,
            }
        }
        (-1, 0) => {
            return match (from, to) {
                ('S', '-') => true,
                ('S', 'L') => true,
                ('S', 'F') => true,
                ('-', '-') => true,
                ('-', 'L') => true,
                ('-', 'F') => true,
                ('J', '-') => true,
                ('J', 'L') => true,
                ('J', 'F') => true,
                ('7', '-') => true,
                ('7', 'L') => true,
                ('7', 'F') => true,
                _ => false,
            }
        }
        (0, 1) => {
            return match (from, to) {
                ('S', '|') => true,
                ('S', 'L') => true,
                ('S', 'J') => true,
                ('|', '|') => true,
                ('|', 'L') => true,
                ('|', 'J') => true,
                ('7', '|') => true,
                ('7', 'L') => true,
                ('7', 'J') => true,
                ('F', '|') => true,
                ('F', 'L') => true,
                ('F', 'J') => true,
                _ => false,
            }
        }
        (0, -1) => {
            return match (from, to) {
                ('S', '|') => true,
                ('S', 'F') => true,
                ('S', '7') => true,
                ('|', '|') => true,
                ('|', 'F') => true,
                ('|', '7') => true,
                ('L', '|') => true,
                ('L', 'F') => true,
                ('L', '7') => true,
                ('J', '|') => true,
                ('J', 'F') => true,
                ('J', '7') => true,
                _ => false,
            }
        }
        _ => false,
    };
}
