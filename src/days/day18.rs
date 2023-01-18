use itertools::izip;
use regex::Regex;
use std::{collections::HashSet, fs};

fn parse_line(input: &str) -> Option<Cube> {
    let re = Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
    let cap = re.captures(input)?;

    Some((
        cap[1].parse().ok()?,
        cap[2].parse().ok()?,
        cap[3].parse().ok()?,
    ))
}

fn parse_input() -> Vec<Cube> {
    let input =
        &fs::read_to_string("./input/18.txt").expect("Should have been able to read the file");
    input.lines().filter_map(parse_line).collect()
}

type Cube = (i32, i32, i32);
fn get_neighbors((x, y, z): Cube) -> HashSet<Cube> {
    [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]
    .iter()
    .map(|(a, b, c)| (x + a, y + b, z + c))
    .collect()
}

fn add(all: &mut HashSet<Cube>, q: Cube) -> i32 {
    let int: i32 = all
        .intersection(&get_neighbors(q))
        .count()
        .try_into()
        .unwrap();
    all.insert(q);

    return 6 - int - int;
}

pub fn solution() {
    let mut clump = HashSet::new();
    let input = [
        (2, 2, 2),
        (1, 2, 2),
        (3, 2, 2),
        (2, 1, 2),
        (2, 3, 2),
        (2, 2, 1),
        (2, 2, 3),
        (2, 2, 4),
        (2, 2, 6),
        (1, 2, 5),
        (3, 2, 5),
        (2, 1, 5),
        (2, 3, 5),
    ];

    let input = parse_input();

    let res: i32 = input.iter().map(|&q| add(&mut clump, q)).sum();
    println!("Sol1: {res:?}")
}
