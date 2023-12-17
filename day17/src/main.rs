use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut data: HashMap<(i32, i32), u32> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            data.insert((x as i32, y as i32), ch.to_digit(10).unwrap());
        }
    }

    let size = input.lines().count();

    // part1(&data, size);}
    part2(&data, size);
}

// WHY is this so crazy slow?
fn part1(grid: &HashMap<(i32, i32), u32>, grid_size: usize) {
    let mut queue: BinaryHeap<Reverse<(u32, (i32, i32), (i32, i32), i32)>> = BinaryHeap::new();
    let mut seen: HashSet<(u32, (i32, i32), (i32, i32), i32)> = HashSet::new();

    queue.push(Reverse((0, (0, 0), (0, 0), 0)));

    loop {
        let head = queue.pop().unwrap();

        let score = head.0 .0;
        let position = head.0 .1;
        let delta = head.0 .2;
        let count = head.0 .3;

        if seen.get(&(score, position, delta, count)).is_some() {
            println!("seen: {:?}", head);
            continue;
        }
        seen.insert((score, position, delta, count));

        if position == (grid_size as i32 - 1, grid_size as i32 - 1) {
            println!("score: {:?}", score);
            break;
        }

        if count < 3 && delta != (0, 0) {
            let new_position = (position.0 + delta.0, position.1 + delta.1);
            if is_inside(new_position, grid_size) {
                let &grid_value = grid.get(&new_position).unwrap();
                queue.push(Reverse((
                    score + grid_value,
                    new_position,
                    delta,
                    count + 1,
                )));
            }
        }

        for next in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if delta.0 == -1 * next.0 && delta.1 == -1 * next.1 {
                continue;
            }

            if next == delta {
                continue;
            }

            let new_position = (position.0 + next.0, position.1 + next.1);
            if is_inside(new_position, grid_size) {
                let &grid_value = grid.get(&new_position).unwrap();
                queue.push(Reverse((score + grid_value, new_position, next, 1)));
            }
        }
    }
}

fn is_inside(coord: (i32, i32), size: usize) -> bool {
    return coord.0 >= 0 && coord.0 < size as i32 && coord.1 >= 0 && coord.1 < size as i32;
}

fn part2(grid: &HashMap<(i32, i32), u32>, grid_size: usize) {
    let mut queue: BinaryHeap<Reverse<(u32, (i32, i32), (i32, i32), i32)>> = BinaryHeap::new();
    let mut seen: HashSet<(u32, (i32, i32), (i32, i32), i32)> = HashSet::new();

    queue.push(Reverse((0, (0, 0), (0, 0), 0)));

    loop {
        let head = queue.pop().unwrap();

        let score = head.0 .0;
        let position = head.0 .1;
        let delta = head.0 .2;
        let count = head.0 .3;

        if seen.get(&(score, position, delta, count)).is_some() {
            println!("seen: {:?}", head);
            continue;
        }
        seen.insert((score, position, delta, count));

        if position == (grid_size as i32 - 1, grid_size as i32 - 1) {
            println!("score: {:?}", score);
            break;
        }

        if count < 10 && delta != (0, 0) {
            let new_position = (position.0 + delta.0, position.1 + delta.1);
            if is_inside(new_position, grid_size) {
                let &grid_value = grid.get(&new_position).unwrap();
                queue.push(Reverse((
                    score + grid_value,
                    new_position,
                    delta,
                    count + 1,
                )));
            }
        }

        if count < 4 && delta != (0, 0) {
            continue;
        }

        for next in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if delta.0 == -1 * next.0 && delta.1 == -1 * next.1 {
                continue;
            }

            if next == delta {
                continue;
            }

            let new_position = (position.0 + next.0, position.1 + next.1);
            if is_inside(new_position, grid_size) {
                let &grid_value = grid.get(&new_position).unwrap();
                queue.push(Reverse((score + grid_value, new_position, next, 1)));
            }
        }
    }
}
