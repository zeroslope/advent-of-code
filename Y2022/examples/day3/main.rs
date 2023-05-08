use std::io::{BufRead, Cursor};

fn letter_count(bytes: &[u8]) -> Vec<i32> {
    let mut count = vec![0; 256];
    for &b in bytes {
        count[b as usize] += 1;
    }
    count
}

fn main() {
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let mut res: i32 = 0;
    for line in cursor.lines().map(|l| l.unwrap()) {
        if !line.is_empty() {
            let bytes = line.as_bytes();
            let n = bytes.len() / 2;
            let l = letter_count(&bytes[..n]);
            let r = letter_count(&bytes[n..]);
            for i in 0..26 {
                let ch = (b'a' + i) as usize;
                if l[ch] > 0 && r[ch] > 0 {
                    res += (i + 1) as i32;
                    break;
                }
            }
            for i in 0..26 {
                let ch = (b'A' + i) as usize;
                if l[ch] > 0 && r[ch] > 0 {
                    res += (i + 26 + 1) as i32;
                    break;
                }
            }
        }
    }
    dbg!(res);

    let cursor = Cursor::new(f);
    let mut res: i32 = 0;
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();
    while lines.peek().is_some() {
        let a = letter_count(lines.next().unwrap().as_bytes());
        let b = letter_count(lines.next().unwrap().as_bytes());
        let c = letter_count(lines.next().unwrap().as_bytes());
        for i in 0..26 {
            let ch = (b'a' + i) as usize;
            if a[ch] > 0 && b[ch] > 0 && c[ch] > 0 {
                res += (i + 1) as i32;
                break;
            }
            let ch = (b'A' + i) as usize;
            if a[ch] > 0 && b[ch] > 0 && c[ch] > 0 {
                res += (i + 26 + 1) as i32;
                break;
            }
        }
    }
    dbg!(res);
}
