use std::collections::HashSet;

#[derive(Debug)]
struct Matrix {
    matrix: Vec<char>,
    width: usize,
    size: usize,
}

impl Matrix {
    pub fn new(input: String) -> Self {
        let lines = input.lines();

        let width = lines.clone().peekable().peek().expect("no item :(").len();
        let height = lines.clone().count();

        return Matrix {
            matrix: input.chars().filter(|&c| c != '\n').collect(),
            width: width,
            size: width * height,
        };
    }

    fn find_numbers(&self) -> Vec<(i32, usize, usize)> {
        let mut numbers: Vec<(i32, usize, usize)> = Vec::new();

        let mut current_number: String = String::from("");
        let mut current_start = 0;

        for (index, value) in self.matrix.iter().enumerate() {
            if index + 1 % self.width == 0 || !value.is_digit(10) {
                if current_number.clone() != "" {
                    numbers.push((
                        current_number.clone().parse::<i32>().unwrap(),
                        current_start,
                        index,
                    ));
                    current_number = String::from("");
                }
                continue;
            }

            if current_number == "" {
                current_start = index;
            }
            current_number.push(*value);
        }

        return numbers;
    }

    fn find_stars(&self) -> Vec<usize> {
        return self
            .matrix
            .iter()
            .enumerate()
            .filter(|x| *x.1 == '*')
            .map(|x| x.0)
            .collect();
    }

    fn has_adjacent_symbol(&self, index: usize) -> bool {
        if !self.matrix.get(index).unwrap().is_digit(10) {
            return false;
        }

        let w = self.width as i32;
        let offsets: [i32; 8] = [-1, -w - 1, -w, -w + 1, 1, w - 1, w, w + 1];

        for offset in offsets {
            let adjacent_index = index as i32 + offset;

            if adjacent_index < 0 {
                continue;
            }

            if adjacent_index >= self.size as i32 {
                continue;
            }

            let value = self.matrix.get(adjacent_index as usize).unwrap();
            if !value.is_digit(10) && *value != '.' {
                return true;
            }
        }

        return false;
    }

    fn adjacent_number_indicies(&self, index: usize) -> Vec<usize> {
        let mut indicies: Vec<usize> = Vec::new();

        let w = self.width as i32;
        let offsets: [i32; 8] = [-1, -w - 1, -w, -w + 1, 1, w - 1, w, w + 1];

        for offset in offsets {
            let adjacent_index = index as i32 + offset;

            if adjacent_index < 0 {
                continue;
            }

            if adjacent_index >= self.size as i32 {
                continue;
            }

            let value = self.matrix.get(adjacent_index as usize).unwrap();
            if value.is_digit(10) {
                indicies.push(adjacent_index as usize);
            }
        }

        return indicies;
    }
}

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let m = Matrix::new(input);

    println!("part1: {}", part1(&m));
    println!("part2: {}", part2(&m));
}

fn part1(m: &Matrix) -> i32 {
    let mut sum = 0;
    for number in m.find_numbers() {
        for index in number.1..number.2 {
            if m.has_adjacent_symbol(index) {
                sum += number.0;
                break;
            }
        }
    }
    return sum;
}

fn part2(m: &Matrix) -> i32 {
    let numbers = m.find_numbers();

    let stars = m.find_stars();

    let mut sum = 0;

    for star_index in stars {
        let incidies = m.adjacent_number_indicies(star_index);

        let res: HashSet<(i32, i32)> = incidies
            .iter()
            .map(|x| -> (i32, i32) {
                for (index, value) in numbers.iter().enumerate() {
                    if *x >= value.1 && *x <= value.2 {
                        return (index as i32, value.0);
                    }
                }
                return (-1, -1);
            })
            .filter(|&x| x.0 != -1)
            .collect();

        if res.len() != 2 {
            continue;
        }

        sum += res.iter().fold(1, |acc, x| acc * x.1);
    }

    return sum;
}
