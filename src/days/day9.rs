use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::mem::replace;

#[derive(Debug)]
pub enum Dir {
    U,
    D,
    L,
    R,
}
type Cmd = (Dir, i32);

fn parse_line(input: &str) -> Option<Cmd> {
    let re = Regex::new(r"([UDLR]) (\d+)").unwrap();
    let cap = re.captures(input)?;
    Some(match &cap[1] {
        "U" => (Dir::U, cap[2].parse().ok()?),
        "L" => (Dir::L, cap[2].parse().ok()?),
        "R" => (Dir::R, cap[2].parse().ok()?),
        "D" => (Dir::D, cap[2].parse().ok()?),
        _ => return None,
    })
}
fn parse_file(input: &str) -> Vec<Cmd> {
    input
        .split("\n")
        .into_iter()
        .filter_map(parse_line)
        .collect()
}
fn add(p: &Point, q: &Point) -> Point {
    (p.0 + q.0, p.1 + q.1)
}

type Point = (i32, i32);
type Field = HashSet<Point>;

struct State {
    seen: Field,
    rope: Vec<Point>,
}

fn touches(head: &Point, tail: &Point) -> bool {
    let d = ((head.0 - tail.0).abs(), (head.1 - tail.1).abs());
    match d {
        (0, 0) => true,
        (1, 0) => true,
        (0, 1) => true,
        (1, 1) => true,
        _ => false,
    }
}

fn do_step(seen: &mut Field, head: &mut Point, tail: &mut Point, dir: &Dir) {
    let v: Point = match dir {
        Dir::U => (0, 1),
        Dir::D => (0, -1),
        Dir::L => (-1, 0),
        Dir::R => (1, 0),
    };
    replace(head, add(head, &v));
    if touches(&head, &tail) {
        return;
    }

    let d = (head.0 - tail.0, head.1 - tail.1);

    let new_tail = match d {
        (-2, _) => add(head, &(1, 0)),
        (2, _) => add(head, &(-1, 0)),
        (_, 2) => add(head, &(0, -1)),
        (_, -2) => add(head, &(0, 1)),
        _ => panic!("crash and burn"),
    };

    seen.insert(new_tail);
    replace(tail, new_tail);
}

fn step(state: &mut State, cmd: Cmd) {
    let (dir, steps) = cmd;
    if steps == 0 {
        return;
    }

    let mut head = replace(&mut state.rope[0], (0, 0));
    let mut tail = replace(&mut state.rope[1], (0, 0));
    // move head
    // follow tail1
    // follow tail2
    // follow tail3..
    do_step(&mut state.seen, &mut head, &mut tail, &dir);

    replace(&mut state.rope[0], head);
    replace(&mut state.rope[1], tail);
    step(state, (dir, steps - 1))
}

pub fn solution() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let input =
        fs::read_to_string("./input/09.txt").expect("Should have been able to read the file");
    let cmds = parse_file(input.as_str());
    let mut state = State {
        seen: HashSet::<Point>::from([(0, 0)]),
        rope: Vec::from([(0, 0), (0, 0)]),
    };
    for cmd in cmds {
        step(&mut state, cmd);
    }
    println!("{:?}", state.seen.len())
}
