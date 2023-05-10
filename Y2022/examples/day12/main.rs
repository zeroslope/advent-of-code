use anyhow::Result;
use std::{
    collections::VecDeque,
    io::{BufRead, Cursor},
};

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");
    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut graph: Vec<Vec<i32>> = Vec::new();
    for (i, line) in lines.into_iter().enumerate() {
        let row: Vec<i32> = line
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(j, &b)| {
                if b == b'S' {
                    start = (i, j);
                    0
                } else if b == b'E' {
                    end = (i, j);
                    25
                } else {
                    (b - b'a') as i32
                }
            })
            .collect();
        graph.push(row);
    }

    let n = graph.len();
    let m = graph[0].len();
    let mut dist = vec![vec![std::usize::MAX; m]; n];
    dist[start.0][start.1] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        let h = graph[i][j];
        let d = dist[i][j];
        if i > 0 && (graph[i - 1][j] - h <= 1) && dist[i - 1][j] > d + 1 {
            dist[i - 1][j] = d + 1;
            queue.push_back((i - 1, j));
        }
        if i < n - 1 && (graph[i + 1][j] - h <= 1) && dist[i + 1][j] > d + 1 {
            dist[i + 1][j] = d + 1;
            queue.push_back((i + 1, j));
        }
        if j > 0 && (graph[i][j - 1] - h <= 1) && dist[i][j - 1] > d + 1 {
            dist[i][j - 1] = d + 1;
            queue.push_back((i, j - 1));
        }
        if j < m - 1 && (graph[i][j + 1] - h <= 1) && dist[i][j + 1] > d + 1 {
            dist[i][j + 1] = d + 1;
            queue.push_back((i, j + 1));
        }
    }

    dbg!(dist[end.0][end.1]);

    // reverse the height, from end to source
    for i in 0..n {
        for j in 0..m {
            graph[i][j] = 25 - graph[i][j];
        }
    }
    let mut dist = vec![vec![std::usize::MAX - 10; m]; n];
    dist[end.0][end.1] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(end);

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        let h = graph[i][j];
        let d = dist[i][j];
        if i > 0 && (graph[i - 1][j] - h <= 1) && dist[i - 1][j] > d + 1 {
            dist[i - 1][j] = d + 1;
            queue.push_back((i - 1, j));
        }
        if i < n - 1 && (graph[i + 1][j] - h <= 1) && dist[i + 1][j] > d + 1 {
            dist[i + 1][j] = d + 1;
            queue.push_back((i + 1, j));
        }
        if j > 0 && (graph[i][j - 1] - h <= 1) && dist[i][j - 1] > d + 1 {
            dist[i][j - 1] = d + 1;
            queue.push_back((i, j - 1));
        }
        if j < m - 1 && (graph[i][j + 1] - h <= 1) && dist[i][j + 1] > d + 1 {
            dist[i][j + 1] = d + 1;
            queue.push_back((i, j + 1));
        }
    }

    let mut part2 = std::usize::MAX;

    for i in 0..n {
        for j in 0..m {
            if graph[i][j] == 25 {
                part2 = part2.min(dist[i][j]);
            }
        }
    }

    dbg!(part2);

    Ok(())
}
