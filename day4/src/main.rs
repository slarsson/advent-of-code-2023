use std::collections::VecDeque;

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let games: Vec<(Vec<i32>, Vec<i32>)> = input
        .lines()
        .map(|s| -> (Vec<i32>, Vec<i32>) {
            let values: Vec<Vec<i32>> = s
                .split([':', '|'])
                .enumerate()
                .filter(|&(index, _)| index != 0)
                .map(|(_, x)| {
                    x.split(' ')
                        .filter(|xx| !xx.is_empty())
                        .map(|xx| xx.parse::<i32>().unwrap())
                        .collect()
                })
                .collect();
            return (
                values.first().unwrap().clone(),
                values.last().unwrap().clone(),
            );
        })
        .collect();

    println!("part1: {}", part1(games.clone()));
    println!("part2: {}", part2(games.clone()));
}

fn part1(games: Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let scores: Vec<i32> = games
        .iter()
        .map(|x| -> i32 {
            let count =
                x.0.iter()
                    .filter(|have_number| x.1.contains(have_number))
                    .count() as i32;

            match count {
                0 => return 0,
                1 => return 1,
                _x => return i32::pow(2, count as u32 - 1),
            }
        })
        .collect();
    return scores.iter().fold(0, |acc, x| acc + x);
}

fn part2(games: Vec<(Vec<i32>, Vec<i32>)>) -> usize {
    let mut cards: VecDeque<usize> = games.iter().enumerate().map(|(index, _)| index).collect();

    let scores: Vec<usize> = games
        .iter()
        .map(|x| {
            return x
                .0
                .iter()
                .filter(|have_number| x.1.contains(have_number))
                .count();
        })
        .collect();

    let mut sum = 0;
    loop {
        if cards.len() == 0 {
            break;
        }

        let head = cards.pop_front().unwrap();
        let wins = scores.get(head).unwrap();

        if *wins == 0 {
            continue;
        }

        // lol......
        let mut new_games: VecDeque<usize> = (head + 1..(head + 1 + wins)).collect();
        sum += new_games.len();
        cards.append(&mut new_games);
    }

    return sum + games.len();
}
