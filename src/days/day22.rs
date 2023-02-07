use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Part {
    One,
    Two,
}

#[derive(Debug, Clone)]
enum Tile {
    Free,
    Wall,
}
type Point = (i32, i32);
type Board = HashMap<Point, Tile>;

fn parse_board(input: &str) -> Board {
    let mut board = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        for thing in line.chars() {
            match thing {
                '.' => board.insert((x, y), Tile::Free),
                '#' => board.insert((x, y), Tile::Wall),
                _ => None,
            };
            x = x + 1;
        }
        x = 0;
        y = y + 1;
    }
    return board;
}

#[derive(Debug)]
enum Op {
    Rot(i32),
    Step(i32),
}
fn parse_instructions(input: &str) -> Option<Vec<Op>> {
    let re = Regex::new(r"(\d+)|([LR])").ok()?;
    let cap = re.find_iter(input);

    let mut res = Vec::new();
    for c in cap {
        match c.as_str() {
            "L" => res.push(Op::Rot(-1)),
            "R" => res.push(Op::Rot(1)),
            d => res.push(Op::Step(d.parse().ok()?)),
        }
    }
    Some(res)
}

#[derive(Debug)]
struct State {
    part: Part,
    board: Board,
    position: Point,
    orientation: i32,
    path: HashMap<Point, i32>,
}

fn glue(
    borders: &mut HashMap<Point, (Point, i32)>,
    outer: &Vec<Point>,
    inner: &Vec<Point>,
    orientation: i32,
) {
    let new = outer
        .into_iter()
        .cloned()
        .zip(inner.into_iter().cloned().map(|x| (x, orientation)))
        .collect::<HashMap<Point, (Point, i32)>>();

    borders.extend(new);
}

impl State {
    fn new(board: &Board, part: Part) -> Self {
        let my = board.keys().min_by_key(|(_, y)| y).unwrap().1;
        let m = board
            .keys()
            .filter(|(_, y)| *y == my)
            .min_by_key(|(x, _)| x)
            .unwrap();
        State {
            part,
            board: board.clone(),
            position: m.clone(),
            path: HashMap::new(),
            orientation: 0,
        }
    }

    // Well.. this required some drawings and a cut out :P
    // I think, by embedding this in 3d and actually bending, one could get this automatically.. but meh.

    fn neighbor_for_cube(&mut self, off_pos: &Point) -> (Point, i32) {
        let mut border = HashMap::new();
        let a1 = (100..150).map(|a| (a, 50));
        let a2 = (50..100).map(|a| (99, a));
        glue(&mut border, &a1.collect(), &a2.collect(), 2);

        let a1 = (100..150).map(|a| (a, 49));
        let a2 = (50..100).map(|a| (100, a));
        glue(&mut border, &a2.collect(), &a1.collect(), 3);

        let b1 = (0..50).rev().map(|a| (150, a));
        let b2 = (100..150).map(|a| (99, a));
        glue(&mut border, &b1.collect(), &b2.collect(), 2);

        let b1 = (0..50).rev().map(|a| (149, a));
        let b2 = (100..150).map(|a| (100, a));
        glue(&mut border, &b2.collect(), &b1.collect(), 2);

        let c1 = (50..100).map(|a| (a, 150));
        let c2 = (150..200).map(|a| (49, a));
        glue(&mut border, &c1.collect(), &c2.collect(), 2);

        let c1 = (50..100).map(|a| (a, 149));
        let c2 = (150..200).map(|a| (50, a));
        glue(&mut border, &c2.collect(), &c1.collect(), 3);

        let d1 = (50..100).map(|a| (49, a));
        let d2 = (0..50).map(|a| (a, 100));
        glue(&mut border, &d1.collect(), &d2.collect(), 1);

        let d1 = (50..100).map(|a| (50, a));
        let d2 = (0..50).map(|a| (a, 99));
        glue(&mut border, &d2.collect(), &d1.collect(), 0);

        let e1 = (100..150).map(|a| (-1, a));
        let e2 = (0..50).rev().map(|a| (50, a));
        glue(&mut border, &e1.collect(), &e2.collect(), 0);

        let e1 = (100..150).map(|a| (0, a));
        let e2 = (0..50).rev().map(|a| (49, a));
        glue(&mut border, &e2.collect(), &e1.collect(), 0);

        let f1 = (100..150).rev().map(|a| (a, -1));
        let f2 = (0..50).rev().map(|a| (a, 199));
        glue(&mut border, &f1.collect(), &f2.collect(), self.orientation);

        let f1 = (100..150).rev().map(|a| (a, 0));
        let f2 = (0..50).rev().map(|a| (a, 200));
        glue(&mut border, &f2.collect(), &f1.collect(), self.orientation);

        let g1 = (150..200).rev().map(|a| (-1, a));
        let g2 = (50..100).rev().map(|a| (a, 0));
        glue(&mut border, &g1.collect(), &g2.collect(), 1);

        let g1 = (150..200).rev().map(|a| (0, a));
        let g2 = (50..100).rev().map(|a| (a, -1));
        glue(&mut border, &g2.collect(), &g1.collect(), 0);

        let p = border.get(off_pos);

        p.unwrap().clone()
    }

