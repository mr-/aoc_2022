use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};
use par_map::ParMap;

use regex::Regex;

#[derive(Clone, Debug)]
struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: i32,
    cost_ore_robot: Cost,
    cost_clay_robot: Cost,
    cost_obsidian_robot: Cost,
    cost_geode_robot: Cost,
}

fn parse() -> Vec<Blueprint> {
    let input = "Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 13 obsidian.
Blueprint 2: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 3: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 4: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 8 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 3 ore and 13 obsidian.
Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 7: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 5 clay. Each geode robot costs 2 ore and 10 obsidian.
Blueprint 9: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 11: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 12: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 11 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 13: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 3 ore and 19 obsidian.
Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 15: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.
Blueprint 16: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 15 clay. Each geode robot costs 4 ore and 16 obsidian.
Blueprint 17: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 18: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 4 ore and 20 obsidian.
Blueprint 19: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 2 ore and 20 obsidian.
Blueprint 20: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 18 obsidian.
Blueprint 21: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 22: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 23: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 7 obsidian.
Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 9 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 25: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 6 clay. Each geode robot costs 2 ore and 20 obsidian.
Blueprint 26: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 10 clay. Each geode robot costs 2 ore and 11 obsidian.
Blueprint 27: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 16 obsidian.
Blueprint 29: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 12 obsidian.
Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 19 obsidian.";
    input.lines().filter_map(parse_blueprint).collect()
}

fn parse_blueprint(input: &str) -> Option<Blueprint> {
    let r = Regex::new(r"Blueprint (\d+):").ok()?;
    let cap = r.captures(input)?;
    return Some(Blueprint {
        id: cap[1].parse().ok()?,
        cost_ore_robot: parse_cost("ore", input)?,
        cost_clay_robot: parse_cost("clay", input)?,
        cost_obsidian_robot: parse_cost("obsidian", input)?,
        cost_geode_robot: parse_cost("geode", input)?,
    });
}
fn parse_cost(robot: &str, input: &str) -> Option<Cost> {
    let ore_robot_regex = Regex::new(
        vec![r"Each ", robot, r" robot costs ([^\.]+)"]
            .join("")
            .as_str(),
    )
    .ok()?;
    let robot_cap = ore_robot_regex.captures(input)?;
    let m = &robot_cap[1];

    let receipt = Regex::new(r"(\d+) (\w+)").ok()?;
    let foo = receipt.find_iter(m);
    let mut cost = Cost {
        ore: 0,
        clay: 0,
        obsidian: 0,
    };
    for x in foo {
        let c = receipt.captures(x.as_str())?;
        match &c[2] {
            "ore" => cost.ore = c[1].parse().ok()?,
            "clay" => cost.clay = c[1].parse().ok()?,
            "obsidian" => cost.obsidian = c[1].parse().ok()?,
            _ => panic!("well well"),
        }
    }

    Some(cost)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robots: (i32, i32),
    clay_robots: (i32, i32),
    obsidian_robots: (i32, i32),
    geode_robots: (i32, i32),
}

fn deduct(cost: &Cost, state: &State) -> Option<State> {
    if cost.ore <= state.ore && cost.clay <= state.clay && cost.obsidian <= state.obsidian {
        return Some(State {
            ore: state.ore - cost.ore,
            clay: state.clay - cost.clay,
            obsidian: state.obsidian - cost.obsidian,
            ..*state
        });
    }
    None
}

fn buy_ore_robot(bp: &Blueprint, state: &State) -> Option<State> {
    deduct(&bp.cost_ore_robot, state).map(|deducted| State {
        ore_robots: (deducted.ore_robots.0, deducted.ore_robots.1 + 1),
        ..deducted
    })
}

fn buy_clay_robot(bp: &Blueprint, state: &State) -> Option<State> {
    deduct(&bp.cost_clay_robot, state).map(|deducted| State {
        clay_robots: (deducted.clay_robots.0, deducted.clay_robots.1 + 1),
        ..deducted
    })
}

fn buy_obsidian_robot(bp: &Blueprint, state: &State) -> Option<State> {
    deduct(&bp.cost_obsidian_robot, state).map(|deducted| State {
        obsidian_robots: (deducted.obsidian_robots.0, deducted.obsidian_robots.1 + 1),
        ..deducted
    })
}

fn buy_geode_robot(bp: &Blueprint, state: &State) -> Option<State> {
    deduct(&bp.cost_geode_robot, state).map(|deducted| State {
        geode_robots: (deducted.geode_robots.0, deducted.geode_robots.1 + 1),
        ..deducted
    })
}

fn collect(state: &State) -> State {
    State {
        ore: state.ore + state.ore_robots.0,
        clay: state.clay + state.clay_robots.0,
        obsidian: state.obsidian + state.obsidian_robots.0,
        geode: state.geode + state.geode_robots.0,
        ..*state
    }
}

