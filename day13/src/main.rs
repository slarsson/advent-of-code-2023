fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let cards: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|block| {
            return block.lines().map(|row| row.chars().collect()).collect();
        })
        .collect();

    println!("part1: {}", part1(cards.clone()));
    println!("part2: {}", part2(cards.clone()));
}

fn part1(cards: Vec<Vec<Vec<char>>>) -> usize {
    let mut sum = 0;

    for card in cards {
        sum += 100 * get_single_line_or_panic(&card);
        sum += get_single_line_or_panic(&transpose(&card));
    }

    return sum;
}

fn part2(cards: Vec<Vec<Vec<char>>>) -> usize {
    let mut sum = 0;

    for mut card in cards.clone() {
        let height = card.len();
        let width = card[0].len();

        let o_row = get_single_line_or_panic(&card);
        let o_col = get_single_line_or_panic(&transpose(&card));

        'outer: for y in 0..height {
            for x in 0..width {
                card[y][x] = flip(card[y][x]);

                let row = get_single_line_or_panic_with_exclude(&card, o_row);
                if row != 0 {
                    sum += 100 * row;
                    break 'outer;
                }

                let col = get_single_line_or_panic_with_exclude(&transpose(&card), o_col);
                if col != 0 {
                    sum += col;
                    break 'outer;
                }

                card[y][x] = flip(card[y][x]);
            }
        }
    }

    return sum;
}

fn get_single_line_or_panic(card: &Vec<Vec<char>>) -> usize {
    let hits = detect_mirrors(card);
    if hits.len() > 0 {
        assert!(hits.len() == 1);
        return hits[0];
    }
    return 0;
}

fn get_single_line_or_panic_with_exclude(card: &Vec<Vec<char>>, exclude: usize) -> usize {
    let hits: Vec<usize> = detect_mirrors(card);
    if hits.len() > 0 {
        assert!(hits.len() == 1 || hits.len() == 2);
        return match hits.iter().find(|&&x| x != exclude) {
            Some(&v) => v,
            None => 0,
        };
    }
    return 0;
}

fn detect_mirrors(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let mut lines: Vec<usize> = Vec::new();

    for index in 0..matrix.len() - 1 {
        let split_before = index + 1;
        let (first, second) = matrix.split_at(split_before);
        if first.iter().rev().zip(second).all(|v| v.0 == v.1) {
            lines.push(split_before);
        }
    }

    return lines;
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = matrix[0].len();
    let height = matrix.len();

    let mut transposed: Vec<Vec<char>> = vec![vec!['.'; height]; width];

    for y in 0..height {
        for x in 0..width {
            transposed[x][y] = matrix[y][x];
        }
    }

    return transposed;
}

fn flip(ch: char) -> char {
    if ch == '.' {
        return '#';
    }
    return '.';
}
