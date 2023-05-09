use bmp::{px, Image, Pixel};
use std::io::{BufRead, Cursor};

fn main() {
    let f = include_str!("./input.txt");
    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut memory = Vec::new();

    let mut x = 1;
    let mut cycle = 0;
    let mut part1 = 0;
    for line in lines {
        if line == "noop" {
            cycle += 1;
            memory.push(x);
            if (cycle + 1 - 20) % 40 == 0 {
                part1 += x * (cycle + 1);
            }
        } else {
            let v = line[5..].parse::<i32>().unwrap();
            cycle += 1;
            memory.push(x);
            if (cycle + 1 - 20) % 40 == 0 {
                part1 += x * (cycle + 1);
            }
            cycle += 1;
            x += v;
            memory.push(x);
            if (cycle + 1 - 20) % 40 == 0 {
                part1 += x * (cycle + 1);
            }
        }
    }
    dbg!(part1);

    let mut last = 0;
    let mut image = Vec::new();
    for i in 0..6 {
        let mut row = Vec::new();
        for j in 0i32..40 {
            if last <= j + 1 && j + 1 <= last + 2 {
                row.push(1);
            } else {
                row.push(0);
            }
            last = memory[i * 40 + j as usize];
        }
        image.push(row);
    }
    let mut img = Image::new(40, 6);
    for (x, y) in img.coordinates() {
        if image[y as usize][x as usize] == 1 {
            img.set_pixel(x, y, px!(255, 255, 255));
        } else {
            img.set_pixel(x, y, px!(0, 0, 0));
        }
    }
    img.save("./examples/day10/crt.bmp").unwrap();
}
