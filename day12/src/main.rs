use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut rows: Vec<(String, Vec<usize>)> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let numbers: Vec<usize> = parts[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        rows.push((parts[0].to_string(), numbers));
    }

    println!("part1: {}", part1(rows.clone()));
    println!("part2: {}", part2(rows.clone()));
}

fn part1(rows: Vec<(String, Vec<usize>)>) -> i64 {
    let mut sum = 0;
    for (word, numbers) in rows {
        let mut cache: HashMap<String, i64> = HashMap::new();
        sum += solve(word, numbers, &mut cache);
    }
    return sum;
}

fn part2(rows: Vec<(String, Vec<usize>)>) -> i64 {
    let mut sum = 0;
    for (word, numbers) in rows {
        let mut cache: HashMap<String, i64> = HashMap::new();

        let extend_word = format!("{}?{}?{}?{}?{}", word, word, word, word, word);

        let size = numbers.len() * 5;
        let extend_numbers = numbers.into_iter().cycle().take(size).collect();

        sum += solve(extend_word, extend_numbers, &mut cache);
    }
    return sum;
}

fn solve(word: String, numbers: Vec<usize>, cache: &mut HashMap<String, i64>) -> i64 {
    if word == "" {
        return if numbers.len() == 0 { 1 } else { 0 };
    }

    if numbers.len() == 0 {
        return if word.contains('#') { 0 } else { 1 };
    }

    let key = fingerprint(word.clone(), &numbers);
    match cache.get(&key) {
        Some(&value) => return value,
        None => {}
    }

    let mut sum = 0;
    let head = word.chars().next().unwrap();

    if head == '.' || head == '?' {
        sum += solve(word.chars().skip(1).collect(), numbers.clone(), cache);
    }

    if head == '#' || head == '?' {
        let &current_number = numbers.iter().next().unwrap();
        if word.len() >= current_number {
            if word.len() == current_number || word.chars().nth(current_number).unwrap() != '#' {
                if !word.get(0..current_number).unwrap().contains('.') {
                    sum += solve(
                        word.chars().skip(current_number + 1).collect(),
                        numbers.into_iter().skip(1).collect(),
                        cache,
                    );
                }
            }
        }
    }

    cache.insert(key, sum);

    return sum;
}

fn fingerprint(word: String, numbers: &Vec<usize>) -> String {
    return word.clone()
        + &numbers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
}
