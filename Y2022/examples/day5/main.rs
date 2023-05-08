use regex::Regex;
use std::io::{BufRead, Cursor};

fn main() {
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let n = (lines.peek().unwrap().len() + 1) / 4;
    let mut stack1: Vec<Vec<u8>> = vec![vec![]; n];
    while !lines.peek().unwrap().as_bytes()[1].is_ascii_digit() {
        let line = lines.next().unwrap();
        line.as_bytes().chunks(4).enumerate().for_each(|(i, w)| {
            if w[0] == b'[' {
                stack1[i].push(w[1]);
            }
        });
    }
    stack1.iter_mut().for_each(|v| v.reverse());
    let mut stack2 = stack1.clone();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let lines = lines.skip(2);
    for line in lines {
        let caps = re.captures(&line).unwrap();
        let cnt = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        for _ in 0..cnt {
            let val = stack1[from].pop().unwrap();
            stack1[to].push(val);
        }

        let len = stack2[from].len();
        let val = stack2[from].split_off(len - cnt);
        stack2[to].extend(val);
    }
    let part1: Vec<u8> = stack1.into_iter().map(|s| *s.last().unwrap()).collect();
    dbg!(String::from_utf8(part1).unwrap());
    let part2: Vec<u8> = stack2.into_iter().map(|s| *s.last().unwrap()).collect();
    dbg!(String::from_utf8(part2).unwrap());
}
