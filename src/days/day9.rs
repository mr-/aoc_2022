use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::mem::replace;

#[derive(Debug, Clone)]
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

fn do_move(head: &mut Point, dir: &Dir) {
    let v: Point = match dir {
        Dir::U => (0, 1),
        Dir::D => (0, -1),
        Dir::L => (-1, 0),
        Dir::R => (1, 0),
    };
    let new_head: Point = add(head, &v);
    *head = new_head;
}

fn do_follow(head: &Point, tail: &mut Point) -> bool {
    if touches(&head, &tail) {
        return false;
    }

    let d = (head.0 - tail.0, head.1 - tail.1);

    let new_tail = match d {
        (-2, -2) => add(head, &(1, 1)),
        (2, -2) => add(head, &(-1, 1)),
        (-2, 2) => add(head, &(1, -1)),
        (2, 2) => add(head, &(-1, -1)),
        (-2, _) => add(head, &(1, 0)),
        (2, _) => add(head, &(-1, 0)),
        (_, 2) => add(head, &(0, -1)),
        (_, -2) => add(head, &(0, 1)),
        _ => panic!("cra and burn"),
    };

    *tail = new_tail;
    return true;
}

fn step(state: &mut State, cmd: Cmd) {
    let (dir, steps) = cmd;
    if steps == 0 {
        return;
    }

    let mut head = replace(&mut state.rope[0], (0, 0));
    do_move(&mut head, &dir);
    state.rope[0] = head;
    for i in 0..=state.rope.len() - 2 {
        let mut tail = replace(&mut state.rope[i + 1], (0, 0));
        do_follow(&state.rope[i], &mut tail);
        state.rope[i + 1] = tail;
    }

    state.seen.insert(state.rope[state.rope.len() - 1]);

    step(state, (dir, steps - 1))
}

fn pp_seen(seen: &Field, rope: &Vec<Point>) {
    let (s1, s2): (Vec<i32>, Vec<i32>) = seen
        .clone()
        .into_iter()
        .chain(rope.clone().into_iter())
        .unzip();

    let (min1, max1) = (s1.iter().min().unwrap(), s1.iter().max().unwrap());
    let (min2, max2) = (s2.iter().min().unwrap(), s2.iter().max().unwrap());
    let rope_points = rope
        .into_iter()
        .zip(0..rope.len())
        .collect::<HashMap<&Point, usize>>();

    for j in (*min2..=*max2).rev() {
        for i in *min1..=*max1 {
            if rope_points.contains_key(&(i, j)) {
                print!("{:?}", rope_points[&(i, j)]);
                continue;
            }
            if seen.contains(&(i, j)) {
                print!("#")
            } else {
                print!("*")
            }
        }
        println!();
    }
    println!();
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
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    let input =
        &fs::read_to_string("./input/09.txt").expect("Should have been able to read the file");
    let cmds = parse_file(input);

    let mut state = State {
        seen: HashSet::<Point>::from([(0, 0)]),
        rope: Vec::from([(0, 0), (0, 0)]),
    };
    for cmd in cmds.clone() {
        step(&mut state, cmd);
    }
    println!("sol1: {:?}", state.seen.len());

    let mut state = State {
        seen: HashSet::<Point>::from([(0, 0)]),
        rope: Vec::from([
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ]),
    };
    for cmd in cmds {
        step(&mut state, cmd);
    }
    println!("sol2: {:?}", state.seen.len());
}
