use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

type Point = (i32, i32);

// key = target, val = sources
type Plan = HashMap<Point, Vec<Point>>;
type Board = HashSet<Point>;

enum Direction {
    N,
    W,
    S,
    E,
}

fn parse_board(input: &str) -> HashSet<Point> {
    let mut board = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        for thing in line.chars() {
            match thing {
                '#' => board.insert((x, y)),
                _ => false,
            };
            x = x + 1;
        }
        x = 0;
        y = y + 1;
    }
    return board;
}

struct State {
    start: Direction,
    board: Board,
}

impl State {
    fn new(input: &str) -> Self {
        let board = parse_board(input);
        State {
            start: Direction::N,
            board,
        }
    }

    fn do_step(&mut self) -> bool {
        let directions = get_directions(&self.start);
        let mut plan = HashMap::new();
        for elf in &self.board {
            plan_for(&self.board, &mut plan, &elf, &directions)
        }
        let can_move = plan
            .into_iter()
            .filter(|(k, v)| v.len() == 1)
            .map(|(k, v)| (k, v[0]))
            .collect::<HashMap<Point, Point>>();

        if can_move.len() == 0 {
            return false;
        }

        let to_move = can_move.values();
        let targets = can_move.keys();

        self.start = next_direction(&self.start);
        for x in to_move {
            self.board.remove(x);
        }
        for x in targets {
            self.board.insert(*x);
        }
        return true;
    }

    fn boundary(&self) -> (Point, Point) {
        let maxx = self.board.iter().max_by_key(|x| x.0).unwrap().0;
        let maxy = self.board.iter().max_by_key(|x| x.1).unwrap().1;
        let minx = self.board.iter().min_by_key(|x| x.0).unwrap().0;
        let miny = self.board.iter().min_by_key(|x| x.1).unwrap().1;

        ((minx, miny), (maxx, maxy))
    }

    fn pp(&self) {
        let ((minx, miny), (maxx, maxy)) = self.boundary();
        for y in miny..=maxy {
            for x in minx..=maxx {
                let c = &(x, y);
                if self.board.contains(c) {
                    print!("#")
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn score(&self) -> i32 {
        let ((minx, miny), (maxx, maxy)) = self.boundary();
        let area = ((maxx - minx).abs() + 1) * ((maxy - miny).abs() + 1);

        area - (self.board.len() as i32)
    }
}

fn check_direction(board: &Board, elf: &Point, direction: &Direction) -> Option<Point> {
    let candidates = match direction {
        Direction::N => vec![
            (elf.0 - 1, elf.1 - 1),
            (elf.0, elf.1 - 1),
            (elf.0 + 1, elf.1 - 1),
        ],
        Direction::W => vec![
            (elf.0 - 1, elf.1 + 1),
            (elf.0 - 1, elf.1),
            (elf.0 - 1, elf.1 - 1),
        ],
        Direction::S => vec![
            (elf.0 - 1, elf.1 + 1),
            (elf.0, elf.1 + 1),
            (elf.0 + 1, elf.1 + 1),
        ],
        Direction::E => vec![
            (elf.0 + 1, elf.1 - 1),
            (elf.0 + 1, elf.1),
            (elf.0 + 1, elf.1 + 1),
        ],
    };
    let neighborhood = vec![
        (elf.0 - 1, elf.1 - 1),
        (elf.0, elf.1 - 1),
        (elf.0 + 1, elf.1 - 1),
        (elf.0 - 1, elf.1 + 1),
        (elf.0 - 1, elf.1),
        (elf.0, elf.1 + 1),
        (elf.0 + 1, elf.1 + 1),
        (elf.0 + 1, elf.1),
    ];

    if neighborhood.iter().all(|c| !board.contains(c)) {
        return None;
    }

    if candidates.iter().any(|c| board.contains(c)) {
        return None;
    } else {
        return Some(candidates[1]);
    }
}

fn get_directions(start: &Direction) -> Vec<Direction> {
    match start {
        Direction::N => vec![Direction::N, Direction::S, Direction::W, Direction::E],
        Direction::S => vec![Direction::S, Direction::W, Direction::E, Direction::N],
        Direction::W => vec![Direction::W, Direction::E, Direction::N, Direction::S],
        Direction::E => vec![Direction::E, Direction::N, Direction::S, Direction::W],
    }
}
fn next_direction(start: &Direction) -> Direction {
    match start {
        Direction::N => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::E,
        Direction::E => Direction::N,
    }
}

fn plan_for(board: &Board, plan: &mut Plan, elf: &Point, directions: &Vec<Direction>) {
    for direction in directions {
        if let Some(target) = check_direction(board, elf, direction) {
            plan.entry(target)
                .and_modify(|v| v.push(elf.clone()))
                .or_insert(vec![elf.clone()]);
            return;
        }
    }
}

pub fn solution() {
    let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    let input =
        &fs::read_to_string("./input/23.txt").expect("Should have been able to read the file");

    let mut board = State::new(input);
    for i in 1..=10 {
        let moved = board.do_step();
    }
    let score = board.score();
    println!("Solution 1: {score:?}");

    let mut board = State::new(input);
    for i in 1..=10000 {
        let moved = board.do_step();
        if !moved {
            println!("Solution 2: Step {i} with score {:?}", board.score());
            return;
        }
    }
}