    fn neighbor_for_torus(&self, off_pos: &Point) -> (Point, i32) {
        let new_pos = match self.orientation {
            0 => *self
                .board
                .keys()
                .filter(|x| x.1 == self.position.1)
                .min_by_key(|x| x.0)
                .unwrap(),
            2 => *self
                .board
                .keys()
                .filter(|x| x.1 == self.position.1)
                .max_by_key(|x| x.0)
                .unwrap(),
            1 => *self
                .board
                .keys()
                .filter(|x| x.0 == self.position.0)
                .min_by_key(|x| x.1)
                .unwrap(),
            3 => *self
                .board
                .keys()
                .filter(|x| x.0 == self.position.0)
                .max_by_key(|x| x.1)
                .unwrap(),
            _ => panic!("wtf2"),
        };
        (new_pos, self.orientation)
    }

    fn step(&mut self) -> bool {
        let mut new_pos = match self.orientation {
            0 => (self.position.0 + 1, self.position.1 + 0),
            1 => (self.position.0 + 0, self.position.1 + 1),
            2 => (self.position.0 - 1, self.position.1 + 0),
            3 => (self.position.0 + 0, self.position.1 - 1),
            _ => panic!("wtf.."),
        };
        let mut new_orientation = self.orientation;

        if !self.board.contains_key(&new_pos) {
            let foo = if self.part == Part::One {
                self.neighbor_for_torus(&new_pos)
            } else {
                self.neighbor_for_cube(&new_pos)
            };
            new_pos = foo.0;
            new_orientation = foo.1;
        }

        if let Some(Tile::Free) = self.board.get(&new_pos) {
            self.position = new_pos;
            self.orientation = new_orientation;
            // println!("{new_pos:?} {:?}", self.orientation);
            self.path.insert(new_pos, self.orientation);
            // println!("  {:?}", new_pos);
            return true;
        }

        if let Some(Tile::Wall) = self.board.get(&new_pos) {
            return false;
        }

        false
    }

    fn pp(&self) {
        let mx = self.board.keys().max_by_key(|x| x.0).unwrap().0;
        let my = self.board.keys().max_by_key(|x| x.1).unwrap().1;
        let from_orientation = |o| match o {
            0 => ">",
            1 => "v",
            2 => "<",
            3 => "^",
            _ => panic!("wtf3"),
        };

        for y in 0..=my {
            for x in 0..=mx {
                let c = &(x, y);
                if self.position == *c {
                    print!("{}", from_orientation(self.orientation));
                } else if let Some(o) = self.path.get(c) {
                    print!("{}", from_orientation(*o));
                } else if let Some(Tile::Wall) = self.board.get(c) {
                    print!("#");
                } else if let Some(Tile::Free) = self.board.get(c) {
                    print!(".")
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
    fn score(&self) -> i32 {
        println!(
            "1000 * {:?} + 4 * {:?} + {:?}",
            self.position.1 + 1,
            self.position.0 + 1,
            self.orientation
        );
        1000 * (self.position.1 + 1) + 4 * (self.position.0 + 1) + self.orientation
    }
}

fn parse_input(input: &str, part: Part) -> Option<(State, Vec<Op>)> {
    let [board, ops] = input.split("\n\n").collect::<Vec<&str>>()[..] else {return None;};

    let board = parse_board(board);
    let ops = parse_instructions(ops)?;

    let state = State::new(&board, part);

    Some((state, ops))
}

fn do_steps(state: &mut State, steps: i32) {
    if steps <= 0 {
        return;
    }
    let did_step = state.step();
    if did_step {
        do_steps(state, steps - 1)
    }
}

fn do_thing(state: &mut State, op: &Op) {
    match op {
        Op::Rot(i) => state.orientation = (state.orientation + i).rem_euclid(4),
        Op::Step(i) => do_steps(state, *i),
    }
}

fn dos(input: &str, part: Part) -> State {
    let foo = parse_input(input, part).unwrap();
    let mut state = foo.0;
    let ops = foo.1;
    for thing in ops {
        do_thing(&mut state, &thing);
    }
    state
}

pub fn solution() {
    let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    let input =
        &fs::read_to_string("./input/22.txt").expect("Should have been able to read the file");

    let state = dos(input, Part::One);
    let score = state.score();
    println!("Solution 1: {score:?}");

    let state = dos(input, Part::Two);
    let score = state.score();
    println!("Solution 2: {score:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wraps_up() {
        let input = " ...
 ...
 ...
   ...
   ...
   ...

LLLLLRL1";
        let state = dos(input, Part::One);
        assert_eq!(state.position, (1, 2))
    }
    #[test]
    fn wraps_down() {
        let input = "...
...
...
  ...
  ...
  ...

1R3";
        let state = dos(input, Part::One);
        assert_eq!(state.position, (1, 0));
        assert_eq!(state.orientation, 1);
    }
    #[test]
    fn wraps_right() {
        let input = "...
...
...
  ...
  ...
  ...

3";
        let state = dos(input, Part::One);
        assert_eq!(state.position, (0, 0));
        assert_eq!(state.orientation, 0);
    }

    #[test]
    fn wraps_left() {
        let input = "...
...
...
  ...
  ...
  ...

1RR3";
        let state = dos(input, Part::One);
        assert_eq!(state.position, (1, 0));
        assert_eq!(state.orientation, 2);
    }

    #[test]
    fn wraps_wall() {
        let input = "...
...
.#.
  ...
  ...
  ...

1L10";
        let state = dos(input, Part::One);
        assert_eq!(state.position, (1, 0));
        assert_eq!(state.orientation, 3);
    }
    #[test]
    fn wraps_alot() {
        let input = "...
...
.#.
  ...
  ...
  ...

2R61";
        let state = dos(input, Part::One);
        state.pp();
        assert_eq!(state.position, (2, 1));
        assert_eq!(state.orientation, 1);
    }
}
