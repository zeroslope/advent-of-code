use std::collections::HashSet;
use std::io::{BufRead, Cursor};

const LAST: usize = 9;

fn main() {
    let f = include_str!("./input.txt");
    let cursor = Cursor::new(f);
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();
    let mut set: HashSet<(i32, i32)> = HashSet::new();

    let mut tail = (0, 0);
    let mut head = (0, 0);

    set.insert(tail);
    for line in lines {
        let mut line = line.split_ascii_whitespace();
        let direction = line.next().unwrap();
        let step = line.next().unwrap().parse::<i32>().unwrap();
        let diff = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        // NOTE: 逻辑有问题，以part2的逻辑为准
        for _ in 0..step {
            let last = head;
            head.0 += diff.0;
            head.1 += diff.1;
            if i32::abs(head.0 - tail.0) > 1 || i32::abs(head.1 - tail.1) > 1 {
                tail = last;
                set.insert(tail);
            }
        }
    }
    dbg!(set.len());

    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut heads = vec![(0, 0); 10];
    set.insert((0, 0));

    for line in lines {
        let mut line = line.split_ascii_whitespace();
        let direction = line.next().unwrap();
        let step = line.next().unwrap().parse::<i32>().unwrap();
        let diff = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..step {
            heads[0].0 += diff.0;
            heads[0].1 += diff.1;
            for i in 1..10 {
                let mut x = heads[i - 1].0 - heads[i].0;
                let mut y = heads[i - 1].1 - heads[i].1;
                if i32::abs(x) > 1 || i32::abs(y) > 1 {
                    if x == 0 || y == 0 {
                        heads[i].0 += x / 2;
                        heads[i].1 += y / 2;
                    } else {
                        if x.abs() == 2 {
                            x /= 2;
                        }
                        if y.abs() == 2 {
                            y /= 2;
                        }
                        heads[i].0 += x;
                        heads[i].1 += y;
                    }
                }
            }
            set.insert(heads[LAST]);
        }
    }
    dbg!(set.len());
}
