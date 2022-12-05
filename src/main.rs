use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::mem::replace;
use regex::Regex;

pub fn solution1() {
    let contents = fs::read_to_string("./01.txt")
        .expect("Should have been able to read the file");
    let s :Vec<Vec<&str>>= contents.split("\n\n").map(|x| x.split("\n").collect()).collect();
    let bags : Vec<Vec<i32>> = s.into_iter()
        .map(|x| x.into_iter()
             .filter_map(|y| y.parse::<i32>().ok()).collect::<Vec<i32>>()
        )
        .collect();
    let mut sums = bags.into_iter()
        .map(|x| x.into_iter().sum::<i32>())
        .collect::<Vec<i32>>();
    sums.sort();
    sums.reverse();
    let sol1 = sums[0];
    let sol2 : i32 = sums.into_iter().take(3).sum();

    println!("sol 1 {:?}", sol1);
    println!("sol 2 {:?}", sol2);
}

pub fn solution2() {

    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    let points = HashMap::from([
        (("A", "X"), 1+3),
        (("A", "Y"), 2+6),
        (("A", "Z"), 3+0),

        (("B", "X"), 1+0),
        (("B", "Y"), 2+3),
        (("B", "Z"), 3+6),

        (("C", "X"), 1+6),
        (("C", "Y"), 2+0),
        (("C", "Z"), 3+3),
    ]);

    let contents = fs::read_to_string("./02.txt")
        .expect("Should have been able to read the file");

    let s :Vec<Vec<&str>>= contents
        .split("\n")
        .map(|x| x.split(" ").collect())
        .filter(|x : &Vec<&str>| x.len() > 1)
        .collect();
    let sol1 : i32 = s.clone().into_iter().map(|x| points[&(x[0],x[1])]).sum();
    println!("sol1 {}", sol1);

    // X Loss, Y Draw, Z Win
    // A for Rock, B for Paper, and C for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    let points2 = HashMap::from([
        (("A", "X"), 0 + 3),
        (("A", "Y"), 3 + 1),
        (("A", "Z"), 6 + 2),

        (("B", "X"), 0 + 1),
        (("B", "Y"), 3 + 2),
        (("B", "Z"), 6 + 3),

        (("C", "X"), 0 + 2),
        (("C", "Y"), 3 + 3),
        (("C", "Z"), 6 + 1),
    ]);
    let sol2 : i32 = s.into_iter().map(|x| points2[&(x[0],x[1])]).sum();
    println!("sol2 {}", sol2);
}

fn find_common(s : &str) -> i32 {
    let middle = s.len()/2;
    let fst : HashSet<char> = s[..middle].chars().collect();
    let snd : HashSet<char> = s[middle..].chars().collect();
    let u = fst.intersection(&snd);
    let common = u.clone().nth(0).unwrap();
    get_value(common.clone())
}

fn get_value(s:char) -> i32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let ascii_lowercase = ('a'..='z').collect::<Vec<char>>();
    let ascii_uppercase = ('A'..='Z').collect::<Vec<char>>();
    let chars : Vec<char> = ascii_lowercase.into_iter().chain(ascii_uppercase.into_iter()).collect();
    let len : i32 = 52;
    let vals : HashMap<char, i32>= chars.into_iter()
        .zip(1..=len)
        .map(|(a,b)| (a,b))
        .collect();

    vals[&s]
}

fn get_badges(bags : Vec<&str>) -> i32 {
    if bags.len() < 3 {
        return 0;
    }

    let g1 : HashSet<char> = bags[0].chars().collect();
    let g2 : HashSet<char> = bags[1].chars().collect();
    let g3 : HashSet<char> = bags[2].chars().collect();
    let i1 = g1.intersection(&g2).cloned().collect::<HashSet<char>>();
    let badges = i1.intersection(&g3).collect::<HashSet<&char>>();
    let badge = badges.into_iter().nth(0).unwrap();
    let val = get_value(badge.clone());

    let rest  :Vec<&str> = bags.into_iter().skip(3).collect();

    return val + get_badges(rest);
}

fn solution3() {
    let contents = fs::read_to_string("./03.txt")
        .expect("Should have been able to read the file");
    let sol1 : i32 = contents.split("\n")
        .filter(|x| x.len() > 1)
        .map(|x| find_common(x))
        .sum();
    println!("sol1 {}", sol1);

    let bags: Vec<&str> = contents.split("\n")
        .filter(|x| x.len() > 1)
        .collect();
    println!("sol2 {}", get_badges(bags));
}
type SectionPair = ((i32, i32), (i32, i32));


fn parse_line(s : &str) -> Option<SectionPair> {
    let (p1, p2)   = match s.split(",").collect::<Vec<&str>>()[..] { [a,b] => Some((a,b)), _ => None }?;
    let (p11, p12) = match p1.split("-").collect::<Vec<&str>>()[..] { [a,b] => Some((a,b)), _ => None }?;
    let (p21, p22) = match p2.split("-").collect::<Vec<&str>>()[..] { [a,b] => Some((a,b)), _ => None }?;

    return Some(((p11.parse().ok()?, p12.parse().ok()?), (p21.parse().ok()?, p22.parse().ok()?)));
}

fn is_contained(p : SectionPair) -> bool {
    let ((a, b), (c,d)) = p;

    return a <= c && b >= d || c <= a && d >= b;
}

fn is_overlap(p : SectionPair) -> bool {
    let ((a, b), (c,d)) = p;

    let no_overlap = a > d || b < c;
    return !no_overlap
}

fn parse_file() -> Vec<SectionPair> {
    let contents = fs::read_to_string("./04.txt")
        .expect("Should have been able to read the file");
    let intervals = contents.split("\n")
        .filter_map(parse_line)
        .collect::<Vec<SectionPair>>();

    return intervals;
}

fn solution4() {
    let assignments = parse_file();

    let count = assignments.clone().into_iter().filter(|x| is_contained(x.clone())).count();
    println!("Sol1 {}", count);

    let count = &assignments.into_iter().filter(|x| is_overlap(x.clone())).count();
    println!("Sol2 {}", count);
}

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

fn solution51() {
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

fn solution52() {
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
fn main() {
    solution52()
}
