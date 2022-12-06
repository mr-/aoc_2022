use std::fs;

pub fn solution() {
    let contents = fs::read_to_string("./input/01.txt")
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
