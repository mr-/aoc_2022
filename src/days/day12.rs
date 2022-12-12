use itertools::Itertools;
use pathfinding::prelude::bfs;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(i32, i32);
type Map = HashMap<Point, i32>;

fn parse_file(input: &str) -> (Map, Point, Point, i32, i32) {
    let mut start: Point = Point(0, 0);
    let mut end: Point = Point(0, 0);
    let mut map: HashMap<Point, i32> = HashMap::new();
    let lines = input.split("\n").collect::<Vec<&str>>();
    for y in 0..lines.len() {
        let cs = lines[y].chars().collect::<Vec<char>>();
        for x in 0..cs.len() {
            let mut vals: HashMap<char, i32> = ('a'..='z').zip(1..=26).collect();
            vals.insert('S', 1);
            vals.insert('E', 26);
            let p = Point(x as i32, y as i32);
            map.insert(p.clone(), vals[&cs[x]]);
            if cs[x] == 'S' {
                start = p.clone();
            }
            if cs[x] == 'E' {
                end = p.clone();
            }
        }
    }
    return (map, start, end, lines[0].len() as i32, lines.len() as i32);
}
fn successors(map: &Map, p: &Point) -> Vec<Point> {
    let &Point(x, y) = p;
    vec![
        Point(x + 1, y),
        Point(x - 1, y),
        Point(x, y + 1),
        Point(x, y - 1),
    ]
    .into_iter()
    // can go from p to q
    // - if p >= q
    // - if p == q+1
    // 4 -> 5
    // 4 -> 4
    // 4 -> 3
    .filter(|q| map.contains_key(q) && map[p] + 1 >= map[q])
    .collect()
}

pub fn solution() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    let input =
        &fs::read_to_string("./input/12.txt").expect("Should have been able to read the file");
    let (map, start, end, mx, my) = parse_file(input);

    let result = bfs(&start, |p| successors(&map, p), |p| *p == end);
    println!("sol1: {:?}", result.map(|v| v.len() - 1));

    let starting_points: Vec<Point> = map
        .clone()
        .into_iter()
        .filter(|(p, v)| v == &1)
        .map(|(p, _)| p.clone())
        .collect();
    let paths = starting_points
        .into_iter()
        .filter_map(|start| bfs(&start, |p| successors(&map, p), |p| *p == end))
        .map(|v| v.len() - 1)
        .sorted()
        .collect::<Vec<usize>>();
    println!("sol2: {:?}", paths)
}
