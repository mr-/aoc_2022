use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::{Itertools, MinMaxResult};

#[derive(Debug, Copy, Clone)]
enum D {
    L,
    R,
}

type Shape = HashSet<(i64, i64)>;

fn shift(shape: &Shape, dir: D) -> Shape {
    match dir {
        D::L => shift_right(shape, -1),
        D::R => shift_right(shape, 1),
    }
}

fn shift_right(shape: &Shape, i: i64) -> Shape {
    shape.into_iter().map(|(x, y)| (*x + i, *y)).collect()
}

fn shift_up(shape: &Shape, i: i64) -> Shape {
    shape.into_iter().map(|(x, y)| (*x, *y + i)).collect()
}

fn drop(shape: &Shape) -> Shape {
    shape.into_iter().map(|(x, y)| (*x, *y - 1)).collect()
}

// ####
fn shape_minus() -> Shape {
    let shape = HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)]);
    shift_right(&shape, 2)
}

//   #
//  ###
//   #
fn shape_plus() -> Shape {
    let shape = HashSet::from([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]);
    shift_right(&shape, 2)
}

// ..#
// ..#
// ###
fn shape_l() -> Shape {
    let shape = HashSet::from([(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)]);
    shift_right(&shape, 2)
}
// #
// #
// #
// #
fn shape_i() -> Shape {
    let shape = HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)]);
    shift_right(&shape, 2)
}

// ##
// ##
fn shape_block() -> Shape {
    let shape = HashSet::from([(0, 0), (1, 0), (0, 1), (1, 1)]);
    shift_right(&shape, 2)
}

fn hits_walls(shape: &Shape) -> bool {
    shape.iter().any(|(x, y)| *x == -1 || *x == 7 || *y <= -1)
}

fn intersect(s1: &Shape, s2: &Shape) -> bool {
    s1.intersection(s2).peekable().peek().is_some()
}

fn pp(s: &Shape, block: &Shape) {
    let Some(y) = block.into_iter().map(|(_, y)| y).max() else {return;};

    println!("{:?}", s);
    for iy in (0..(*y + 1)).rev() {
        print!("|");
        for ix in 0..7 {
            if s.contains(&(ix, iy)) {
                print!("#")
            } else if block.contains(&(ix, iy)) {
                print!("@")
            } else {
                print!(".")
            }
        }
        println!("|")
    }
    println!("---------");
}
//
//   |   3        |------->--->---|
//   |   2       -1 0 1 2 3 4 5 6 7
//  \ /  1
//   v   0   lowest point of shape is at 0
pub fn solution() {
    println!("Sol 17");

    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let input =
        &fs::read_to_string("./input/17.txt").expect("Should have been able to read the file");
    let table = HashMap::from([('>', D::R), ('<', D::L)]);
    let mut drafts = &mut input.chars().filter_map(|c| table.get(&c)).cloned().cycle();
    let blocks = [
        shape_minus(),
        shape_plus(),
        shape_l(),
        shape_i(),
        shape_block(),
    ];
    let iterated = blocks.iter().cycle();

    let mut tower = HashSet::new();
    let mut res = 0;
    for block in iterated.clone().take(2023) {
        // possible optimization to calculate this.
        let max_y = tower
            .clone()
            .into_iter()
            .map(|(x, y)| y)
            .max()
            .unwrap_or(-1);
        println!("Max y {max_y}");
        res = max_y + 1;
        let mut b = shift_up(&block.clone(), max_y + 4);
        //        pp(&tower, &b);

        for draft in &mut drafts {
            let shifted_block = shift(&b, draft);
            let to_drop = if hits_walls(&shifted_block) || intersect(&tower, &shifted_block) {
                &b
            } else {
                &shifted_block
            };

            let dropped = drop(&to_drop);

            if hits_walls(&dropped) || intersect(&tower, &dropped) {
                tower.extend(to_drop);
                break;
            } else {
                b = dropped.clone();
            }
        }
    }
    println!("Sol1: {res:?}")
    // Sol 2: There must be a cycle somewhere so that
    // - start and end position are equal
    // - the same block drops
    // - we are at the same position in the draft.
}
