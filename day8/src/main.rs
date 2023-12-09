use std::collections::{HashMap, HashSet};

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let directions: Vec<char> = input
        .split("\n\n")
        .into_iter()
        .next()
        .unwrap()
        .chars()
        .collect();

    let table: HashMap<String, (String, String)> = input
        .split("\n\n")
        .into_iter()
        .last()
        .unwrap()
        .lines()
        .map(|x| {
            let row: Vec<String> = x
                .replace(['=', '(', ')', ','], "")
                .split_whitespace()
                .filter(|c| !c.is_empty())
                .map(|s| s.to_string())
                .collect();
            return (
                row.get(0).unwrap().to_string(),
                (
                    row.get(1).unwrap().to_string(),
                    row.get(2).unwrap().to_string(),
                ),
            );
        })
        .collect();

    println!("part1: {}", part1(&directions, &table));
    println!("part1: {}", part2(&directions, &table));
}

fn part1(directions: &Vec<char>, table: &HashMap<String, (String, String)>) -> i32 {
    let mut cursor = String::from("AAA");
    let mut count = 0;

    let mut directions_cycle_iter = directions.iter().cycle();

    loop {
        if cursor == "ZZZ" {
            break;
        }

        let dir = directions_cycle_iter.next().unwrap();

        let node = table.get(&cursor).unwrap();

        match dir {
            'L' => cursor = node.0.clone(),
            'R' => cursor = node.1.clone(),
            _ => panic!("noop"),
        }

        count += 1
    }

    return count;
}

fn part2(directions: &Vec<char>, table: &HashMap<String, (String, String)>) -> i64 {
    let cursors: Vec<String> = table
        .iter()
        .map(|(k, _)| k.to_string())
        .filter(|s| s.ends_with('A'))
        .collect();

    let mut directions_cycle_iter = directions.iter().cycle();
    let mut hits: HashSet<i64> = HashSet::new();

    for c in cursors {
        let mut cursor = c.clone();
        let mut count = 0;
        let mut first_hit: Option<usize> = None;

        // assume only one "valid" hit per cursor :)
        loop {
            let dir = directions_cycle_iter.next().unwrap();

            let node = table.get(&cursor).unwrap();

            match dir {
                'L' => cursor = node.0.clone(),
                'R' => cursor = node.1.clone(),
                _ => panic!("noop"),
            }

            count += 1;

            if cursor.ends_with("Z") && count > directions.len() {
                match first_hit {
                    Some(value) => {
                        hits.insert(value as i64);
                        break;
                    }
                    None => first_hit = Some(count),
                }
            }
        }
    }

    return calculate_lcm(hits.into_iter().collect());
}

fn calculate_lcm(numbers: Vec<i64>) -> i64 {
    let a = numbers.get(0).unwrap().clone();
    let b = numbers.get(1).unwrap().clone();

    let mut res = lcm(a, b);
    for number in numbers.into_iter().skip(2) {
        res = lcm(res, number);
    }
    return res;
}

// https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(a: i64, b: i64) -> i64 {
    return (a.abs() * b.abs()) / gcd(a, b);
}

// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            break;
        }

        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}
