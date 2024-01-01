use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input: String = std::fs::read_to_string("../input.txt").expect("no file 4 you");

    let mut blocks = input.split("\n\n");

    let pipelines = blocks.next().unwrap();
    let parts = blocks.next().unwrap();

    let mut all_rules: HashMap<String, Vec<Rule>> = HashMap::new();

    for pipeline in pipelines.lines() {
        let mut v = pipeline.split('{');

        let name = v.next().unwrap();

        let rules: Vec<Rule> = v
            .next()
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(',')
            .map(|r| Rule::new(r))
            .collect();

        all_rules.insert(name.to_string(), rules);
    }

    let mut all_parts: Vec<Part> = Vec::new();

    for part in parts.lines() {
        let chars_to_trim = ['x', 'm', 'a', 's', '{', '}', '='];
        let filtered: String = part
            .chars()
            .filter(|c| !chars_to_trim.contains(&c))
            .collect();

        let mut values = filtered.split(',');

        all_parts.push(Part {
            x: values.next().unwrap().parse::<i32>().unwrap(),
            m: values.next().unwrap().parse::<i32>().unwrap(),
            a: values.next().unwrap().parse::<i32>().unwrap(),
            s: values.next().unwrap().parse::<i32>().unwrap(),
        })
    }

    println!("part1: {}", part1(all_rules.clone(), all_parts.clone()));
    println!("part2: {}", part2(all_rules.clone()));
}

#[derive(Debug, Clone)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum Action {
    LessThan,
    GreaterThan,
    Forward,
}

#[derive(Debug, Clone)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
struct Rule {
    rule_type: Action,
    next: String,
    value: Option<i32>,
    field: Option<Field>,
}

impl Rule {
    fn new(rule: &str) -> Self {
        if !rule.contains(':') {
            return Rule {
                rule_type: Action::Forward,
                next: rule.to_string(),
                value: None,
                field: None,
            };
        }

        let mut parts = rule.split(':');

        let cond = parts.next().unwrap();
        let next = parts.next().unwrap();

        let rule_type = if cond.contains('>') {
            Action::GreaterThan
        } else {
            Action::LessThan
        };

        let mut params = cond.split(['>', '<']);

        let field = match params.next().unwrap() {
            "x" => Field::X,
            "m" => Field::M,
            "a" => Field::A,
            "s" => Field::S,
            _ => panic!("noop"),
        };

        let value = params.next().unwrap().parse::<i32>().unwrap();

        return Rule {
            rule_type: rule_type,
            next: next.to_string(),
            value: Some(value),
            field: Some(field),
        };
    }

    fn evaluate(&self, part: &Part) -> Option<String> {
        if self.rule_type == Action::Forward {
            return Some(self.next.clone());
        }

        let value = match self.field.as_ref().unwrap() {
            Field::X => part.x,
            Field::M => part.m,
            Field::A => part.a,
            Field::S => part.s,
        };

        if self.rule_type == Action::GreaterThan {
            if value > self.value.unwrap() {
                return Some(self.next.clone());
            }
            return None;
        }

        if value < self.value.unwrap() {
            return Some(self.next.clone());
        }
        return None;
    }
}

fn part1(rules: HashMap<String, Vec<Rule>>, parts: Vec<Part>) -> i32 {
    let mut sum: i32 = 0;

    for part in &parts {
        let mut current_rule = "in".to_string();

        'outer: loop {
            for rule in rules.get(&current_rule).unwrap() {
                match rule.evaluate(part) {
                    Some(res) => {
                        if res.eq("R") {
                            break 'outer;
                        }

                        if res.eq("A") {
                            sum += part.x + part.m + part.a + part.s;
                            break 'outer;
                        }

                        current_rule = res;
                        break;
                    }
                    None => continue,
                }
            }
        }
    }

    return sum;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Step {
    pipeline: String,
    x: (i32, i32),
    m: (i32, i32),
    a: (i32, i32),
    s: (i32, i32),
}

fn part2(all_rules: HashMap<String, Vec<Rule>>) -> i64 {
    let mut queue: VecDeque<Step> = VecDeque::new();
    queue.push_back(Step {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
        pipeline: "in".to_string(),
    });

    let mut accepted: HashSet<Step> = HashSet::new();

    loop {
        if queue.len() == 0 {
            break;
        }

        let mut head = queue.pop_front().unwrap();

        let pipeline_state = head.pipeline.clone();

        'outer: loop {
            if pipeline_state == "R" {
                break;
            }

            if pipeline_state == "A" {
                accepted.insert(head.clone());
                break;
            }

            for rule in all_rules.get(&pipeline_state).unwrap() {
                if rule.rule_type == Action::Forward {
                    head.pipeline = rule.next.clone();
                    queue.push_back(head);
                    break 'outer;
                }

                let value = rule.value.clone().unwrap();

                let mut next_head = head.clone();

                next_head.pipeline = rule.next.clone();

                match rule.field.clone().unwrap() {
                    Field::X => next_head.x = include_range(value, rule.rule_type.clone(), head.x),
                    Field::M => next_head.m = include_range(value, rule.rule_type.clone(), head.m),
                    Field::A => next_head.a = include_range(value, rule.rule_type.clone(), head.a),
                    Field::S => next_head.s = include_range(value, rule.rule_type.clone(), head.s),
                };

                match rule.field.clone().unwrap() {
                    Field::X => head.x = exclude_range(value, rule.rule_type.clone(), head.x),
                    Field::M => head.m = exclude_range(value, rule.rule_type.clone(), head.m),
                    Field::A => head.a = exclude_range(value, rule.rule_type.clone(), head.a),
                    Field::S => head.s = exclude_range(value, rule.rule_type.clone(), head.s),
                };

                queue.push_back(next_head);
            }
        }
    }

    let mut sum: i64 = 0;
    for step in accepted {
        sum += (step.x.1 - step.x.0 + 1) as i64
            * (step.m.1 - step.m.0 + 1) as i64
            * (step.a.1 - step.a.0 + 1) as i64
            * (step.s.1 - step.s.0 + 1) as i64;
    }

    return sum;
}

fn include_range(value: i32, rule_type: Action, range: (i32, i32)) -> (i32, i32) {
    return match rule_type {
        Action::GreaterThan => (value + 1, range.1),
        Action::LessThan => (range.0, value - 1),
        _ => panic!("noop"),
    };
}

fn exclude_range(value: i32, rule_type: Action, range: (i32, i32)) -> (i32, i32) {
    return match rule_type {
        Action::GreaterThan => (range.0, value),
        Action::LessThan => (value, range.1),
        _ => panic!("noop"),
    };
}
