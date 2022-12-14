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
    println!("explode {:?} -> {:?}", (ax, ay), (bx, by));
    let l = iproduct!(range(*ax, *bx), range(*ay, *by)).collect();
    println!("    into {:?}", l);
    l
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

pub fn solution() {
    let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    //let input = "1,1 -> 1,3 -> 3,3";
    let paths = lines(input).unwrap().1;
    let e = explode_paths(&paths);
    println!("{:?}", e)
}
