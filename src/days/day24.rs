// Idea: This could be a tree for which the nodes are (minute, coordinate)
// The successors are: (minute + 1, c) for c in neighbors if c is free in minute + 1

use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Blizzard(Point),
}

type Point = (i32, i32);

type Board = HashMap<Point, Vec<Tile>>;

fn pp(board: &Board) {
    let to_str = |t: &Tile| match *t {
        Tile::Wall => "#",
        Tile::Blizzard((-1, 0)) => "<",
        Tile::Blizzard((1, 0)) => ">",
        Tile::Blizzard((0, 1)) => "v",
        Tile::Blizzard((0, -1)) => "^",
        _ => panic!("wtf"),
    };
    let maxx = board.keys().max_by_key(|x| x.0).unwrap().0;
    let maxy = board.keys().max_by_key(|x| x.1).unwrap().1;
    for y in 0..=maxy {
        for x in 0..=maxx {
            let c = &(x, y);
            if let Some(ts) = board.get(c) {
                if ts.len() > 1 {
                    print!("{}", ts.len());
                } else {
                    print!("{}", to_str(&ts[0]));
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn parse_board(input: &str) -> Board {
    let mut board = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        for thing in line.chars() {
            match thing {
                '#' => board
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Tile::Wall),
                '>' => board
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Tile::Blizzard((1, 0))),
                '<' => board
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Tile::Blizzard((-1, 0))),

                '^' => board
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Tile::Blizzard((0, -1))),
                'v' => board
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Tile::Blizzard((0, 1))),
                _ => {}
            };
            x = x + 1;
        }
        x = 0;
        y = y + 1;
    }
    return board;
}

fn maybe_wrap(board: &Board, (x, y): Point) -> Point {
    let maxx = board.keys().max_by_key(|x| x.0).unwrap().0;
    let maxy = board.keys().max_by_key(|x| x.1).unwrap().1;
    match (x, y) {
        (0, _) => (maxx - 1, y),
        (_, 0) => (x, maxy - 1),
        (x, _) if x == maxx => (1, y),
        (_, y) if y == maxy => (x, 1),
        _ => (x, y),
    }
}

fn step(board: &Board) -> Board {
    let mut res = HashMap::new();
    for (k, v) in board.iter() {
        for t in v {
            let coord = match t {
                Tile::Blizzard((a, b)) => maybe_wrap(board, (k.0 + a, k.1 + b)),
                x => *k,
            };
            res.entry(coord).or_insert_with(Vec::new).push(t.clone());
        }
    }

    res
}

fn steps(cache: &mut HashMap<i32, Board>, board: &Board, n: i32) -> Board {
    if let Some(b) = cache.get(&n) {
        return b.clone();
    }

    let res = if n == 0 {
        board.clone()
    } else {
        step(&steps(cache, board, n - 1))
    };
    cache.insert(n, res.clone());

    res
}

fn successors(
    cache: &mut HashMap<i32, Board>,
    board: &Board,
    (maxx, maxy): Point,
    curr_step: i32,
    curr_pos: Point,
    e: Point,
) -> Vec<((i32, Point), i32)> {
    if curr_step > 1000 {
        return Vec::new();
    }
    let future_board = steps(cache, board, curr_step + 1);
    let end = (maxx - 1, maxy);

    let candidates = vec![(0, 1), (1, 0), (0, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|(x, y)| (curr_pos.0 + x, curr_pos.1 + y))
        .filter(|p| !future_board.contains_key(&p))
        .filter(|(x, y)| (*x > 0 && *x < maxx && *y >= 0 && *y < maxy) || (*x, *y) == end)
        .collect::<Vec<Point>>();

    if candidates.contains(&e) {
        return vec![((curr_step + 1, e), 1)];
    }

    candidates
        .iter()
        .map(|p| ((curr_step + 1, *p), 1))
        .collect()
}

fn dist(e: Point, p: Point) -> i32 {
    (e.1 - p.1).abs() + (e.0 - p.0).abs()
}

fn solve(
    cache: &mut HashMap<i32, Board>,
    board: &Board,
    (maxx, maxy): Point,
    s: (i32, Point),
    e: Point,
) -> i32 {
    if let Some(res) = astar(
        &s,
        |&n| successors(cache, &board, (maxx, maxy), n.0, n.1, (maxx, maxy)),
        |&(v, p)| dist(e, p),
        |&(v, p)| {
            let res = p == e;
            return res;
        },
    ) {
        return res.0.len() as i32 - 1;
    } else {
        panic!("Well.. nothing, I guess");
    }
}

pub fn solution() {
    let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    let input =
        &fs::read_to_string("./input/24.txt").expect("Should have been able to read the file");
    let board = parse_board(input);

    let maxx = board.keys().max_by_key(|x| x.0).unwrap().0;
    let maxy = board.keys().max_by_key(|x| x.1).unwrap().1;

    let mut cache = HashMap::new();

    let s1 = solve(
        &mut cache,
        &board,
        (maxx, maxy),
        (0, (1, 0)),
        (maxx - 1, maxy),
    );
    println!("Part 1: Solution {s1:?}");

    let s2 = solve(
        &mut cache,
        &board,
        (maxx, maxy),
        (s1, (maxx - 1, maxy)),
        (1, 0),
    );
    println!("Intermediate Solution 2 {s2:?}");

    let s3 = solve(
        &mut cache,
        &board,
        (maxx, maxy),
        (s1 + s2, (1, 0)),
        (maxx - 1, maxy),
    );
    println!("Intermediate Solution 3 {s3:?}");
    println!("Part 2: {}", s1 + s2 + s3)
}
