use anyhow::Result;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::{alpha1, one_of};
use nom::character::{self, is_alphabetic};
use nom::multi::many_m_n;
use nom::sequence::{terminated, tuple};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::map,
    sequence::{delimited, preceded},
    AsChar, IResult,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, Cursor};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
enum Face {
    Right,
    Down,
    Left,
    Up,
}

impl Face {
    fn turn(self, t: Turn) -> Face {
        Face::from(self as i32 + t as i32)
    }
}

impl From<i32> for Face {
    fn from(value: i32) -> Self {
        match ((value % 4) + 4) % 4 {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
enum Turn {
    Right = 1,
    Left = -1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    Num(usize),
    Turn(Turn),
}

impl Operation {
    fn parse(line: &str) -> IResult<&str, Operation> {
        alt((
            map(complete::u64, |n| Operation::Num(n as usize)),
            map(one_of("RL"), |ch| match ch {
                'R' => Operation::Turn(Turn::Right),
                'L' => Operation::Turn(Turn::Left),
                _ => unreachable!(),
            }),
        ))(line)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn region(&self) {
        (self.0 / 50) * 3 + self.1 / 50
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn move_one_step(old: Pos, new: Pos) -> (Pos, Face) {
    let up = old.0 / 50 * 50;
    let down = up + 50;
    let left = old.1 / 50 * 50;
    let right = left + 50;
    if new.0 < up {
        match old.region() {
            1 => {}
            2 => {}
            4 => {}
            6 => {}
            7 => {}
            9 => {}
            _ => unreachable!(),
        }
    } else if new.0 >= down {
        match old.region() {
            1 => {}
            2 => {}
            4 => {}
            6 => {}
            7 => {}
            9 => {}
            _ => unreachable!(),
        }
    } else if new.1 < left {
        match old.region() {
            1 => {}
            2 => {}
            4 => {}
            6 => {}
            7 => {}
            9 => {}
            _ => unreachable!(),
        }
    } else if new.1 >= right {
        match old.region() {
            1 => {}
            2 => {}
            4 => {}
            6 => {}
            7 => {}
            9 => {}
            _ => unreachable!(),
        }
    }
    todo!()
}

#[derive(Debug)]
struct State {
    pos: Pos,
    face: Face,
}

#[derive(Debug)]
struct Graph {
    g: Vec<Vec<i32>>,
    rows: Vec<(i32, i32)>,
    cols: Vec<(i32, i32)>,
    ops: Vec<Operation>,
    print: Vec<String>,
    // state: State,
}

impl Graph {
    fn parse(lines: impl Iterator<Item = String>) -> Graph {
        let mut lines: Vec<String> = lines.collect();
        let op_line = lines.pop().unwrap();
        let mut ops: Vec<Operation> = Vec::new();
        let mut input = op_line.as_str();
        while let Ok((remain, op)) = Operation::parse(input) {
            ops.push(op);
            input = remain;
        }
        // remove empty line
        lines.pop();

        let print = lines.clone();
        let n = lines.len();
        let m = lines.iter().map(|s| s.len()).max().unwrap();

        let mut g = vec![vec![0; m]; lines.len()];
        for (i, line) in lines.into_iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch == '.' {
                    g[i][j] = 1;
                } else if ch == '#' {
                    g[i][j] = 2;
                }
            }
        }
        let mut rows = vec![(0, 0); n];
        for (i, row) in g.iter().enumerate() {
            let mut l = 0;
            let mut r = m;
            let mut find_l = false;
            for (j, item) in row.iter().enumerate() {
                if !find_l && *item != 0 {
                    l = j;
                    find_l = true;
                }
                if find_l && *item == 0 {
                    r = j;
                    break;
                }
            }
            rows[i] = (l as i32, r as i32);
        }
        let mut cols = vec![(0, 0); m];
        for j in 0..m {
            let mut l = 0;
            let mut r = n;
            let mut find_l = false;
            for i in 0..n {
                if !find_l && g[i][j] != 0 {
                    l = i;
                    find_l = true;
                }
                if find_l && g[i][j] == 0 {
                    r = i;
                    break;
                }
            }
            // println!("l = {}, r = {}", l, r);
            cols[j] = (l as i32, r as i32);
        }

        Graph {
            g,
            rows,
            cols,
            ops,
            print,
        }
    }

    fn step(&mut self, r: usize, c: usize, face: Face) {
        let s = match face {
            Face::Right => ">",
            Face::Left => "<",
            Face::Down => "v",
            Face::Up => "^",
        };
        self.print[r].replace_range(c..(c + 1), s);
    }

    fn part1(&mut self) {
        let n = self.g.len();
        let m = self.g[0].len();
        let mut state = State {
            pos: Pos(0, self.rows[0].0),
            face: Face::Right,
        };
        self.step(state.pos.0 as usize, state.pos.1 as usize, state.face);
        for i in 0..self.ops.len() {
            let op = self.ops[i];
            match op {
                Operation::Num(step) => {
                    let (min, max) = match state.face {
                        Face::Left | Face::Right => self.rows[state.pos.0 as usize],
                        Face::Down | Face::Up => self.cols[state.pos.1 as usize],
                    };
                    let delta = match state.face {
                        Face::Right => Pos(0, 1),
                        Face::Down => Pos(1, 0),
                        Face::Left => Pos(0, -1),
                        Face::Up => Pos(-1, 0),
                    };
                    for _ in 0..step {
                        let mut next = state.pos + delta;
                        match state.face {
                            Face::Left | Face::Right => {
                                next.1 = (next.1 - min + (max - min)) % (max - min) + min;
                            }
                            Face::Down | Face::Up => {
                                next.0 = (next.0 - min + (max - min)) % (max - min) + min;
                            }
                        };
                        if self.g[next.0 as usize][next.1 as usize] == 2 {
                            break;
                        } else {
                            println!("next = {:?}", next);
                            self.step(next.0 as usize, next.1 as usize, state.face);
                            state.pos = next;
                        }
                    }
                }
                Operation::Turn(t) => {
                    state.face = state.face.turn(t);
                    self.step(state.pos.0 as usize, state.pos.1 as usize, state.face);
                }
            }
        }
        println!("state: {:?}", state);
        println!(
            "result: {}",
            (state.pos.0 + 1) * 1000 + (state.pos.1 + 1) * 4 + state.face as i32
        );
        for r in self.print.iter() {
            println!("{}", r);
        }
    }

    fn part2(&mut self) {}
}

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut graph = Graph::parse(lines);
    println!("ops len = {}", graph.ops.len());
    println!(
        "ops first {:?}, last {:?}",
        graph.ops.first(),
        graph.ops.last()
    );
    println!("rows {:?}", graph.rows.len());
    println!("rows {:?}", graph.rows);
    println!("cols {:?}", graph.cols.len());
    println!("cols {:?}", graph.cols);
    graph.part1();

    Ok(())
}
