use std::io::{self, BufRead, Cursor};

fn main() -> io::Result<()> {
    let f = include_str!("./input.txt");
    let f = Cursor::new(f);

    let mut calories = Vec::new();
    let mut single = Vec::new();
    for line in f.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            calories.push(single.clone());
            single.clear();
        } else {
            single.push(line.parse::<u32>().unwrap());
        }
    }
    let mut cnts: Vec<u32> = calories.iter().map(|c| c.iter().sum()).collect();
    cnts.sort_by(|a, b| b.cmp(a));

    dbg!(cnts[0]);

    dbg!(cnts[0] + cnts[1] + cnts[2]);

    Ok(())
}
