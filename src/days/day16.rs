use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::{iproduct, Itertools};
use regex::Regex;

// Valve GG has flow rate=0; tunnels lead to valves FF, HH
fn parse_line(input: &str) -> Option<ValveDef> {
    let re =
        Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap();
    let c = re.captures(input)?;
    let valve = &c[1];
    let rate: i32 = c[2].parse().ok()?;
    let children = c[3].split(", ").map(|x| x.to_string()).collect();

    Some((valve.to_string(), rate, children))
}

fn parse_lines(input: &str) -> Vec<ValveDef> {
    input.split("\n").filter_map(parse_line).collect()
}

type ValveDef = (String, i32, Vec<String>);
type ValveMap<'a> = HashMap<&'a str, &'a ValveDef>;

fn min(a: Option<&i32>, b: Option<&i32>) -> Option<i32> {
    if a.is_none() {
        return b.copied();
    }
    if b.is_none() {
        return a.copied();
    }
    if a < b {
        a.copied()
    } else {
        b.copied()
    }
}

fn plus(a: Option<&i32>, b: Option<&i32>) -> Option<i32> {
    a.and_then(|ra| b.and_then(|rb| Some(ra + rb)))
}

fn distances(tree: &ValveMap) -> HashMap<(String, String), i32> {
    let edges: HashSet<(String, String)> = tree
        .into_iter()
        .flat_map(|x| {
            let xr = x.0.clone();
            return tree
                .clone()
                .get(&xr)
                .unwrap()
                .2
                .clone()
                .into_iter()
                .flat_map(|y| [(xr.to_string(), y.clone()), (y.clone(), xr.to_string())]);
        })
        .collect();

    let mut dist: HashMap<(String, String), i32> = HashMap::new();
    for x in edges {
        dist.insert(x, 1);
    }

    let nodes = tree.keys().collect::<Vec<&&str>>();
    for (k, i, j) in iproduct!(nodes.clone(), nodes.clone(), nodes.clone()) {
        let dij = dist.get(&(i.to_string(), j.to_string()));
        let dkj = dist.get(&(k.to_string(), j.to_string()));
        let dik = dist.get(&(i.to_string(), k.to_string()));
        let p = plus(dik, dkj);
        let ndij = min(dij, p.as_ref());
        //println!("{i}-{j} = {ndij:?} = min({dij:?}, {dik:?} + {dkj:?}) = min({dij:?}, {p:?})");
        if ndij.is_some() {
            dist.insert((i.to_string(), j.to_string()), ndij.unwrap());
        }
    }
    dist
}

fn do_explore(
    old_remaining: i32,
    node: String,
    todo: HashMap<&str, i32>,
    distances: &HashMap<(String, String), i32>,
) -> i32 {
    todo.iter()
        .map(|(next, valve)| {
            let mut new_todo = todo.clone();
            new_todo.remove(next);

            let d = distances.get(&(node.clone(), next.to_string())).unwrap();
            let remaining_time = old_remaining - d - 1;
            if remaining_time <= 0 {
                return 0;
            }
            return remaining_time * valve
                + do_explore(remaining_time, next.to_string(), new_todo, distances);
        })
        .max()
        .unwrap_or(0)
}

fn do_explore_2(
    old_remaining_a: i32,
    old_remaining_b: i32,
    node_a: String,
    node_b: String,
    todo: HashMap<&str, i32>,
    distances: &HashMap<(String, String), i32>,
) -> i32 {
    todo.iter()
        .permutations(2)
        //    iproduct!(todo.iter(), todo.iter())
        .map(|l| {
            let [(next_a, valve_a), (next_b, valve_b)] = *l else {panic!("OH NO")};
            let mut new_todo = todo.clone();
            new_todo.remove(next_a);
            new_todo.remove(next_b);

            let d_a = distances
                .get(&(node_a.clone(), next_a.to_string()))
                .unwrap();
            let remaining_time_a = max(old_remaining_a - d_a - 1, 0);

            let d_b = distances
                .get(&(node_b.clone(), next_b.to_string()))
                .unwrap();
            let remaining_time_b = max(old_remaining_b - d_b - 1, 0);

            return remaining_time_a * valve_a
                + remaining_time_b * valve_b
                + do_explore_2(
                    remaining_time_a,
                    remaining_time_b,
                    next_a.to_string(),
                    next_b.to_string(),
                    new_todo,
                    distances,
                );
        })
        .max()
        .unwrap_or(0)
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn solution() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    //    let input =
    //        &fs::read_to_string("./input/16.txt").expect("Should have been able to read the file");
    let valves = parse_lines(input);
    let valve_map: HashMap<&str, &ValveDef> =
        valves.iter().map(|x @ (i, _, _)| (i.as_str(), x)).collect();
    let mut vm: HashMap<&str, i32> = valve_map
        .clone()
        .into_iter()
        .map(|(x, (_, v, _))| (x, *v))
        .filter(|(_, v)| *v > 0)
        .collect();

    let start = "AA";
    vm.remove(&start);
    let d = distances(&valve_map);
    let sol1 = do_explore(30, start.to_string(), vm.clone(), &d);

    println!("Sol1 {sol1:?}");

    let sol2 = do_explore_2(26, 26, start.to_string(), start.to_string(), vm, &d);
    println!("Sol2 {sol2:?}");
}
