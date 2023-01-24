use std::{collections::HashMap, fs, mem};

use num_bigint::BigInt;
use num_rational::BigRational;
use num_rational::Ratio;
use regex::Regex;

#[derive(Debug, Clone)]
enum Op {
    N(BigRational),
    Var(BigRational),
    Div(String, String),
    Mul(String, String),
    Sub(String, String),
    Add(String, String),
}

fn parse_literal(input: &str) -> Option<(String, Op)> {
    let re = Regex::new(r"([^:]+): (\d+)").ok()?;
    let cap = re.captures(input)?;
    let id = cap[1].to_string();
    let val = cap[2].parse().ok()?;
    if id == "humn" {
        return Some((id, Op::Var(val)));
    }

    Some((id, Op::N(val)))
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

fn evaluate(p: &Instructions, node: &str) -> BigRational {
    match p.get(node).unwrap() {
        Op::N(i) => i.clone(),
        Op::Div(a, b) => evaluate(p, a) / evaluate(p, b),
        Op::Mul(a, b) => evaluate(p, a) * evaluate(p, b),
        Op::Add(a, b) => evaluate(p, a) + evaluate(p, b),
        Op::Sub(a, b) => evaluate(p, a) - evaluate(p, b),
        Op::Var(i) => i.clone(),
    }
}

fn pp(p: &Instructions, node: &str) -> String {
    match p.get(node).unwrap() {
        Op::N(i) => format!("{:?}", i),
        Op::Div(a, b) => format!("({} / {})", pp(p, a), pp(p, b)),
        Op::Mul(a, b) => format!("({} * {})", pp(p, a), pp(p, b)),
        Op::Add(a, b) => format!("({} + {})", pp(p, a), pp(p, b)),
        Op::Sub(a, b) => format!("({} - {})", pp(p, a), pp(p, b)),
        Op::Var(i) => format!("[Var({:?}) = {:?}]", node, i),
    }
}

fn simplify_mut(p: &mut Instructions, node: &str) {
    match p.clone().get(node).unwrap() {
        Op::Div(a, b) => {
            simplify_mut(p, a);
            simplify_mut(p, b);
            if let (Some(Op::N(a)), Some(Op::N(b))) = (p.get(a), p.get(b)) {
                p.insert(node.to_string(), Op::N(a / b));
            }
        }

        Op::Mul(a, b) => {
            simplify_mut(p, a);
            simplify_mut(p, b);
            if let (Some(Op::N(a)), Some(Op::N(b))) = (p.get(a), p.get(b)) {
                p.insert(node.to_string(), Op::N(a * b));
            }
        }
        Op::Add(a, b) => {
            simplify_mut(p, a);
            simplify_mut(p, b);
            if let (Some(Op::N(a)), Some(Op::N(b))) = (p.get(a), p.get(b)) {
                p.insert(node.to_string(), Op::N(a + b));
            }
        }
        Op::Sub(a, b) => {
            simplify_mut(p, a);
            simplify_mut(p, b);
            if let (Some(Op::N(a)), Some(Op::N(b))) = (p.get(a), p.get(b)) {
                p.insert(node.to_string(), Op::N(a - b));
            }
        }
        _ => {}
    };
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
    ins.insert("humn".to_string(), Op::Var(n(3207994218255)));
    let left = evaluate(&ins, li);
    let right = evaluate(&ins, ri);
    println!("left:  {left:?}");
    println!("right: {right:?}");

    println!("So right is constant and left depends on the variable");

    let exp = pp(&simplify(&mut ins.clone(), li), li);
    println!("{exp}");

    println!("It's clear now that we should be able to simplify to something like m * Var + c");
    println!("Setting Var to 0 and evaluating will get us c:");
    let res = ins.insert("humn".to_string(), Op::N(n(0)));
    let c = evaluate(&ins, li);
    println!("c = {:?}", c);

    let res = ins.insert("humn".to_string(), Op::N(n(1)));
    let m = evaluate(&ins, li) - c.clone();
    println!("m = {:?}", m);

    println!("Now `right = m * Var + c``");
    let var = (right - c) / m;
    println!("Sol2: var = {:?}", var);
}

fn simplify(ins: &Instructions, node: &str) -> Instructions {
    let mut ins = ins.clone();
    simplify_mut(&mut ins, node);
    ins
}
fn n(num: i64) -> BigRational {
    BigRational::from(BigInt::from(num))
}
