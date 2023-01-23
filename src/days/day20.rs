use std::fs;

#[derive(Debug)]
struct Ring {
    ring: Vec<Entry>,
}

type Entry = (i64, usize);

impl Ring {
    pub fn new(input: &Vec<i64>) -> Self {
        return Ring {
            ring: input.iter().zip(0..).map(|(x, y)| (*x, y)).collect(),
        };
    }
    fn mix_one(&mut self, next: usize) {
        let len = self.ring.len() as i64;
        let rel = self.ring[next];
        let idx = next as i64 + rel.0;
        let idx = idx.rem_euclid(len - 1);

        self.ring.remove(next);
        self.ring.insert(idx as usize, (rel.0, rel.1));
    }

    fn mix(&mut self) {
        for next in 0..self.ring.len() {
            self.mix_one(self.ring.iter().position(|(_, p)| *p == next).unwrap())
        }
    }

    fn ev(&self) -> i64 {
        let p0 = self.ring.iter().position(|(v, _)| *v == 0).unwrap();
        [p0 + 1000, p0 + 2000, p0 + 3000]
            .iter()
            .map(|x| x % self.ring.len())
            .map(|i| self.ring[i as usize].0)
            .sum()
    }

    fn pp(&self) -> Vec<i64> {
        self.ring.iter().map(|&(x, _)| x).collect()
    }
}

pub fn solution() {
    let input = vec![1, 2, -3, 3, -2, 0, 4];

    let input = &fs::read_to_string("./input/20.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut r = Ring::new(&input);

    r.mix();
    println!("Sol1 {:?}", r.ev());
    let len = input.len();
    let input = input.iter().map(|x| x * 811589153).collect();
    let mut r = Ring::new(&input);
    for _ in 0..10 {
        r.mix();
    }
    println!("Sol2 {:?}", r.ev());
}
