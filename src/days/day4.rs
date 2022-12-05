use std::fs;
type SectionPair = ((i32, i32), (i32, i32));


fn parse_line(s : &str) -> Option<SectionPair> {
    let (p1, p2)   = match s.split(",").collect::<Vec<&str>>()[..] { [a,b] => Some((a,b)), _ => None }?;
    let (p11, p12) = match p1.split("-").collect::<Vec<&str>>()[..] { [a,b] => Some((a,b)), _ => None }?;
    let (p21, p22) = match p2.split("-").collect::<Vec<&str>>()[..] { [a,b] => Some((a,b)), _ => None }?;

    return Some(((p11.parse().ok()?, p12.parse().ok()?), (p21.parse().ok()?, p22.parse().ok()?)));
}

fn is_contained(p : SectionPair) -> bool {
    let ((a, b), (c,d)) = p;

    return a <= c && b >= d || c <= a && d >= b;
}

fn is_overlap(p : SectionPair) -> bool {
    let ((a, b), (c,d)) = p;

    let no_overlap = a > d || b < c;
    return !no_overlap
}

fn parse_file() -> Vec<SectionPair> {
    let contents = fs::read_to_string("./04.txt")
        .expect("Should have been able to read the file");
    let intervals = contents.split("\n")
        .filter_map(parse_line)
        .collect::<Vec<SectionPair>>();

    return intervals;
}

pub fn solution() {
    let assignments = parse_file();

    let count = assignments.clone().into_iter().filter(|x| is_contained(x.clone())).count();
    println!("Sol1 {}", count);

    let count = &assignments.into_iter().filter(|x| is_overlap(x.clone())).count();
    println!("Sol2 {}", count);
}
