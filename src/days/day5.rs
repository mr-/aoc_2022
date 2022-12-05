use std::fs;
use std::collections::VecDeque;
use std::mem::replace;
use regex::Regex;


#[derive(Debug)]
struct Op {
    amount : usize,
    from : usize,
    to: usize 
}
// move 4 from 5 to 9
fn parse_line2(l : &str) -> Option<Op> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let cap = re.captures(l)?;
    Some(Op {
         amount: cap[1].parse().ok()?,
         from: cap[2].parse().ok()?,
         to: cap[3].parse().ok()?,
     })
}

type Stack = Vec<VecDeque<char>>;

fn apply_op(op: Op, stack : &mut Stack) {
    for _ in 0..op.amount {
        let x = stack[op.from].pop_front().unwrap();
        stack[op.to].push_front(x);
    }
}
fn apply_op2(op: Op, stack : &mut Stack) {
    let tail = stack[op.from].split_off(op.amount);
    let mut old_from = replace(&mut stack[op.from], tail);
    old_from.append(&mut stack[op.to]);
    stack[op.to] = old_from;
}

/*
    [P]                 [Q]     [T]
[F] [N]             [P] [L]     [M]
[H] [T] [H]         [M] [H]     [Z]
[M] [C] [P]     [Q] [R] [C]     [J]
[T] [J] [M] [F] [L] [G] [R]     [Q]
[V] [G] [D] [V] [G] [D] [N] [W] [L]
[L] [Q] [S] [B] [H] [B] [M] [L] [D]
[D] [H] [R] [L] [N] [W] [G] [C] [R]
1   2   3   4   5   6   7   8   9
*/

pub fn solution1() {
    // Should start at 1...
    let mut stack = Vec::from([
        VecDeque::from([' ']),
        VecDeque::from(['F', 'H', 'M', 'T', 'V', 'L', 'D']),
        VecDeque::from(['P', 'N', 'T', 'C', 'J', 'G', 'Q', 'H']),
        VecDeque::from(['H', 'P', 'M', 'D', 'S', 'R']),
        VecDeque::from(['F', 'V', 'B', 'L']),
        VecDeque::from(['Q', 'L', 'G', 'H', 'N']),
        VecDeque::from(['P', 'M', 'R', 'G', 'D', 'B', 'W']),
        VecDeque::from(['Q', 'L', 'H', 'C', 'R', 'N', 'M', 'G' ]),
        VecDeque::from(['W', 'L', 'C']),
        VecDeque::from(['T', 'M', 'Z', 'J', 'Q', 'L', 'D', 'R'])]);
    let contents = fs::read_to_string("./05.txt")
        .expect("Should have been able to read the file");

    let moves = contents.split("\n")
        .filter_map(parse_line2)
        .collect::<Vec<Op>>();

    for op in moves {
        apply_op(op, &mut stack)
    }

    for s in stack.iter() {
        print!("{}", s[0]);
    }

}

pub fn solution2() {
    // Should start at 1...
    let mut stack = Vec::from([
        VecDeque::from([' ']),
        VecDeque::from(['F', 'H', 'M', 'T', 'V', 'L', 'D']),
        VecDeque::from(['P', 'N', 'T', 'C', 'J', 'G', 'Q', 'H']),
        VecDeque::from(['H', 'P', 'M', 'D', 'S', 'R']),
        VecDeque::from(['F', 'V', 'B', 'L']),
        VecDeque::from(['Q', 'L', 'G', 'H', 'N']),
        VecDeque::from(['P', 'M', 'R', 'G', 'D', 'B', 'W']),
        VecDeque::from(['Q', 'L', 'H', 'C', 'R', 'N', 'M', 'G' ]),
        VecDeque::from(['W', 'L', 'C']),
        VecDeque::from(['T', 'M', 'Z', 'J', 'Q', 'L', 'D', 'R'])]);
    let contents = fs::read_to_string("./05.txt")
        .expect("Should have been able to read the file");

    let moves = contents.split("\n")
        .filter_map(parse_line2)
        .collect::<Vec<Op>>();

    for op in moves {
        apply_op2(op, &mut stack)
    }

    for s in stack.iter() {
        print!("{}", s[0]);
    }

}
