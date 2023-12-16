use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let items: Vec<Vec<u8>> = input
        .split(',')
        .map(|x| {
            x.chars()
                .map(|ch| ch.to_ascii_lowercase())
                .map(|ch| ch as u8)
                .collect()
        })
        .collect();

    println!("part1: {:?}", part1(items.clone()));
    println!("part2: {:?}", part2(items.clone()));
}

fn part1(items: Vec<Vec<u8>>) -> usize {
    let mut total: usize = 0;
    for chars in items {
        total += hash(&chars);
    }
    return total;
}

fn part2(items: Vec<Vec<u8>>) -> usize {
    let mut lookup: HashMap<String, usize> = HashMap::new();

    let mut boxes: Vec<Vec<(String, u8)>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for item in items {
        let label = extract_label(&item);
        let box_index = hash(
            &label
                .chars()
                .map(|x| x.to_ascii_lowercase() as u8)
                .collect(),
        );

        let &last_char = item.iter().last().unwrap();

        if last_char >= 49 && last_char <= 57 {
            let value = last_char - 48;
            if lookup.get(&label).is_some() {
                let index = boxes[box_index].iter().position(|x| x.0 == label).unwrap();
                boxes[box_index][index] = (label.clone(), value);
            } else {
                lookup.insert(label.clone(), box_index);
                boxes[box_index].push((label.clone(), value));
            }
        } else {
            match boxes[box_index].iter().position(|x| x.0 == label) {
                Some(index) => {
                    let mut boxx = boxes[box_index].clone();
                    boxx.remove(index);
                    boxes[box_index] = boxx;
                    lookup.remove(&label);
                }
                None => {}
            }
        }
    }

    let mut total = 0;
    for (box_index, items) in boxes.iter().enumerate() {
        let mut sum = 0;
        for (item_index, item) in items.iter().enumerate() {
            sum += (box_index + 1) * (item_index + 1) * item.1 as usize;
        }
        total += sum;
    }

    return total;
}

fn hash(chars: &Vec<u8>) -> usize {
    let mut sum: usize = 0;
    for code in chars {
        let value = code.clone();
        sum += value as usize;
        sum *= 17;
        sum %= 256;
    }
    return sum;
}

fn extract_label(chars: &Vec<u8>) -> String {
    return chars
        .iter()
        .map(|x| *x)
        .filter(|&x| x >= 97)
        .map(|x| x as char)
        .collect();
}
