fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("no file 4 you");
    let rows: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    println!("part1: {}", part1(rows.clone()));
    println!("part2: {}", part2(rows.clone()));
}

fn part1(rows: Vec<String>) -> i32 {
    let mut sum = 0;

    for row in rows {
        let numbers: Vec<char> = row.chars().filter(|c| c.is_digit(10)).collect();

        let first = numbers.first().expect("no first value :(");
        let last = numbers.last().expect("no last value :(");

        sum += format!("{}{}", first, last).parse::<i32>().expect("NaN");
    }

    return sum;
}

fn part2(rows: Vec<String>) -> i32 {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;

    for row in rows {
        let chars: Vec<char> = row.chars().collect();
        let mut numbers: Vec<&str> = Vec::new();

        for (index, char) in chars.iter().enumerate() {
            if char.is_digit(10) {
                numbers.push(char_to_letters(char));
                continue;
            }

            for word in words {
                let end = index + word.len();
                if end > chars.len() {
                    continue;
                }

                let chunk = &chars[index..end];
                let chunk_as_string = chunk.iter().collect::<String>();

                if chunk_as_string == word {
                    numbers.push(word);
                }
            }
        }

        let first = numbers.first().expect("no first value :(");
        let last = numbers.last().expect("no last value :(");

        sum += format!("{}{}", letters_to_char(first), letters_to_char(last))
            .parse::<i32>()
            .expect("NaN");
    }

    return sum;
}

fn letters_to_char(str: &str) -> char {
    match str {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '\0',
    }
}

fn char_to_letters(c: &char) -> &str {
    match c {
        '1' => "one",
        '2' => "two",
        '3' => "three",
        '4' => "four",
        '5' => "five",
        '6' => "six",
        '7' => "seven",
        '8' => "eight",
        '9' => "nine",
        _ => "",
    }
}
