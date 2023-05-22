use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, Cursor};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32, i32);

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

const OP: [Point; 6] = [
    Point(0, 1, 0),
    Point(0, -1, 0),
    Point(1, 0, 0),
    Point(-1, 0, 0),
    Point(0, 0, 1),
    Point(0, 0, -1),
];

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut cubes = HashSet::new();
    for line in lines {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        let z = iter.next().unwrap().parse::<i32>().unwrap();
        cubes.insert(Point(x, y, z));
    }

    let mut part1 = cubes.len() * 6;
    for cube in cubes.iter() {
        for op in OP {
            let p = *cube + op;
            if cubes.contains(&p) {
                part1 -= 1;
            }
        }
    }
    dbg!(part1);

    let min_x = cubes.iter().map(|p| p.0).min().unwrap() - 1;
    let max_x = cubes.iter().map(|p| p.0).max().unwrap() + 1;
    let min_y = cubes.iter().map(|p| p.1).min().unwrap() - 1;
    let max_y = cubes.iter().map(|p| p.1).max().unwrap() + 1;
    let min_z = cubes.iter().map(|p| p.2).min().unwrap() - 1;
    let max_z = cubes.iter().map(|p| p.2).max().unwrap() + 1;

    let mut steams = HashSet::new();
    let mut queue = VecDeque::from([Point(min_x, min_y, min_z)]);

    while let Some(cube) = queue.pop_front() {
        for op in OP {
            let p = cube + op;
            if min_x <= p.0
                && p.0 <= max_x
                && min_y <= p.1
                && p.1 <= max_y
                && min_z <= p.2
                && p.2 <= max_z
                && !cubes.contains(&p)
                && !steams.contains(&p)
            {
                steams.insert(p);
                queue.push_back(p);
            }
        }
    }

    let mut part2 = 0;
    for cube in cubes.iter() {
        for op in OP {
            let p = *cube + op;
            if steams.contains(&p) {
                part2 += 1;
            }
        }
    }
    dbg!(part2);

    // 寻找中空的cubes
    // let mut part2 = part1;
    // for cube in cubes.iter() {
    //     for op in OP {
    //         let p = *cube + op;
    //         if !cubes.contains(&p) {
    //             let mut side = 0;
    //             for x in min_x..p.0 {
    //                 let e = Point(x, p.1, p.2);
    //                 if cubes.contains(&e) {
    //                     side += 1;
    //                     break;
    //                 }
    //             }
    //             for x in p.0 + 1..=max_x {
    //                 let e = Point(x, p.1, p.2);
    //                 if cubes.contains(&e) {
    //                     side += 1;
    //                     break;
    //                 }
    //             }
    //             for y in min_y..p.1 {
    //                 let e = Point(p.0, y, p.2);
    //                 if cubes.contains(&e) {
    //                     side += 1;
    //                     break;
    //                 }
    //             }
    //             for y in p.1 + 1..=max_y {
    //                 let e = Point(p.0, y, p.2);
    //                 if cubes.contains(&e) {
    //                     side += 1;
    //                     break;
    //                 }
    //             }
    //             for z in min_z..p.2 {
    //                 let e = Point(p.0, p.1, z);
    //                 if cubes.contains(&e) {
    //                     side += 1;
    //                     break;
    //                 }
    //             }
    //             for z in p.2 + 1..=max_z {
    //                 let e = Point(p.0, p.1, z);
    //                 if cubes.contains(&e) {
    //                     side += 1;
    //                     break;
    //                 }
    //             }
    //             if side == 6 {
    //                 part2 -= 1;
    //                 // println!("{} {} {}", p.0, p.1, p.2);
    //             }
    //         }
    //     }
    // }
    // dbg!(part2);

    Ok(())
}
