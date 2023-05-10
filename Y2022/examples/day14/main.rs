use anyhow::Result;
use std::io::{BufRead, Cursor};

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut graph = vec![vec![0; 1000]; 1000];

    let mut max_y = 0;

    for line in lines {
        let rocks: Vec<_> = line
            .split("->")
            .map(|s| s.trim())
            .map(|s| {
                let mut parts = s.split(',');
                let x = parts.next().unwrap().parse::<usize>().unwrap();
                let y = parts.next().unwrap().parse::<usize>().unwrap();
                (x, y)
            })
            .collect();
        let n = rocks.len();
        // dbg!(&rocks);
        for i in 0..n - 1 {
            let (x1, y1) = rocks[i];
            let (x2, y2) = rocks[i + 1];

            max_y = max_y.max(y1);
            max_y = max_y.max(y2);

            if x1 == x2 {
                let min = y1.min(y2);
                let max = y1.max(y2);
                for y in min..=max {
                    graph[y][x1] = 1;
                }
            } else {
                let min = x1.min(x2);
                let max = x1.max(x2);
                for x in min..=max {
                    graph[y1][x] = 1;
                }
            }
        }
    }

    // for i in 0..10 {
    //     for j in 494..504 {
    //         if graph[i][j] == 1 {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let graph2 = graph.clone();

    let mut count = 0;
    'outer: loop {
        let mut sand = (500, 0);
        for _ in 0..500 {
            let x = &mut sand.0;
            let y = &mut sand.1;
            if graph[*y + 1][*x] == 0 {
                // 向下滑
                *y += 1;
            } else if graph[*y + 1][*x - 1] == 0 {
                // 向左下滑
                *y += 1;
                *x -= 1;
            } else if graph[*y + 1][*x + 1] == 0 {
                // 向右下滑
                *y += 1;
                *x += 1;
            } else {
                // 放下
                graph[*y][*x] = 2;
                count += 1;
                break;
            }
            if *y > max_y {
                break 'outer;
            }
        }
    }

    dbg!(count);

    let mut graph = graph2;

    let mut count = 0;
    loop {
        let mut sand = (500, 0);
        if graph[sand.1][sand.0] == 2 {
            break;
        }

        for _ in 0..500 {
            let x = &mut sand.0;
            let y = &mut sand.1;
            if graph[*y + 1][*x] == 0 {
                // 向下滑
                *y += 1;
            } else if graph[*y + 1][*x - 1] == 0 {
                // 向左下滑
                *y += 1;
                *x -= 1;
            } else if graph[*y + 1][*x + 1] == 0 {
                // 向右下滑
                *y += 1;
                *x += 1;
            } else {
                // 放下
                graph[*y][*x] = 2;
                count += 1;
                break;
            }
            if *y == max_y + 1 {
                graph[*y][*x] = 2;
                count += 1;
                break;
            }
        }
    }

    dbg!(count);

    Ok(())
}
