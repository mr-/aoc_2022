use itertools::{iproduct, izip, Itertools};
use regex::Regex;
use std::{collections::HashSet, fs, thread};

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
fn get_neighbors(&(x, y, z): &Cube) -> HashSet<Cube> {
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
        .intersection(&get_neighbors(&q))
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
    println!("Sol1: {res:?}");
    let x = input.clone().into_iter().map(|(x, _, _)| x).minmax();
    let y = input.clone().into_iter().map(|(_, x, _)| x).minmax();
    let z = input.clone().into_iter().map(|(_, _, x)| x).minmax();

    println!("{x:?}, {y:?}, {z:?}");
    // -> MinMax(0, 19), MinMax(0, 18), MinMax(0, 19)

    // Idea: Take a bounding box between -1 and 20
    // Grow, recursively, from -1,-1,-1 in all directions..
    // Take the box, substract growth and clump. That gives the enclosed cubes.
    // Subtract the surface are from the enclosed cubes from the clump's surface area.
    let mut growth = HashSet::new();
    grow(&clump, &mut growth, &(-1, -1, -1));

    let outer: HashSet<Cube> = clump.union(&growth).cloned().collect();
    let outer_box = outer_box();
    let inner = outer_box.symmetric_difference(&outer);

    let mut inner_clump = HashSet::new();
    let inner_surface: i32 = inner.into_iter().map(|&q| add(&mut inner_clump, q)).sum();

    let sol2 = res - inner_surface;
    println!("Sol2 {sol2:?}")
}

fn outer_box() -> HashSet<Cube> {
    iproduct!(-1..21, -1..21, -1..21).collect()
}

fn is_inside(&(x, y, z): &Cube) -> bool {
    x >= -1 && x <= 20 && y >= -1 && y <= 20 && z >= -1 && z <= 20
}

fn grow(clump: &HashSet<Cube>, growth: &mut HashSet<Cube>, q: &Cube) {
    let neighbors = get_neighbors(q);
    let foo = growth.clone();
    let new_growth = neighbors
        .iter()
        .filter(|q| is_inside(q))
        .filter(|q| !clump.contains(q))
        .filter(|q| !foo.contains(q));
    growth.extend(new_growth.clone());
    for p in new_growth {
        grow(clump, growth, p);
    }
}
