use anyhow::Result;
use std::io::{BufRead, Cursor};

fn get_coordinates(nums: &[(usize, i64)]) -> i64 {
    let n = nums.len();
    let zero = nums.iter().position(|(_, n)| *n == 0).unwrap();
    nums[(zero + 1000) % n].1 + nums[(zero + 2000) % n].1 + nums[(zero + 3000) % n].1
}

fn decryption(nums: &[(usize, i64)], times: i64) -> i64 {
    let n = nums.len();
    let nn = n as i64;
    let mut shuffle = nums.to_owned();
    for _ in 0..times {
        for i in 0..n {
            let (index, mut offset) = nums[i];
            let pos = shuffle.iter().position(|(ii, _)| *ii == index).unwrap();
            // 移动时要排除自己
            offset %= nn - 1;
            let mut new_pos = pos as i64 + offset;
            if new_pos >= nn {
                new_pos = new_pos - nn + 1;
            } else if new_pos <= 0 {
                new_pos = new_pos + nn - 1;
            }
            // dbg!(pos, new_pos, offset);
            let val = shuffle.remove(pos as usize);
            shuffle.insert(new_pos as usize, val);
            // println!("shuffle={:?}", shuffle);
        }
    }
    get_coordinates(&shuffle)
}

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut nums = Vec::new();
    for (i, line) in lines.enumerate() {
        nums.push((i, line.parse::<i64>().unwrap()));
    }
    let part1 = decryption(&nums, 1);
    println!("part1: {}", part1);
    nums.iter_mut().for_each(|(_, n)| *n *= 811589153);
    let part2 = decryption(&nums, 10);
    println!("part2: {}", part2);
    Ok(())
}
