use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solution1() {
    let contents = fs::read_to_string("./01.txt")
        .expect("Should have been able to read the file");
    let s :Vec<Vec<&str>>= contents.split("\n\n").map(|x| x.split("\n").collect()).collect();
    let bags : Vec<Vec<i32>> = s.into_iter()
        .map(|x| x.into_iter()
             .filter_map(|y| y.parse::<i32>().ok()).collect::<Vec<i32>>()
        )
        .collect();
    let mut sums = bags.into_iter()
        .map(|x| x.into_iter().sum::<i32>())
        .collect::<Vec<i32>>();
    sums.sort();
    sums.reverse();
    let sol1 = sums[0];
    let sol2 : i32 = sums.into_iter().take(3).sum();

    println!("sol 1 {:?}", sol1);
    println!("sol 2 {:?}", sol2);
}

pub fn solution2() {

    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    let points = HashMap::from([
        (("A", "X"), 1+3),
        (("A", "Y"), 2+6),
        (("A", "Z"), 3+0),

        (("B", "X"), 1+0),
        (("B", "Y"), 2+3),
        (("B", "Z"), 3+6),

        (("C", "X"), 1+6),
        (("C", "Y"), 2+0),
        (("C", "Z"), 3+3),
    ]);

    let contents = fs::read_to_string("./02.txt")
        .expect("Should have been able to read the file");

    let s :Vec<Vec<&str>>= contents
        .split("\n")
        .map(|x| x.split(" ").collect())
        .filter(|x : &Vec<&str>| x.len() > 1)
        .collect();
    let sol1 : i32 = s.clone().into_iter().map(|x| points[&(x[0],x[1])]).sum();
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
    let sol2 : i32 = s.into_iter().map(|x| points2[&(x[0],x[1])]).sum();
    println!("sol2 {}", sol2);
}

fn find_common(s : &str) -> i32 {
    let middle = s.len()/2;
    let fst : HashSet<char> = s[..middle].chars().collect();
    let snd : HashSet<char> = s[middle..].chars().collect();
    let u = fst.intersection(&snd);
    let common = u.clone().nth(0).unwrap();
    get_value(common.clone())
}

fn get_value(s:char) -> i32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let ascii_lowercase = ('a'..='z').collect::<Vec<char>>();
    let ascii_uppercase = ('A'..='Z').collect::<Vec<char>>();
    let chars : Vec<char> = ascii_lowercase.into_iter().chain(ascii_uppercase.into_iter()).collect();
    let len : i32 = 52;
    let vals : HashMap<char, i32>= chars.into_iter()
        .zip(1..=len)
        .map(|(a,b)| (a,b))
        .collect();

    vals[&s]
}

fn get_badges(bags : Vec<&str>) -> i32 {
    if bags.len() < 3 {
        return 0;
    }

    let g1 : HashSet<char> = bags[0].chars().collect();
    let g2 : HashSet<char> = bags[1].chars().collect();
    let g3 : HashSet<char> = bags[2].chars().collect();
    let i1 = g1.intersection(&g2).cloned().collect::<HashSet<char>>();
    let badges = i1.intersection(&g3).collect::<HashSet<&char>>();
    let badge = badges.into_iter().nth(0).unwrap();
    let val = get_value(badge.clone());

    let rest  :Vec<&str> = bags.into_iter().skip(3).collect();

    return val + get_badges(rest);
}

fn solution3() {
    let contents = fs::read_to_string("./03.txt")
        .expect("Should have been able to read the file");
    let sol1 : i32 = contents.split("\n")
        .filter(|x| x.len() > 1)
        .map(|x| find_common(x))
        .sum();
    println!("sol1 {}", sol1);

    let bags: Vec<&str> = contents.split("\n")
        .filter(|x| x.len() > 1)
        .collect();
    println!("sol2 {}", get_badges(bags));
}

fn main() {
    solution3()
}
