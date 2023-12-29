use std::collections::HashSet;

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut bricks: Vec<Brick> = input
        .lines()
        .map(|line| {
            let mut parts = line.split('~');
            let a = list_to_numbers(parts.next().unwrap());
            let b = list_to_numbers(parts.next().unwrap());
            return Brick::new(a, b);
        })
        .collect();

    move_bricks_to_bottom(&mut bricks);

    println!("part1: {}", part1(bricks.clone()));
    println!("part2: {}", part2(bricks.clone()));
}

fn list_to_numbers(s: &str) -> (i32, i32, i32) {
    let numbers: Vec<i32> = s.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
    return (numbers[0], numbers[1], numbers[2]);
}

fn move_bricks_to_bottom(bricks: &mut Vec<Brick>) {
    loop {
        let mut moves = 0;

        for i in 0..bricks.len() {
            if !bricks[i].is_blocked(&bricks) {
                bricks[i].move_down();
                moves += 1;
            }
        }

        if moves == 0 {
            break;
        }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    dimension: (i32, i32, i32),
    position: (i32, i32, i32), // "lower" corner
}

impl Brick {
    fn new(a: (i32, i32, i32), b: (i32, i32, i32)) -> Self {
        let (x1, x2) = (a.0.min(b.0), a.0.max(b.0));
        let (y1, y2) = (a.1.min(b.1), a.1.max(b.1));
        let (z1, z2) = (a.2.min(b.2), a.2.max(b.2));

        let dimension = (x2 - x1 + 1, y2 - y1 + 1, z2 - z1 + 1);

        Brick {
            dimension: dimension,
            position: (x1, y1, z1),
        }
    }

    fn vertices(&self) -> ((i32, i32), (i32, i32), (i32, i32)) {
        return (
            (self.position.0, self.position.0 + self.dimension.0 - 1),
            (self.position.1, self.position.1 + self.dimension.1 - 1),
            (self.position.2, self.position.2 + self.dimension.2 - 1),
        );
    }

    fn is_blocked_by(&self, other: &Brick) -> bool {
        let (x1, y1, z1) = self.vertices();
        let (x2, y2, z2) = other.vertices();

        return ((x1.0 <= x2.0 && x2.0 <= x1.1) || (x2.0 <= x1.0 && x1.0 <= x2.1))
            && ((y1.0 <= y2.0 && y2.0 <= y1.1) || (y2.0 <= y1.0 && y1.0 <= y2.1))
            && z1.0 - 1 == z2.1;
    }

    fn is_blocked(&self, bricks: &Vec<Brick>) -> bool {
        if self.is_on_bottom() {
            return true;
        }

        for brick in bricks {
            if self == brick {
                continue;
            }

            if self.is_blocked_by(brick) {
                return true;
            }
        }
        return false;
    }

    fn is_on_bottom(&self) -> bool {
        return self.position.2 == 1;
    }

    fn blocking(&self, bricks: &Vec<Brick>) -> Vec<Brick> {
        if self.is_on_bottom() {
            return Vec::new();
        }

        let mut out: Vec<Brick> = Vec::new();
        for brick in bricks {
            if self == brick {
                continue;
            }

            if self.is_blocked_by(brick) {
                out.push(brick.clone())
            }
        }

        return out;
    }

    fn supporting(&self, bricks: &Vec<Brick>) -> Vec<Brick> {
        let mut out: Vec<Brick> = Vec::new();
        for brick in bricks {
            if self == brick {
                continue;
            }

            if brick.is_on_bottom() {
                continue;
            }

            if brick.is_blocked_by(self) {
                out.push(brick.clone())
            }
        }

        return out;
    }

    fn move_down(&mut self) {
        self.position.2 = (self.position.2 - 1).max(1);
    }
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        return self.position == other.position && self.dimension == other.dimension;
    }
}

fn part1(bricks: Vec<Brick>) -> i32 {
    let mut can_move = 0;
    for brick in &bricks {
        let supporting_bricks = brick.supporting(&bricks);

        if supporting_bricks.len() == 0 {
            can_move += 1;
            continue;
        }

        if supporting_bricks
            .iter()
            .all(|supporting_brick| supporting_brick.blocking(&bricks).len() >= 2)
        {
            can_move += 1;
        }
    }

    return can_move;
}

fn part2(bricks: Vec<Brick>) -> usize {
    let mut total = 0;
    for i in 0..bricks.len() {
        let mut state = bricks.clone();

        state.remove(i);

        let mut effected_bricks: HashSet<usize> = HashSet::new();

        loop {
            let mut moves = 0;

            for j in 0..state.len() {
                if !state[j].is_blocked(&state) {
                    effected_bricks.insert(j);
                    state[j].move_down();
                    moves += 1;
                }
            }

            if moves == 0 {
                break;
            }
        }

        total += effected_bricks.len();
    }

    return total;
}
