fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let numbers: Vec<Vec<usize>> = input
        .lines()
        .map(|row| -> Vec<usize> {
            return row
                .split(' ')
                .map(|x| x.chars().filter(|x| x.is_numeric()).collect())
                .filter(|x: &String| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
        })
        .collect();

    let races: Vec<(usize, usize)> = numbers
        .get(0)
        .unwrap()
        .iter()
        .cloned()
        .zip(numbers.get(1).unwrap().iter().cloned())
        .collect();

    println!("part1: {}", solve(races.clone()));
    println!(
        "part2: {}",
        solve(vec![(
            merge_numbers(numbers.get(0).unwrap()),
            (merge_numbers(numbers.get(1).unwrap()))
        )])
    );
}

fn solve(races: Vec<(usize, usize)>) -> i32 {
    let mut sum = 1;
    for race in races {
        let mut wins = 0;
        for wait in 1..(race.0 - 1) {
            let distance = (race.0 - wait) * wait;
            if distance > race.1 {
                wins += 1;
            }
        }
        if wins == 0 {
            continue;
        }
        sum *= wins;
    }

    return sum;
}

fn merge_numbers(v: &Vec<usize>) -> usize {
    return v
        .iter()
        .map(|num| num.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
}
