use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem;
use std::mem::replace;
use std::mem::take;

use itertools::Itertools;
use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Default)]
enum Op {
    Plus(u64),
    Mult(u64),
    #[default]
    Sq,
}

#[derive(Debug, Clone, Default)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    op: Op,
    test: (u64, u64, u64),
}
fn parse_monkey(input: &str) -> Option<Monkey> {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let re = Regex::new(r"Monkey (\d+)").unwrap();
    let cap = re.captures(lines[0])?;
    let id: u64 = cap[1].parse().ok()?;

    let re = Regex::new(r"Starting items: ([\d, ]+)").unwrap();
    let items = re.captures(lines[1])?[1]
        .split(", ")
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<VecDeque<u64>>();

    let re = Regex::new(r"Operation: new = old ([+*]) (.+)").unwrap();
    let cap = re.captures(lines[2])?;
    let op = match (&cap[1], &cap[2]) {
        ("+", d) => Op::Plus(d.parse::<u64>().ok()?),
        ("*", "old") => Op::Sq,
        ("*", d) => Op::Mult(d.parse::<u64>().ok()?),
        (_, _) => panic!("at the disco"),
    };

    let re = Regex::new(r"Test: divisible by ([\d]+)").unwrap();
    let cap = re.captures(lines[3])?;
    let test_div: u64 = cap[1].parse().ok()?;

    let re = Regex::new(r"If true: throw to monkey ([\d+])").unwrap();
    let cap = re.captures(lines[4])?;
    let test_true: u64 = cap[1].parse().ok()?;

    let re = Regex::new(r"If false: throw to monkey ([\d+])").unwrap();
    let cap = re.captures(lines[5])?;
    let test_false: u64 = cap[1].parse().ok()?;

    return Some(Monkey {
        id,
        items,
        op,
        test: (test_div, test_true, test_false),
    });
}
fn parse_file(input: &str) -> Vec<Monkey> {
    let monkeys = input.split("\n\n");
    monkeys.filter_map(parse_monkey).collect()
}

fn calculate_level(monkey: &Monkey, item: u64) -> u64 {
    let new_worry = match monkey.op {
        Op::Sq => item * item,
        Op::Mult(i) => item * i,
        Op::Plus(i) => item + i,
    };
    new_worry / 3
}
fn calculate_level_2(monkey: &Monkey, item: u64, modulo: u64) -> u64 {
    let new_worry = match monkey.op {
        Op::Sq => item * item,
        Op::Mult(i) => item * i,
        Op::Plus(i) => item + i,
    };
    new_worry % modulo
}

fn calculate_target(monkey: &Monkey, level: u64) -> u64 {
    if level % monkey.test.0 == 0 {
        return monkey.test.1;
    } else {
        return monkey.test.2;
    }
}

pub fn solution() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    let input =
        &fs::read_to_string("./input/11.txt").expect("Should have been able to read the file");
    sol1(input);
    sol2(input);
}

fn sol2(input: &str) {
    let mut monkeys = parse_file(input);
    let mut seen: HashMap<usize, u64> = HashMap::new();

    let modulo: u64 = monkeys.clone().into_iter().map(|x| x.test.0).product();
    for _ in 0..10000 {
        play_round(&mut seen, &mut monkeys, Some(modulo));
    }
    let max = seen
        .into_iter()
        .map(|(_, c)| c)
        .sorted()
        .rev()
        .take(2)
        .collect::<Vec<u64>>();
    println!("sol2 {:?}", max[0] * max[1]);
}

fn sol1(input: &str) {
    let mut monkeys = parse_file(input);
    let mut seen: HashMap<usize, u64> = HashMap::new();

    for _ in 0..20 {
        play_round(&mut seen, &mut monkeys, None);
    }
    let max = seen
        .into_iter()
        .map(|(_, c)| c)
        .sorted()
        .rev()
        .take(2)
        .collect::<Vec<u64>>();
    println!("sol1 {:?}", max[0] * max[1]);
}

fn play_round(seen: &mut HashMap<usize, u64>, monkeys: &mut Vec<Monkey>, modulo: Option<u64>) {
    for source_id in 0..monkeys.len() {
        let mut monkey = take(&mut monkeys[source_id]);
        while monkey.items.len() > 0 {
            let item = monkey.items.pop_front().unwrap();
            let level = if modulo.is_none() {
                calculate_level(&monkey, item)
            } else {
                calculate_level_2(&monkey, item, modulo.unwrap())
            };
            let target = calculate_target(&monkey, level);
            let target_id = monkeys
                .clone()
                .into_iter()
                .position(|x| x.id == target)
                .unwrap();
            let mut target = mem::take(&mut monkeys[target_id as usize]);
            target.items.push_back(level);
            mem::replace(&mut monkeys[target_id as usize], target);

            if seen.contains_key(&source_id) {
                seen.insert(source_id, seen[&source_id] + 1);
            } else {
                seen.insert(source_id, 1);
            }
        }

        replace(&mut monkeys[source_id], monkey);
    }
}
