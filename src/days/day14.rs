use std::{collections::HashSet, fs, ops::RangeInclusive};

use itertools::{iproduct, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

type Point = (i32, i32);

fn number(input: &str) -> IResult<&str, i32> {
    map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        out.parse::<i32>()
    })(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map_res(tuple((number, tag(","), number)), |(a, _, b)| {
        Ok::<Point, ()>((a, b))
    })(input)
}

fn path(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list0(tag(" -> "), point)(input)
}

type Path = Vec<Point>;

fn lines(input: &str) -> IResult<&str, Vec<Path>> {
    separated_list0(tag("\n"), path)(input)
}

fn range(a: i32, b: i32) -> Vec<i32> {
    if a <= b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

fn explode((ax, ay): &Point, (bx, by): &Point) -> Vec<Point> {
    iproduct!(range(*ax, *bx), range(*ay, *by)).collect()
}

fn explode_path(path: &Path) -> HashSet<Point> {
    path.iter()
        .zip(path.iter().skip(1))
        .flat_map(|(a, b)| explode(a, b))
        .collect()
}

fn explode_paths(paths: &Vec<Path>) -> HashSet<Point> {
    paths.iter().flat_map(explode_path).collect()
}
#[derive(Debug, PartialEq)]
pub enum Status {
    Falling,
    Gone,
    Stopped,
}

fn drop_tick(wall: &HashSet<Point>, sand: &HashSet<Point>, p: &mut Point) -> Status {
    let candidates = [(p.0, p.1 + 1), (p.0 - 1, p.1 + 1), (p.0 + 1, p.1 + 1)];
    let free_point = candidates
        .iter()
        .find(|p| !wall.contains(p) && !sand.contains(p));
    if let Some(fp) = free_point {
        let max_y = wall.iter().map(|(_, y)| y).max().unwrap();
        if max_y <= &fp.1 {
            return Status::Gone;
        }
        *p = *fp;
        return Status::Falling;
    }
    return Status::Stopped;
}

fn drop_tick2(wall: &HashSet<Point>, sand: &HashSet<Point>, p: &mut Point) -> Status {
    let candidates = [(p.0, p.1 + 1), (p.0 - 1, p.1 + 1), (p.0 + 1, p.1 + 1)];
    let free_point = candidates
        .iter()
        .find(|p| !wall.contains(p) && !sand.contains(p));
    if let Some(fp) = free_point {
        let max_y = wall.iter().map(|(_, y)| y).max().unwrap();
        *p = *fp;
        if max_y + 1 == fp.1 {
            return Status::Stopped;
        }
        return Status::Falling;
    }
    return Status::Stopped;
}

fn drop(wall: HashSet<Point>, v: i32) {
    let mut start = (500, 0);
    let mut sand = HashSet::new();
    let drop_v = if v == 1 { drop_tick } else { drop_tick2 };
    loop {
        let res = drop_v(&wall, &sand, &mut start);
        if res == Status::Stopped {
            sand.insert(start.clone());
            if start == (500, 0) {
                println!("sol2: {:?}", sand.len());
                return;
            }
            start = (500, 0);
        }
        if res == Status::Gone {
            println!("sol1: {:?}", sand.len());
            return;
        }
    }
}

pub fn solution() {
    let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    let input =
        &fs::read_to_string("./input/14.txt").expect("Should have been able to read the file");
    let paths = lines(input).unwrap().1;
    let e = explode_paths(&paths);
    let max_y = e.clone().into_iter().map(|(_, y)| y).max().unwrap();
    println!("dropping to {:?}", max_y);
    drop(e.clone(), 1);
    drop(e, 2);
}
