fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let dig_plan_part1: Vec<(char, i64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');

            let dir = parts.next().unwrap().chars().next().unwrap();
            let distance = parts.next().unwrap().parse::<i64>().unwrap();

            return (dir, distance);
        })
        .collect();

    let dig_plan_part2: Vec<(char, i64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');

            let hex = parts.nth(2).unwrap().replace(['#', '(', ')'], "");
            let distance = &hex[0..5];
            let dir = match hex.chars().last().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("noop"),
            };
            return (dir, i64::from_str_radix(distance, 16).unwrap());
        })
        .collect();

    println!("part1: {}", solve(dig_plan_part1.clone()));
    println!("part2: {}", solve(dig_plan_part2.clone()));
}

fn solve(dig_plan: Vec<(char, i64)>) -> i64 {
    let mut edge: Vec<(i64, i64)> = Vec::new();
    let mut edge_size = 0;

    let mut cursor: (i64, i64) = (0, 0);
    for (direction, distance) in dig_plan {
        let change = match direction {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!("noop"),
        };

        let next = (
            cursor.0 + distance * change.0,
            cursor.1 + distance * change.1,
        );

        edge.push(next);
        edge_size += distance;

        cursor = next;
    }

    assert!(cursor == (0, 0));

    // https://en.wikipedia.org/wiki/Shoelace_formula does not include "full" border area
    let mut area = 0;
    for window in edge.windows(2) {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];
        area += (x1 * y2) - (x2 * y1);
    }
    area /= 2;

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b/2 - 1, i = points inside polygon, b = points on edge

    let inside_area = (area + 1) - (edge_size / 2);

    return inside_area + edge_size;
}
