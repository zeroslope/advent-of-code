use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::io::{BufRead, Cursor};

fn check_row(data: &[((i64, i64), (i64, i64))], y: i64) -> (Vec<(i64, i64)>, HashSet<i64>) {
    let mut beacon_y = HashSet::new();
    let mut segs = Vec::new();

    for (sensor, beacon) in data {
        if beacon.1 == y {
            beacon_y.insert(beacon.0);
        }

        let dis = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();
        let dis_y = (sensor.1 - y).abs();
        if dis >= dis_y {
            let dis_x = dis - dis_y;
            // println!("{:?} {:?}  dis_x {}", sensor, beacon, dis_x);
            segs.push((sensor.0 - dis_x, sensor.0 + dis_x));
        }
    }

    segs.sort();
    // println!("{:?}", segs);
    let mut combine_segs = Vec::new();
    let mut l = segs[0].0;
    let mut r = segs[0].1;
    for i in 0..segs.len() {
        if segs[i].0 > r + 1 {
            combine_segs.push((l, r));
            l = segs[i].0;
            r = segs[i].1;
        } else {
            r = r.max(segs[i].1);
        }
    }
    combine_segs.push((l, r));
    (combine_segs, beacon_y)
}

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();
    let coord_re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

    let mut data = Vec::new();

    let y = 2000000;

    for line in lines {
        let mut caps = coord_re.captures_iter(&line);
        let cap = caps.next().unwrap();
        let sensor = (
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        let cap = caps.next().unwrap();
        let beacon = (
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        data.push((sensor, beacon));
    }

    let (combine_segs, beacon_y) = check_row(&data, 10);
    println!(
        "part1 = {}",
        combine_segs.iter().fold(0, |acc, seg| {
            let cnt = beacon_y.iter().fold(0, |acc, &x| {
                if seg.0 <= x && x <= seg.1 {
                    acc + 1
                } else {
                    acc
                }
            });
            acc + (seg.1 - seg.0 + 1) - cnt
        })
    );

    for i in 0..4000000 {
        let (combine_segs, _) = check_row(&data, i);
        if combine_segs.len() > 1 {
            println!("i= {} segs= {:?}", i, combine_segs);
            println!("part2 = {}", (combine_segs[0].1 + 1) * 4000000 + i);
            break;
        }
    }

    Ok(())
}
