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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Op {
    Add,
    Minus,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node {
    Number(i64),
    Expression(String, Op, String),
}

impl Node {
    fn parse(line: &str) -> IResult<&str, (String, Node)> {
        map(
            tuple((
                take_while_m_n(4, 4, |c: char| c.is_alpha()),
                tag(": "),
                alt((
                    map(complete::i64, |n| Node::Number(n)),
                    map(
                        tuple((
                            take_while_m_n(4, 4, |c: char| c.is_alpha()),
                            delimited(tag(" "), one_of("+-*/"), tag(" ")),
                            take_while_m_n(4, 4, |c: char| c.is_alpha()),
                        )),
                        |(a, op, b): (&str, char, &str)| match (a, op, b) {
                            (a, '+', b) => Node::Expression(a.to_string(), Op::Add, b.to_string()),
                            (a, '-', b) => {
                                Node::Expression(a.to_string(), Op::Minus, b.to_string())
                            }
                            (a, '*', b) => Node::Expression(a.to_string(), Op::Mul, b.to_string()),
                            (a, '/', b) => Node::Expression(a.to_string(), Op::Div, b.to_string()),
                            _ => unreachable!(),
                        },
                    ),
                )),
            )),
            |(name, _, Node)| (name.to_string(), Node),
        )(line)
    }
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<String, Node>,
    is_humn_parent: HashMap<String, bool>,
}

impl Graph {
    fn parse(lines: impl Iterator<Item = String>) -> Graph {
        let mut nodes = HashMap::new();
        for line in lines {
            let (name, node) = Node::parse(&line).unwrap().1;
            nodes.insert(name, node);
        }
        Graph {
            nodes,
            ..Default::default()
        }
    }

    fn part1(&self) -> i64 {
        self.dfs("root")
    }

    fn part2(&mut self) {
        let root = self.nodes.get("root").unwrap().clone();
        if let Node::Expression(lhs, op, rhs) = root {
            let (not_humn, humn) = if self.has_humn(&lhs) {
                (rhs, lhs)
            } else {
                (lhs, rhs)
            };
            let not_humn_val = self.dfs(&not_humn);
            println!("not_humn: {}; humn: {}", not_humn_val, self.dfs(&humn));
            self.calc_humn(&humn, not_humn_val);
        }
    }

    fn dfs(&self, name: &str) -> i64 {
        match self.nodes.get(name).unwrap() {
            Node::Number(num) => *num,
            Node::Expression(lhs, op, rhs) => {
                let lhs = self.dfs(lhs);
                let rhs = self.dfs(rhs);
                match op {
                    Op::Add => lhs + rhs,
                    Op::Minus => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                }
            }
        }
    }

    fn calc_humn(&self, name: &str, val: i64) {
        match self.nodes.get(name).unwrap() {
            Node::Number(_) => {
                println!("humn: {}", val);
            }
            Node::Expression(lhs, op, rhs) => {
                if let Some(true) = self.is_humn_parent.get(lhs) {
                    let other = self.dfs(rhs);
                    let need = match op {
                        Op::Add => val - other,
                        Op::Minus => val + other,
                        Op::Mul => val / other,
                        Op::Div => val * other,
                    };
                    println!("{} {:?} {} = {}, need = {}", lhs, op, other, val, need);
                    self.calc_humn(lhs, need);
                } else {
                    let other = self.dfs(lhs);
                    let need = match op {
                        Op::Add => val - other,
                        Op::Minus => other - val,
                        Op::Mul => val / other,
                        Op::Div => other / val,
                    };
                    println!("{} {:?} {} = {}, need = {}", other, op, rhs, val, need);
                    self.calc_humn(rhs, need);
                }
            }
        }
    }

    fn has_humn(&mut self, name: &str) -> bool {
        if name == "humn" {
            self.is_humn_parent.insert(name.to_string(), true);
            return true;
        }
        let node = self.nodes.get(name).unwrap().clone();
        let res = match node {
            Node::Number(_) => false,
            Node::Expression(lhs, _, rhs) => {
                let lhs = self.has_humn(&lhs);
                let rhs = self.has_humn(&rhs);
                lhs || rhs
            }
        };
        self.is_humn_parent.insert(name.to_string(), res);
        res
    }
}

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut graph = Graph::parse(lines);
    dbg!(graph.part1());

    graph.part2();

    Ok(())
}
