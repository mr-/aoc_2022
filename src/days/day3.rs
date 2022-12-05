use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

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

pub fn solution() {
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
