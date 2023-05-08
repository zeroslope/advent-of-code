use std::io::{BufRead, Cursor};

fn main() {
    let f = include_str!("./input.txt");
    let cursor = Cursor::new(f);
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();
    let n = lines.peek().unwrap().len();
    let mut graph = Vec::with_capacity(n);
    for line in lines {
        let mut row = Vec::with_capacity(n);
        for &b in line.as_bytes() {
            row.push((b - b'0') as i32);
        }
        graph.push(row);
    }
    let mut part1 = 0;
    let mut vis = vec![vec![false; n]; n];
    for i in 0..n {
        let mut max = -1;
        for j in 0..n {
            if graph[i][j] > max {
                max = graph[i][j];
                if !vis[i][j] {
                    part1 += 1;
                }
                vis[i][j] = true;
            }
        }
        let mut max = -1;
        for j in (0..n).rev() {
            if graph[i][j] > max {
                max = graph[i][j];
                if !vis[i][j] {
                    part1 += 1;
                }
                vis[i][j] = true;
            }
        }
    }

    for j in 0..n {
        let mut max = -1;
        for i in 0..n {
            if graph[i][j] > max {
                max = graph[i][j];
                if !vis[i][j] {
                    part1 += 1;
                }
                vis[i][j] = true;
            }
        }
        let mut max = -1;
        for i in (0..n).rev() {
            if graph[i][j] > max {
                max = graph[i][j];
                if !vis[i][j] {
                    part1 += 1;
                }
                vis[i][j] = true;
            }
        }
    }

    dbg!(part1);

    let mut part2 = 0;
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            // dbg!(i, j, graph[i][j]);
            let mut score = 1;

            let mut x = i - 1;
            while x > 0 {
                if graph[x][j] >= graph[i][j] {
                    break;
                }
                x -= 1;
            }
            // println!("up {}", i - x);
            score *= i - x;

            let mut x = i + 1;
            while x < n - 1 {
                if graph[x][j] >= graph[i][j] {
                    break;
                }
                x += 1;
            }
            // println!("down {}", x - i);
            score *= x - i;

            let mut y = j - 1;
            while y > 0 {
                if graph[i][y] >= graph[i][j] {
                    break;
                }
                y -= 1;
            }
            // println!("left {}", j - y);
            score *= j - y;

            let mut y = j + 1;
            while y < n - 1 {
                if graph[i][y] >= graph[i][j] {
                    break;
                }
                y += 1;
            }
            // println!("right {}", y - j);
            score *= y - j;

            if score > part2 {
                part2 = score;
            }
        }
    }
    dbg!(part2);
}
