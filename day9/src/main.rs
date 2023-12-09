fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let rows: Vec<Vec<i64>> = input
        .lines()
        .map(|s| s.split(' ').map(|v| v.parse::<i64>().unwrap()).collect())
        .collect();

    println!("part1: {}", part1(rows.clone()));
    println!("part2: {}", part2(rows.clone()));
}

fn part1(rows: Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for row in rows {
        sum += row.iter().last().unwrap() + solve(&row)
    }
    return sum;
}

fn part2(rows: Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for row in rows {
        let row_rev: Vec<i64> = row.iter().rev().cloned().collect();
        sum += row_rev.iter().last().unwrap() + solve(&row_rev)
    }
    return sum;
}

fn solve(sequence: &Vec<i64>) -> i64 {
    if !sequence.iter().any(|&x| x != 0) {
        return 0;
    }

    let next_sequence: Vec<i64> = sequence
        .iter()
        .zip(sequence.iter().skip(1).clone())
        .map(|(&a, &b)| b - a)
        .collect();

    return next_sequence.iter().last().unwrap() + solve(&next_sequence);
}
