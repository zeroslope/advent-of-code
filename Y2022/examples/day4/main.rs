use std::io::{BufRead, Cursor};

fn main() {
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    for line in cursor.lines().map(|l| l.unwrap()) {
        if !line.is_empty() {
            let mut elfs = line.split(",");
            let first = elfs.next().unwrap();
            let mut first = first.split("-");
            let l1 = first.next().unwrap().parse::<i32>().unwrap();
            let r1 = first.next().unwrap().parse::<i32>().unwrap();

            let second = elfs.next().unwrap();
            let mut second = second.split("-");
            let l2 = second.next().unwrap().parse::<i32>().unwrap();
            let r2 = second.next().unwrap().parse::<i32>().unwrap();

            if (l1 <= l2 && r2 <= r1) || (l2 <= l1 && r1 <= r2) {
                part1 += 1;
            }

            if !((r1 < l2) || (r2 < l1)) {
                part2 += 1;
            }
        }
    }
    dbg!(part1);
    dbg!(part2);
}