fn advance_robots(state: &State) -> State {
    State {
        ore_robots: (state.ore_robots.0 + state.ore_robots.1, 0),
        geode_robots: (state.geode_robots.0 + state.geode_robots.1, 0),
        obsidian_robots: (state.obsidian_robots.0 + state.obsidian_robots.1, 0),
        clay_robots: (state.clay_robots.0 + state.clay_robots.1, 0),
        ..*state
    }
}

fn apply_robot(bp: &Blueprint, robot: Robot, state: &State) -> Option<State> {
    match robot {
        Robot::Clay => buy_clay_robot(bp, state),
        Robot::Ore => buy_ore_robot(bp, state),
        Robot::Obsidian => buy_obsidian_robot(bp, state),
        Robot::Geode => buy_geode_robot(bp, state),
    }
}

fn max_geodes_upper_bound(step: i32, state: &State) -> i32 {
    //steps  5 4 3 2 1
    //bots   1 1 1 1 1
    //       0 1 2 3 4
    (step) * (step - 1) / 2 + state.geode + step * state.geode_robots.0
}

fn advance_internal(step: i32, bp: &Blueprint, state: &State) -> Vec<State> {
    let types = [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode];

    let wait = if buy_clay_robot(bp, state).is_some()
        && buy_geode_robot(bp, state).is_some()
        && buy_obsidian_robot(bp, state).is_some()
        && buy_ore_robot(bp, state).is_some()
    {
        vec![]
    } else {
        vec![state.clone()]
    };

    let res = types
        .into_iter()
        .filter_map(|x| apply_robot(bp, x, state))
        .chain(wait)
        .map(|x| collect(&x))
        .map(|x| advance_robots(&x))
        .collect::<Vec<State>>();
    return res;
}

type Cache = HashMap<(State, i32), State>;

fn advance(
    cache: &mut Cache,
    step: i32,
    bp: &Blueprint,
    state: &State,
    current_max: &mut i32,
) -> State {
    if step == 1 {
        let res = collect(state);
        return res.clone();
    }

    if max_geodes_upper_bound(step, state) <= *current_max {
        return collect(state);
    }

    let advancements = advance_internal(step, bp, state);
    let result = advancements
        .into_iter()
        .map(|x| {
            if let Some(res) = cache.get(&(x.clone(), step)) {
                return res.clone();
            } else {
                let res = advance(cache, step - 1, bp, &x, current_max);
                cache.insert((x, step), res.clone());
                return res;
            }
        })
        .max_by_key(|x| x.geode)
        .unwrap();

    if result.geode > *current_max {
        *current_max = result.geode;
    }

    result
}

pub fn solution() {
    let bp1 = Blueprint {
        id: 1,
        cost_ore_robot: Cost {
            ore: 4,
            clay: 0,
            obsidian: 0,
        },
        cost_clay_robot: Cost {
            ore: 2,
            clay: 0,
            obsidian: 0,
        },
        cost_obsidian_robot: Cost {
            ore: 3,
            clay: 14,
            obsidian: 0,
        },
        cost_geode_robot: Cost {
            ore: 2,
            clay: 0,
            obsidian: 7,
        },
    };
    let bp2 = Blueprint {
        id: 2,
        cost_ore_robot: Cost {
            ore: 2,
            clay: 0,
            obsidian: 0,
        },
        cost_clay_robot: Cost {
            ore: 3,
            clay: 0,
            obsidian: 0,
        },
        cost_obsidian_robot: Cost {
            ore: 3,
            clay: 8,
            obsidian: 0,
        },
        cost_geode_robot: Cost {
            ore: 3,
            clay: 0,
            obsidian: 12,
        },
    };

    let blueprints = parse();
    // let blueprints = [bp1, bp2];

    sol2(blueprints);
}

fn sol1(bps: Vec<Blueprint>) {
    let res: i32 = bps
        .into_iter()
        .par_map(|bp| (bp.id, solve(bp, 24)))
        .map(|(id, state)| id * state.geode)
        .sum();

    println!("sol1: {res:?}");
}

fn sol2(bps: Vec<Blueprint>) {
    let res: i32 = bps
        .into_iter()
        .take(3)
        .par_map(|bp| (bp.id, solve(bp, 32)))
        .map(|(id, state)| state.geode)
        .product();

    println!("sol2: {res:?}");
}

fn solve(bp: Blueprint, steps: i32) -> State {
    let id = bp.id;

    let mut cache = HashMap::new();
    let state = State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_robots: (1, 0),
        clay_robots: (0, 0),
        obsidian_robots: (0, 0),
        geode_robots: (0, 0),
    };
    let mut current_max = 0;
    let res = advance(&mut cache, steps, &bp, &state, &mut current_max);
    let thigns = res.geode;
    println!(" Solve {id:?}  {thigns:?}");
    return res;
}
