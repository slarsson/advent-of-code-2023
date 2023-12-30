use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let nodes: HashMap<String, Rc<RefCell<Node>>> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");

            let (name, typ) = parse_name(parts.next().unwrap());

            let receivers: Vec<String> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            return (
                name.to_string(),
                Rc::new(RefCell::new(Node::new(name.to_string(), typ, receivers))),
            );
        })
        .collect();

    for (name, node) in &nodes {
        let the_node = node.borrow();
        for receiver in &the_node.receivers {
            if nodes.get(receiver).is_none() {
                continue;
            }

            let mut x = nodes.get(receiver).unwrap().borrow_mut();
            x.add_input(&name);
        }
    }

    println!("part1: {}", part1(nodes.clone()));
    println!("part2: {}", part2(nodes.clone()));
}

fn parse_name(s: &str) -> (&str, Option<char>) {
    if s.starts_with(['%', '&']) {
        let ch = s.chars().next();
        let name = &s[1..];
        return (name, ch);
    }
    return (s, None);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Default,
}

#[derive(Debug, Clone)]
struct Node {
    module_type: Module,
    receivers: Vec<String>,
    inputs: HashMap<String, Pulse>,
    alive: bool,
}

impl Node {
    fn new(name: String, typ: Option<char>, receivers: Vec<String>) -> Self {
        let module_type: Module = match typ {
            Some(value) => match value {
                '%' => Module::FlipFlop,
                '&' => Module::Conjunction,
                _ => panic!("noop"),
            },
            None => {
                if name.eq("broadcaster") {
                    Module::Broadcaster
                } else {
                    Module::Default
                }
            }
        };

        return Node {
            module_type: module_type,
            receivers: receivers,
            inputs: HashMap::new(),
            alive: false,
        };
    }

    fn add_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string(), Pulse::Low);
    }

    fn receive(&mut self, sender: String, pulse: Pulse) -> Vec<(String, Pulse)> {
        self.inputs.insert(sender, pulse);

        match self.module_type {
            Module::Broadcaster => {
                return self.receivers.iter().map(|r| (r.clone(), pulse)).collect();
            }
            Module::FlipFlop => {
                if pulse == Pulse::Low {
                    let out = if self.alive { Pulse::Low } else { Pulse::High };
                    self.alive = !self.alive;
                    return self.receivers.iter().map(|r| (r.clone(), out)).collect();
                }
                return Vec::new();
            }
            Module::Conjunction => {
                let all = self.inputs.iter().all(|(_, &v)| v == Pulse::High);

                let out = if all { Pulse::Low } else { Pulse::High };

                return self.receivers.iter().map(|r| (r.clone(), out)).collect();
            }
            _ => {
                return Vec::new();
            }
        }
    }
}

fn part1(nodes: HashMap<String, Rc<RefCell<Node>>>) -> i32 {
    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let mut broadcaster = nodes
            .get(&String::from("broadcaster"))
            .unwrap()
            .borrow_mut();

        low_count += 1;

        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();

        for (to, pulse) in broadcaster.receive("button".to_string(), Pulse::Low) {
            queue.push_back(("broadcaster".to_string(), to, pulse));
        }

        loop {
            if queue.len() == 0 {
                break;
            }

            let (from, to, pulse) = queue.pop_front().unwrap();

            match pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            }

            if nodes.get(&to).is_none() {
                continue;
            }

            let mut node = nodes.get(&to).unwrap().borrow_mut();

            for (next_to, next_pulse) in node.receive(from, pulse) {
                queue.push_back((to.clone(), next_to, next_pulse));
            }
        }
    }

    return low_count * high_count;
}

// tg -> (ln, db, vq, tf)
fn part2(nodes: HashMap<String, Rc<RefCell<Node>>>) -> i64 {
    let mut ln: Vec<i64> = Vec::new();
    let mut db: Vec<i64> = Vec::new();
    let mut vq: Vec<i64> = Vec::new();
    let mut tf: Vec<i64> = Vec::new();

    for click in 1..20000 {
        let mut broadcaster = nodes
            .get(&String::from("broadcaster"))
            .unwrap()
            .borrow_mut();

        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();

        for (to, pulse) in broadcaster.receive("button".to_string(), Pulse::Low) {
            queue.push_back(("broadcaster".to_string(), to, pulse));
        }

        loop {
            if queue.len() == 0 {
                break;
            }

            let (from, to, pulse) = queue.pop_front().unwrap();

            if from == "ln" && pulse == Pulse::High {
                ln.push(click);
            }

            if from == "db" && pulse == Pulse::High {
                db.push(click);
            }

            if from == "vq" && pulse == Pulse::High {
                vq.push(click);
            }

            if from == "tf" && pulse == Pulse::High {
                tf.push(click);
            }

            if nodes.get(&to).is_none() {
                continue;
            }

            let mut node = nodes.get(&to).unwrap().borrow_mut();

            for (next_to, next_pulse) in node.receive(from, pulse) {
                queue.push_back((to.clone(), next_to, next_pulse));
            }
        }
    }

    return calculate_lcm(vec![
        ln[1] - ln[0],
        db[1] - db[0],
        vq[1] - vq[0],
        tf[1] - tf[0],
    ]);
}

//
// copy from day 8
//

fn calculate_lcm(numbers: Vec<i64>) -> i64 {
    let a = numbers.get(0).unwrap().clone();
    let b = numbers.get(1).unwrap().clone();

    let mut res = lcm(a, b);
    for number in numbers.into_iter().skip(2) {
        res = lcm(res, number);
    }
    return res;
}

// https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(a: i64, b: i64) -> i64 {
    return (a.abs() * b.abs()) / gcd(a, b);
}

// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            break;
        }

        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}
