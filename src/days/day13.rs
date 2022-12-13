use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, terminated},
    IResult,
};

#[derive(Debug, Clone)]
enum List {
    N(i32),
    L(Vec<List>),
}
impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        compare(self.clone(), other.clone()) == Equal
    }
}
impl Eq for List {}
impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self.clone(), other.clone())
    }
}

fn number(input: &str) -> IResult<&str, List> {
    map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        out.parse::<i32>().map(|x| List::N(x))
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, List> {
    map_res(
        delimited(
            tag("["),
            separated_list0(tag(","), alt((number, parse_line))),
            tag("]"),
        ),
        |out: Vec<List>| Ok::<List, ()>(List::L(out)),
    )(input)
}

fn parse_block(input: &str) -> Option<(List, List)> {
    let [l, r] = input.split("\n").collect::<Vec<&str>>()[..] else {return None;};

    Some((parse_line(l).ok()?.1, parse_line(r).ok()?.1))
}

fn parse_file(input: &str) -> Vec<(List, List)> {
    input.split("\n\n").filter_map(parse_block).collect()
}

fn compare(left: List, right: List) -> Ordering {
    match (left, right) {
        (List::N(l), List::N(r)) => l.cmp(&r),
        (List::L(l), List::N(r)) => compare(List::L(l), List::L(vec![List::N(r)])),
        (List::N(l), List::L(r)) => compare(List::L(vec![List::N(l)]), List::L(r)),
        (List::L(l), List::L(r)) => {
            let zipped = l
                .clone()
                .into_iter()
                .zip(r.clone().into_iter())
                .collect::<Vec<(List, List)>>();
            for (il, ir) in zipped {
                let o = compare(il.clone(), ir.clone());
                if o != Equal {
                    return o;
                }
            }
            l.len().cmp(&r.len())
        }
    }
}

pub fn solution() {
    let cases = vec![
        ("[1,1,3,1,1]", "[1,1,5,1,1]", Less),
        ("[[1],[2,3,4]]", "[[1],4]", Less),
        ("[9]", "[[8,7,6]]", Greater),
        ("[[4,4],4,4]", "[[4,4],4,4,4]", Less),
        ("[7,7,7,7]", "[7,7,7]", Greater),
        ("[]", "[3]", Less),
        ("[[[]]]", "[[]]", Greater),
        (
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            Greater,
        ),
    ];

    for (l, r, exp) in cases {
        let pl = parse_line(l).unwrap().1;
        let pr = parse_line(r).unwrap().1;
        let v = compare(pl, pr);
        if v != exp {
            println!("{:?} < {:?} => {:?} should be {:?}", l, r, v, exp);
        }
    }
    let input =
        &fs::read_to_string("./input/13.txt").expect("Should have been able to read the file");
    let blocks = parse_file(input);
    let sol1 = (1..)
        .zip(blocks.iter())
        .filter_map(|(i, (l, r))| {
            if compare(l.clone(), r.clone()) == Less {
                Some(i)
            } else {
                None
            }
        })
        .sum::<i32>();
    println!("sol1: {:?}", sol1);

    let two = parse_line("[[2]]").unwrap().1;
    let six = parse_line("[[6]]").unwrap().1;
    let lines = input
        .split("\n")
        .filter_map(|l| parse_line(l).ok())
        .map(|x| x.1)
        .chain(vec![two.clone(), six.clone()])
        .sorted()
        .collect::<Vec<List>>();
    let ptwo = lines.clone().into_iter().position(|x| x == two).unwrap() + 1;

    let psix = lines.clone().into_iter().position(|x| x == six).unwrap() + 1;
    println!("sol2 {:?}", ptwo * psix);
}
