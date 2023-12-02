fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("no file 4 you");
    let games: Vec<Vec<Vec<(u8, u8)>>> = input.lines().map(|s| parse_row(s)).collect();

    println!("part1: {}", part1(games.clone()));
    println!("part2: {}", part2(games.clone()));
}

fn part1(games: Vec<Vec<Vec<(u8, u8)>>>) -> i32 {
    let max_limits: Vec<(u8, u8)> = vec![(12, 0), (13, 1), (14, 2)];

    let mut sum: i32 = 0;

    for (index, game) in games.iter().enumerate() {
        let res = game
            .iter()
            .map(|set| compare(set.clone(), max_limits.clone()))
            .fold(true, |acc, xx| acc && xx);

        if res {
            sum += index as i32 + 1;
        }
    }

    return sum;
}

fn part2(games: Vec<Vec<Vec<(u8, u8)>>>) -> i32 {
    return games
        .iter()
        .map(|game| -> i32 {
            let max_values = game.iter().flatten().fold((0, 0, 0), |acc, x| match x.1 {
                0 => {
                    if x.0 > acc.0 {
                        return (x.0, acc.1, acc.2);
                    }
                    return acc;
                }
                1 => {
                    if x.0 > acc.1 {
                        return (acc.0, x.0, acc.2);
                    }
                    return acc;
                }
                2 => {
                    if x.0 > acc.2 {
                        return (acc.0, acc.1, x.0);
                    }
                    return acc;
                }
                _ => panic!("noop"),
            });
            return max_values.0 as i32 * max_values.1 as i32 * max_values.2 as i32;
        })
        .fold(0, |acc, x| acc + x);
}

fn compare(current_set: Vec<(u8, u8)>, max_limits: Vec<(u8, u8)>) -> bool {
    return current_set
        .iter()
        .map(|set_item: &(u8, u8)| -> bool {
            max_limits
                .iter()
                .map(|max_limit_item| {
                    if set_item.1 != max_limit_item.1 {
                        return true;
                    }
                    return set_item.0 <= max_limit_item.0;
                })
                .fold(true, |acc, x| acc && x)
        })
        .fold(true, |acc, x| acc && x);
}

fn parse_row(str: &str) -> Vec<Vec<(u8, u8)>> {
    let sets: Vec<&str> = str
        .split([':', ';'])
        .enumerate()
        .filter(|&(index, _)| index != 0)
        .map(|(_, set)| set.trim())
        .collect();

    return sets
        .iter()
        .map(|set| -> Vec<(u8, u8)> {
            return set
                .split(",")
                .map(|s| s.trim().split(" "))
                .map(|value| parse_row_set(value.collect()))
                .collect();
        })
        .collect();
}

fn parse_row_set(pair: Vec<&str>) -> (u8, u8) {
    if pair.len() != 2 {
        panic!("expected two items, got {}", pair.len())
    }

    let count = pair[0].parse::<u8>().expect("NaN");

    let cube_type = match pair[1] {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => panic!("not red, blue or green"),
    };

    return (count, cube_type);
}
