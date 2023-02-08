use std::fs;

fn parse_char(input: char) -> i64 {
    match input {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("wtf.."),
    }
}

fn from_snafu(input: &str) -> i64 {
    let power: i64 = 5;
    (0..)
        .zip(input.chars().rev().map(parse_char))
        .map(|(e, v)| v * power.pow(e))
        .sum()
}

fn to_base(n: i64, base: i64) -> Vec<i64> {
    let mut res = Vec::new();
    let mut y = n;

    while y > 0 {
        let m = y % base;
        res.push(m);
        y = y / base;
    }
    res
}

fn snafuify(n: Vec<i64>) -> Vec<i64> {
    let mut carry_over: i64 = 0;
    let mut res = Vec::new();

    for part in n {
        let (d, co) = match part + carry_over {
            0 => (0, 0),
            1 => (1, 0),
            2 => (2, 0),
            3 => (-2, 1),
            4 => (-1, 1),
            5 => (0, 1),
            _ => panic!("How did I get to {:?}", part),
        };
        carry_over = co;
        res.push(d);
    }

    if carry_over > 0 {
        res.push(carry_over)
    }

    res
}

fn pp(n: Vec<i64>) -> String {
    let to_char = |x| match x {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("How did I get to {:?}", x),
    };

    n.into_iter().rev().map(to_char).collect()
}

fn to_snafu(n: i64) -> String {
    pp(snafuify(to_base(n, 5)))
}

pub fn solution() {
    let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    let input =
        &fs::read_to_string("./input/25.txt").expect("Should have been able to read the file");
    println!("Solution 25");

    let mut s = 0;
    for line in input.lines() {
        let snafu = from_snafu(line);
        s += snafu;
        let s = to_snafu(snafu);
        println!("{line:?} => {snafu:?} => {s:?}");
    }

    let res = to_snafu(s);
    println!("Sol 1: {res:?}");
}
