use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    grid.insert((0, 0));

    let mut start: (i32, i32) = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start = (x as i32, y as i32),
                '#' => {
                    grid.insert((x as i32, y as i32));
                }
                _ => {}
            };
        }
    }

    println!("part1: {}", part1(&grid, start, 65));
}

fn part1(grid: &HashSet<(i32, i32)>, start: (i32, i32), target: i32) -> usize {
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start.0, start.1)));

    loop {
        if queue.len() == 0 {
            break;
        }

        let head = queue.pop().unwrap().0;

        if visited.get(&head).is_some() {
            continue;
        }
        visited.insert(head);

        if head.0 == target {
            continue;
        }

        if visited.get(&(head.0 - 1, head.1, head.2)).is_some() {
            continue;
        }

        for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = (head.1 + delta.0, head.2 + delta.1);

            if grid.get(&next).is_some() {
                continue;
            }

            queue.push(Reverse((head.0 + 1, next.0, next.1)));
        }
    }

    return visited.iter().filter(|x| x.0 == target).count();
}
