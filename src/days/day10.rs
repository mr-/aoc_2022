use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::mem::replace;

#[derive(Debug, Clone)]
enum Op {
    N,
    A(i32),
}

fn parse_line(line: &str) -> Option<Op> {
    let re = Regex::new(r"noop").unwrap();
    if re.is_match(line) {
        return Some(Op::N);
    }
    let re = Regex::new(r"addx (-?\d+)").unwrap();
    let cap = re.captures(line)?;
    Some(Op::A(cap[1].parse().ok()?))
}

fn parse_file(input: &str) -> Vec<Op> {
    input.split("\n").filter_map(parse_line).collect()
}

type History = Vec<i32>;

fn evaluate(ops: &Vec<Op>) -> (History, i32) {
    let mut history = Vec::new();
    let mut current_value = 1;

    for op in ops {
        match op {
            Op::N => history.push(current_value),
            Op::A(i) => {
                history.push(current_value);
                history.push(current_value);
                current_value = current_value + i;
            }
        }
    }
    (history, current_value)
}

pub fn solution() {
    let input = "noop
addx 3
addx -5";
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    let input =
        &fs::read_to_string("./input/10.txt").expect("Should have been able to read the file");
    let ops = parse_file(input);
    let history = evaluate(&ops);

    let indices: Vec<i32> = Vec::from([20, 60, 100, 140, 180, 220]);
    let res: i32 = indices
        .into_iter()
        .map(|i| i * history.0[(i - 1) as usize])
        .sum();
    println!("sol1: {:?}", res);

    let h = history.0;

    println!("Solution 2");
    for i in 0..6 {
        let l1 = get_line(&h, i);
        for l in l1 {
            print!("{}", l);
        }
        println!();
    }
}

fn get_line(h: &Vec<i32>, line: usize) -> Vec<&str> {
    let offset = line * 40;
    (0..40)
        .clone()
        .map(|i| {
            if Vec::from([h[i + offset] - 1, h[i + offset], h[i + offset] + 1])
                .contains(&(i as i32))
            {
                "#"
            } else {
                "."
            }
        })
        .collect()
}
