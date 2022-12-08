use itertools::iproduct;
use std::collections::HashMap;
use std::fs;

type Point = (u32, u32);
type Grid = (u32, u32, HashMap<Point, u32>);

fn parse_file(content: &str) -> Grid {
    let mut grid: HashMap<Point, u32> = HashMap::new();
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let input_lines = content.split("\n");
    for line in input_lines.filter(|l| l.len() > 1) {
        j = 0;
        for character in line.chars() {
            grid.insert((i, j), character.to_digit(10).unwrap());
            j = j + 1;
        }
        i = i + 1;
    }

    return (i, j, grid);
}

fn scenic_score(grid: &Grid, p: &Point) -> u32 {
    if p.0 == 0 || p.1 == 0 || p.0 == grid.0 - 1 || p.1 == grid.1 - 1 {
        return 0;
    }

    let tree_size = grid.2[p];

    let a = (0..p.0)
        .rev()
        .map(|x| (x, p.1))
        .take_while(|q| grid.2[&q] < tree_size)
        .collect::<Vec<Point>>();
    let ca = a.len() as u32
        + if a.len() > 0 && a[a.len() - 1].0 == 0 {
            0
        } else {
            1
        };

    let b = (p.0 + 1..grid.0)
        .map(|x| (x, p.1))
        .take_while(|q| grid.2[&q] < tree_size)
        .collect::<Vec<Point>>();
    let cb = b.len() as u32
        + if b.len() > 0 && b[b.len() - 1].0 == grid.0 - 1 {
            0
        } else {
            1
        };

    let c = (0..p.1)
        .rev()
        .map(|x| (p.0, x))
        .take_while(|q| grid.2[&q] < tree_size)
        .collect::<Vec<Point>>();
    let cc = c.len() as u32
        + if c.len() > 0 && c[c.len() - 1].1 == 0 {
            0
        } else {
            1
        };

    let d = (p.1 + 1..grid.1)
        .map(|x| (p.0, x))
        .take_while(|q| grid.2[&q] < tree_size)
        .collect::<Vec<Point>>();

    let cd = d.len() as u32
        + if d.len() > 0 && d[d.len() - 1].1 == grid.1 - 1 {
            0
        } else {
            1
        };
    return ca * cb * cd * cc;
}

fn is_visible(grid: &Grid, p: &Point) -> bool {
    if p.0 == 0 || p.1 == 0 || p.0 == grid.0 || p.1 == grid.1 {
        return true;
    }

    let tree_size = grid.2[p];

    let a = (0..p.0)
        .map(|x| (x, p.1))
        .map(|q| grid.2[&q])
        .all(|x| x < tree_size);

    let b = (p.0 + 1..grid.0)
        .map(|x| (x, p.1))
        .map(|q| grid.2[&q])
        .all(|x| x < tree_size);

    let c = (0..p.1)
        .map(|x| (p.0, x))
        .map(|q| grid.2[&q])
        .all(|x| x < tree_size);

    let d = (p.1 + 1..grid.1)
        .map(|x| (p.0, x))
        .map(|q| grid.2[&q])
        .all(|x| x < tree_size);

    return a || b || c || d;
}

pub fn solution() {
    let input = "30373
25512
65332
33549
35390";

    let input =
        fs::read_to_string("./input/08.txt").expect("Should have been able to read the file");
    let grid = parse_file(&input);
    let c = iproduct!((0..grid.0), (0..grid.1))
        .filter(|p| is_visible(&grid, p))
        .count();
    println!("sol1: {:?}", c);

    let d = iproduct!((0..grid.0), (0..grid.1))
        .map(|p| scenic_score(&grid, &p))
        .max();
    println!("sol2: {:?}", d)
}
