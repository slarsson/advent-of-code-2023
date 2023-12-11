use std::collections::HashSet;

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                grid.insert((x as i32, y as i32));
            }
        }
    }

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut empty_rows: Vec<i32> = Vec::new();
    'outer: for y in 0..height {
        for x in 0..width {
            if grid.get(&(x as i32, y as i32)).is_some() {
                continue 'outer;
            }
        }
        empty_rows.push(y as i32);
    }

    let mut empty_cols: Vec<i32> = Vec::new();
    'outer: for x in 0..width {
        for y in 0..height {
            if grid.get(&(x as i32, y as i32)).is_some() {
                continue 'outer;
            }
        }
        empty_cols.push(x as i32);
    }

    println!(
        "part1: {}",
        part1(grid.clone(), empty_rows.clone(), empty_cols.clone())
    );
    println!(
        "part2: {}",
        part2(grid.clone(), empty_rows.clone(), empty_cols.clone())
    );
}

fn part1(grid: HashSet<(i32, i32)>, empty_rows: Vec<i32>, empty_cols: Vec<i32>) -> i32 {
    let mut grid_large: Vec<(i32, i32)> = Vec::new();

    for coord in grid.iter() {
        let x = coord.0 + empty_cols.iter().filter(|&&x| x < coord.0).count() as i32;
        let y = coord.1 + empty_rows.iter().filter(|&&y| y < coord.1).count() as i32;
        grid_large.push((x, y));
    }

    let mut sum = 0;
    for (index, a) in grid_large.iter().enumerate() {
        for b in grid_large.iter().skip(index + 1) {
            sum += manhattan(a, b);
        }
    }

    return sum;
}

fn part2(grid: HashSet<(i32, i32)>, empty_rows: Vec<i32>, empty_cols: Vec<i32>) -> i64 {
    let scale = 1000000 - 1;
    let mut grid_large: Vec<(i32, i32)> = Vec::new();

    for coord in grid.iter() {
        let x = coord.0 + scale * empty_cols.iter().filter(|&&x| x < coord.0).count() as i32;
        let y = coord.1 + scale * empty_rows.iter().filter(|&&y| y < coord.1).count() as i32;
        grid_large.push((x, y));
    }

    let mut sum = 0;
    for (index, a) in grid_large.iter().enumerate() {
        for b in grid_large.iter().skip(index + 1) {
            sum += manhattan(a, b) as i64;
        }
    }

    return sum;
}

fn manhattan(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}
