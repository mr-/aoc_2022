use std::{collections::HashMap, fs};

use regex::Regex;

#[derive(Debug)]
enum Op {
    N(i64),
    Div(String, String),
    Mul(String, String),
    Sub(String, String),
    Add(String, String),
}

fn parse_literal(input: &str) -> Option<(String, Op)> {
    let re = Regex::new(r"([^:]+): (\d+)").ok()?;
    let cap = re.captures(input)?;

    Some((cap[1].to_string(), Op::N(cap[2].parse().ok()?)))
}

fn parse_op(input: &str) -> Option<(String, Op)> {
    let re = Regex::new(r"([^:]+): (\w+) (.) (\w+)").ok()?;
    let cap = re.captures(input)?;

    let a = cap[2].to_string();
    let b = cap[4].to_string();
    let op = match &cap[3] {
        "/" => Op::Div(a, b),
        "*" => Op::Mul(a, b),
        "+" => Op::Add(a, b),
        "-" => Op::Sub(a, b),
        _ => panic!("uhoh.."),
    };
    let res = (cap[1].to_string(), op);

    Some(res)
}

fn parse_line(input: &str) -> Option<(String, Op)> {
    parse_literal(input).or(parse_op(input))
}

type Instructions = HashMap<String, Op>;

fn evaluate(p: &Instructions, node: &str) -> i64 {
    match p.get(node).unwrap() {
        Op::N(i) => *i,
        Op::Div(a, b) => evaluate(p, a) / evaluate(p, b),
        Op::Mul(a, b) => evaluate(p, a) * evaluate(p, b),
        Op::Add(a, b) => evaluate(p, a) + evaluate(p, b),
        Op::Sub(a, b) => evaluate(p, a) - evaluate(p, b),
    }
}

fn pp(p: &Instructions, node: &str) -> String {
    if node == "humn" {
        return "humn".to_string();
    }
    match p.get(node).unwrap() {
        Op::N(i) => format!("{:?}", i),
        Op::Div(a, b) => format!("({} / {})", pp(p, a), pp(p, b)),
        Op::Mul(a, b) => format!("({} * {})", pp(p, a), pp(p, b)),
        Op::Add(a, b) => format!("({} + {})", pp(p, a), pp(p, b)),
        Op::Sub(a, b) => format!("({} - {})", pp(p, a), pp(p, b)),
    }
}

pub fn solution() {
    let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    let input =
        &fs::read_to_string("./input/21.txt").expect("Should have been able to read the file");

    let mut ins: Instructions = input.lines().into_iter().filter_map(parse_line).collect();
    let res = evaluate(&ins, "root");
    println!("Sol1: {res:?}");
    println!("");

    // root: pgtp + vrvh
    let li = "pgtp";
    let ri = "vrvh";
    let left = evaluate(&ins, li);
    let right = evaluate(&ins, ri);
    println!("left:  {left:?}");
    println!("right: {right:?}");

    println!("");
    ins.insert("humn".to_string(), Op::N(3207994218255));
    let left = evaluate(&ins, li);
    let right = evaluate(&ins, ri);
    println!("left:  {left:?}");
    println!("right: {right:?}");
    let exp = pp(&ins, li);
    println!("{exp}")
}
