use regex::Regex;
use std::io::{BufRead, Cursor};

fn main() {
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let line = lines.next().unwrap();
    let mut cnt = vec![0; 26];
    let mut l = 0;
    let mut part1 = 0;
    let bytes = line.as_bytes();
    for (i, &ch) in bytes.iter().enumerate() {
        let ch = (ch - b'a') as usize;
        while cnt[ch] > 0 {
            cnt[(bytes[l] - b'a') as usize] -= 1;
            l += 1;
        }
        cnt[ch] += 1;
        // dbg!(i, l);
        if i - l + 1 == 4 {
            part1 = i + 1;
            break;
        }
    }
    dbg!(part1);

    let mut cnt = vec![0; 26];
    let mut l = 0;
    let mut part2 = 0;
    for (i, &ch) in bytes.iter().enumerate() {
        let ch = (ch - b'a') as usize;
        while cnt[ch] > 0 {
            cnt[(bytes[l] - b'a') as usize] -= 1;
            l += 1;
        }
        cnt[ch] += 1;
        if i - l + 1 == 14 {
            part2 = i + 1;
            break;
        }
    }
    dbg!(part2);
}
