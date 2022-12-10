use std::collections::HashMap;
use std::fs;

pub fn solution() {
    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    let points = HashMap::from([
        (("A", "X"), 1 + 3),
        (("A", "Y"), 2 + 6),
        (("A", "Z"), 3 + 0),
        (("B", "X"), 1 + 0),
        (("B", "Y"), 2 + 3),
        (("B", "Z"), 3 + 6),
        (("C", "X"), 1 + 6),
        (("C", "Y"), 2 + 0),
        (("C", "Z"), 3 + 3),
    ]);

    let contents =
        fs::read_to_string("./input/02.txt").expect("Should have been able to read the file");

    let s: Vec<Vec<&str>> = contents
        .split("\n")
        .map(|x| x.split(" ").collect())
        .filter(|x: &Vec<&str>| x.len() > 1)
        .collect();
    let sol1: i32 = s.clone().into_iter().map(|x| points[&(x[0], x[1])]).sum();
    println!("sol1 {}", sol1);

    // X Loss, Y Draw, Z Win
    // A for Rock, B for Paper, and C for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    let points2 = HashMap::from([
        (("A", "X"), 0 + 3),
        (("A", "Y"), 3 + 1),
        (("A", "Z"), 6 + 2),
        (("B", "X"), 0 + 1),
        (("B", "Y"), 3 + 2),
        (("B", "Z"), 6 + 3),
        (("C", "X"), 0 + 2),
        (("C", "Y"), 3 + 3),
        (("C", "Z"), 6 + 1),
    ]);
    let sol2: i32 = s.into_iter().map(|x| points2[&(x[0], x[1])]).sum();
    println!("sol2 {}", sol2);
}
