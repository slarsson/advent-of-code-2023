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
}

fn part1(grid: &HashMap<(i32, i32), char>) -> i32 {
    let path = resolve_path(grid);
    return (path.len() as i32 + 1) / 2;
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

// ..........
// .S------7.
// .|F----7|.
// .||....||.
// .||....||.
// .|L-7F-J|.
// .|..||..|.
// .L--JL--J.
// ..........

// fn next_direction(current: Direction, next_tile: char) -> Direction {
//     return match current {
//         Direction::Up => match next_tile {
//             'F' => Direction::Right,
//             '7' => Direction::Left,
//             '|' => Direction::Up,
//             _ => panic!("noop"),
//         },
//         Direction::Down => match next_tile {
//             'L' => Direction::Right,
//             'J' => Direction::Left,
//             '|' => Direction::Down,
//             _ => panic!("noop"),
//         },
//         Direction::Left => match next_tile {
//             'F' => Direction::Down,
//             'L' => Direction::Up,
//             '-' => Direction::Left,
//             _ => panic!("noop"),
//         },
//         Direction::Right => match next_tile {
//             '7' => Direction::Down,
//             'J' => Direction::Up,
//             '-' => Direction::Right,
//             _ => panic!("noop"),
//         },
//     };
// }

// https://imgur.com/a/ukstWKO#ZKurCuH

// fn print_grid(
//     grid: &HashMap<(i32, i32), char>,
//     border: &HashSet<(i32, i32)>,
//     seen: &HashSet<(i32, i32)>,
//     width: usize,
//     height: usize,
// ) {
//     for y in 0..height + 2 {
//         let mut line = String::new();
//         for x in 0..width + 2 {
//             let coord = &((x as i32) - 1, (y as i32) - 1);

//             let border_coord = border.get(coord);
//             let grid_coord = grid.get(coord);
//             let seen_coord = seen.get(coord);

//             if border_coord.is_some() {
//                 line.push('X');
//             } else if seen_coord.is_some() {
//                 line.push('@');
//             } else if grid_coord.is_some() {
//                 line.push(*grid_coord.unwrap());
//             } else {
//                 line.push('_');
//             }
//         }
//         println!("{}", line);
//     }
// }
