use std::{cmp, collections::HashSet, fs, ops::RangeInclusive};

use itertools::Itertools;
use regex::Regex;

type Point = (i32, i32);
type Circle = (Point, Point);

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15\
fn line(input: &str) -> Option<Circle> {
    let re = Regex::new(
        r"^Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)",
    )
    .unwrap();
    let cap = re.captures(input)?;
    let sensor: Point = (cap[1].parse().ok()?, cap[2].parse().ok()?);
    let beacon: Point = (cap[3].parse().ok()?, cap[4].parse().ok()?);

    Some((sensor, beacon))
}

fn parse_file(input: &str) -> Vec<Circle> {
    input.split("\n").filter_map(line).collect()
}

fn dist(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

type Interval = (i32, i32);
fn intersect(circle: &Circle, y: i32) -> Option<Interval> {
    let radius = dist(&circle.0, &circle.1);
    let c = circle.0;

    // looking for x s.t.
    // radius = dist(c, (x,y))
    // radius = |c.0 - x| + |c.1 - y|
    // |x - c.0| = radius - |y - c.1|
    // |x - c.0| = a

    let a = radius - (y - c.1).abs();
    if a < 0 {
        return None;
    }
    let p1 = -a + c.0;
    let p2 = a + c.0;

    if p1 <= p2 {
        Some((p1, p2))
    } else {
        Some((p2, p1))
    }
}

fn intersects(p: &Interval, q: &Interval) -> bool {
    !(p.1 < q.0 || p.0 > q.1)
}

fn union(is: Vec<Interval>, new: Interval) -> Vec<Interval> {
    let (intersecting, rest): (Vec<Interval>, Vec<Interval>) =
        is.iter().partition(|x| intersects(*x, &new));
    if intersecting.len() == 0 {
        return rest;
    }
    let intersection = (
        intersecting.iter().map(|(x, _)| x).min().unwrap().clone(),
        intersecting.iter().map(|(_, y)| y).max().unwrap().clone(),
    );

    rest.into_iter().chain(vec![intersection]).collect()
}

fn union_all(is: &Vec<Interval>) -> Vec<Interval> {
    let mut res = is.clone();

    for i in is {
        res = union(res, *i);
    }

    res
}

//fn union(is: &Vec<Interval>) -> Vec<Interval> {
//    let res = vec![];
//    for interval in is:

// TODO: Don't forget to substract the actual beacons on the line.
pub fn solution() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    let yline = 10;
    let input =
        &fs::read_to_string("./input/15.txt").expect("Should have been able to read the file");
    let yline = 2000000;
    let circles = parse_file(input);
    sol1(&circles, yline);

    for i in 0..4000000 {
        let (c, intervals) = sol2(&circles, i, 0, 4000000);
        if c < 4000001 {
            println!("Found something {i:?} {:?}", union_all(&intervals));
            // -> Found something 3411840 [(-458153, 2829679), (2829681, 4582777)]
            // answer is  4000000*2829680 + 3411840
            return;
        }
    }
}

fn sol1(circles: &Vec<Circle>, yline: i32) {
    let intervals: Vec<Interval> = circles
        .iter()
        .filter_map(|c| intersect(c, yline))
        .sorted_by_key(|x| x.0)
        .collect();
    let unions = union_all(&intervals);
    let point_count: i32 = unions.iter().map(|(l, r)| r - l + 1).sum();
    let beacon_count = circles
        .into_iter()
        .filter(|(_, (_, y))| *y == yline)
        .map(|(_, (x, _))| x)
        .collect::<HashSet<&i32>>()
        .len() as i32;
    println!(
        "{point_count:?} - {beacon_count:?} = {:?}",
        point_count - beacon_count
    );
}

fn sol2(circles: &Vec<Circle>, yline: i32, min_x: i32, max_x: i32) -> (i32, Vec<Interval>) {
    let intervals: Vec<Interval> = circles
        .iter()
        .filter_map(|c| intersect(c, yline))
        .sorted_by_key(|x| x.0)
        .collect();
    (
        union_all(&intervals)
            .iter()
            .map(|(a, b)| (cmp::max(min_x, *a), cmp::min(b, &max_x)))
            .map(|(l, r)| r - l + 1)
            .sum(),
        intervals,
    )
}
