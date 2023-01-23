use std::fs;

#[derive(Debug)]
struct Ring {
    next: Option<usize>,
    ring: Vec<Entry>,
}

type Entry = (i32, bool);

impl Ring {
    pub fn new(input: &Vec<i32>) -> Self {
        return Ring {
            next: Some(0),
            ring: input.iter().map(|x| (*x, false)).collect(),
        };
    }
    fn mix_one(&mut self) -> bool {
        if let Some(next) = self.next {
            let len = self.ring.len() as i32;
            let rel = self.ring[next];
            let idx = next as i32 + rel.0;
            let idx = idx.rem_euclid(len - 1);

            self.ring.remove(next);
            self.ring.insert(idx as usize, (rel.0, true));
            let pos = self.ring.iter().position(|(_, b)| *b == false);
            self.next = pos;
            return true;
        }
        false
    }
    fn mix(&mut self) {
        while true == self.mix_one() {}
    }

    fn ev(&self) -> i32 {
        let p0 = self.ring.iter().position(|(v, _)| *v == 0).unwrap();
        [p0 + 1000, p0 + 2000, p0 + 3000]
            .iter()
            .map(|x| x % self.ring.len())
            .map(|i| self.ring[i as usize].0)
            .sum()
    }

    fn pp(&self) -> Vec<i32> {
        self.ring.iter().map(|&(x, _)| x).collect()
    }
}

pub fn solution() {
    let input = vec![1, 2, -3, 3, -2, 0, 4];

    let input = &fs::read_to_string("./input/20.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    // println!("{input:?}");

    let mut r = Ring::new(&input);
    println!("{:?}", r.pp());

    // r.mix_one();
    // println!("{:?}", r.pp());
    // println!("{:?} expected", [2, 1, -3, 3, -2, 0, 4]);
    // println!();
    // r.mix_one();
    // println!("{:?}", r.pp());
    // println!("{:?} expected", [1, -3, 2, 3, -2, 0, 4]);
    // println!();
    // r.mix_one();
    // println!("{:?}", r.pp());
    // println!("{:?} expected", [1, 2, 3, -2, -3, 0, 4]);
    // println!();
    // r.mix_one();
    // println!("{:?}", r.pp());
    // println!("{:?} expected", [1, 2, -2, -3, 0, 3, 4]);
    // println!();
    // r.mix_one();
    // println!("{:?}", r.pp());
    // println!("{:?} expected", [1, 2, -3, 0, 3, 4, -2]);
    // println!();
    // r.mix_one();
    // println!("{:?}", r.pp());
    // println!("{:?} expected", [1, 2, -3, 0, 3, 4, -2]);
    // println!();
    // r.mix_one();
    // println!("{:?}", r.pp());
    // println!("{:?} expected", [1, 2, -3, 4, 0, 3, -2]);
    // println!();

    r.mix();
    // println!("{:?}", r.pp());
    // println!("{:?} expected", [1, 2, -3, 4, 0, 3, -2]);
    println!("{:?}", r.ev());
}
