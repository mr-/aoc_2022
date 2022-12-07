use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::mem::replace;
use std::vec::Vec;

#[derive(Clone, Debug)]
enum Cmd {
    Cd(String),
    Ls(Vec<File>),
}

// 14848514 b.txt
// 8504156 c.dat
fn parse_ls(line: &str) -> Option<File> {
    let re = Regex::new(r"^(\d+) (.+)$").ok()?;
    let cap = re.captures(line)?;
    Some((cap[2].to_string(), cap[1].parse().ok()?))
}

// $ cd /
// $ ls
fn parse_block(block: &str) -> Option<Cmd> {
    let split_lines = block.trim_start().split("\n").collect::<Vec<&str>>();
    let mut lines = split_lines.into_iter();
    let line = lines.next()?;

    let re = Regex::new(r"^cd (.+)$").unwrap();
    if let Some(cap) = re.captures(line) {
        return Some(Cmd::Cd(cap[1].to_string()));
    }
    let re = Regex::new(r"^ls$").unwrap();
    if let Some(_) = re.captures(line) {
        let files = lines
            .take_while(|x| !x.starts_with("$"))
            .filter_map(|x| parse_ls(x))
            .collect();
        return Some(Cmd::Ls(files));
    }
    None
}

fn parse_file(content: &str) -> Vec<Cmd> {
    let blocks = content.split("$");
    blocks.into_iter().filter_map(parse_block).collect()
}

type Path = Vec<String>;
type File = (String, usize);
type DirSize = (Path, usize);

fn to_file_list(cmds: &mut Vec<Cmd>, prefix: Path, current: &mut Vec<(Path, Vec<File>)>) {
    let Some(cmd) = cmds.pop() else { return; };
    match cmd {
        Cmd::Cd(s) => {
            let mut new_prefix = prefix.clone();
            if s == ".." {
                new_prefix.pop();
            } else {
                new_prefix.push(s);
            }
            to_file_list(cmds, new_prefix, current);
        }
        Cmd::Ls(files) => {
            current.push((prefix.clone(), files));
            to_file_list(cmds, prefix, current)
        }
    }
}

fn to_size_list(list: Vec<(Path, Vec<File>)>) -> Vec<DirSize> {
    list.into_iter()
        .map(|(p, files)| (p, files.iter().map(|(_, s)| s).sum()))
        .collect()
}

fn to_accum_size_list(list: Vec<DirSize>) -> Vec<DirSize> {
    list.clone()
        .into_iter()
        .map(|(p, _)| {
            (
                p.clone(),
                list.clone()
                    .into_iter()
                    .filter(|(q, _)| q.starts_with(&p))
                    .map(|(_, s)| s)
                    .sum(),
            )
        })
        .collect()
}
pub fn solution() {
    let input =
        fs::read_to_string("./input/07.txt").expect("Should have been able to read the file");
    let block = parse_file(input.as_str());

    let mut levels = Vec::new();
    let mut commands = block.into_iter().rev().collect::<Vec<Cmd>>().clone();
    to_file_list(&mut commands, Vec::new(), &mut levels);
    let accum = to_accum_size_list(to_size_list(levels));

    // Only data preparation up to here..

    let s: usize = accum
        .clone()
        .into_iter()
        .map(|(_, s)| s)
        .filter(|s| s < &100000)
        .sum();
    println!("sol1: {:?}", s);

    let total_space: usize = 70000000;
    let required_space: usize = 30000000;
    let total_used_space = accum[0].1;
    let min_to_delete = required_space - (total_space - total_used_space);

    println!("We need to delete at least {:?}", min_to_delete);

    let mut big_enough = accum
        .clone()
        .into_iter()
        .filter(|(_, s)| s >= &min_to_delete)
        .collect::<Vec<(Path, usize)>>();
    big_enough.sort_by(|a, b| a.1.cmp(&b.1));

    println!("sol2: {:?}", big_enough[0])
}
