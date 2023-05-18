use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::{bytes::complete::tag, IResult};
use std::collections::hash_map::Entry;
use std::{
    collections::{BTreeSet, HashMap},
    io::{BufRead, Cursor},
};
use std::{dbg, println};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name([u8; 2]);

impl Name {
    fn parse(input: &str) -> IResult<&str, Name> {
        map(take(2usize), |s: &str| {
            Self(s.as_bytes().try_into().unwrap())
        })(input)
    }
}

#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    flow: u64,
    links: Vec<Name>,
}

impl Valve {
    fn parse(input: &str) -> IResult<&str, Valve> {
        map(
            tuple((
                preceded(tag("Valve "), Name::parse),
                preceded(tag(" has flow rate="), complete::u64),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), Name::parse),
                ),
            )),
            |(name, flow, links)| Self { name, flow, links },
        )(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State(usize, u64, BTreeSet<usize>);

struct Solution {
    valves: Vec<Valve>,
    valve_map: HashMap<Name, usize>,
    dis: Vec<Vec<u64>>,
    f: HashMap<State, u64>,
    state_map: HashMap<State, State>,
    opened_map: HashMap<BTreeSet<usize>, u64>,
}

impl Solution {
    fn new(valves: Vec<Valve>, dis: Vec<Vec<u64>>, valve_map: HashMap<Name, usize>) -> Self {
        Self {
            valves,
            dis,
            valve_map,
            f: HashMap::new(),
            state_map: HashMap::new(),
            opened_map: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.f = HashMap::new();
        self.state_map = HashMap::new();
        self.opened_map = HashMap::new();
    }

    fn find_best(&mut self, time: u64) -> u64 {
        let start = self.valve_map[&Name(*b"AA")];
        // don't open first
        let mut state = State(start, time, BTreeSet::new());
        let ans = self.dfs(state.clone(), 0);

        println!(
            "{} state: {:?}; reword current {} after {}",
            self.valves[state.0]
                .name
                .0
                .iter()
                .map(|&c| c as char)
                .collect::<String>(),
            state,
            0,
            self.f[&state],
        );
        while let Some(next_state) = self.state_map.get(&state) {
            println!(
                "{} state: {:?}; reword current {} after {}",
                self.valves[next_state.0]
                    .name
                    .0
                    .iter()
                    .map(|&c| c as char)
                    .collect::<String>(),
                next_state,
                self.valves[next_state.0].flow * next_state.1,
                self.f[next_state],
            );
            state = next_state.clone();
        }

        ans
    }

    fn dfs(&mut self, state: State, reword: u64) -> u64 {
        let from = state.0;
        let t = state.1;
        // update opened map
        match self.opened_map.entry(state.2.clone()) {
            Entry::Occupied(o) => {
                let v = o.into_mut();
                *v = (*v).max(reword);
            }
            Entry::Vacant(v) => {
                v.insert(0);
            }
        }
        // end condition
        if state.1 == 0 || state.2.len() == self.valves.len() {
            return 0;
        }
        // memory state
        // if let Some(&val) = self.f.get(&state) {
        //     return val;
        // }
        let mut ans = 0;
        let mut state_from = None;
        for to in 0..self.valves.len() {
            if self.dis[from][to] == u64::MAX
                || state.2.contains(&to)
                || self.dis[from][to] + 1 > t
                || self.valves[to].flow == 0
            {
                continue;
            }
            let mut new_set = state.2.clone();
            new_set.insert(to);
            let new_t = t - self.dis[from][to] - 1;
            let new_state = State(to, new_t, new_set);
            let val = self.dfs(new_state.clone(), reword + self.valves[to].flow * new_t);
            if ans < val {
                ans = val;
                state_from = Some(new_state);
            }
        }
        if state.2.contains(&from) {
            ans += self.valves[from].flow * t;
        }
        if let Some(state_from) = state_from {
            self.state_map.insert(state.clone(), state_from);
        }
        self.f.insert(state, ans);
        ans
    }
}

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();
    let mut valve_map = HashMap::new();
    let mut valves = Vec::new();
    for (i, line) in lines.enumerate() {
        let (_, valve) = Valve::parse(line.as_str()).unwrap();
        valve_map.insert(valve.name, i);
        valves.push(valve);
    }
    let n = valves.len();
    let mut dis = vec![vec![u64::MAX; n]; n];
    for i in 0..n {
        dis[i][i] = 0;
        for j in valves[i].links.iter().map(|&name| valve_map[&name]) {
            dis[i][j] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dis[i][k] != u64::MAX && dis[k][j] != u64::MAX {
                    dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
                }
            }
        }
    }

    let mut sol = Solution::new(valves.clone(), dis, valve_map);
    dbg!(sol.find_best(30));

    sol.clear();

    sol.find_best(26);
    let mut entries: Vec<_> = sol.opened_map.into_iter().collect();
    entries.sort_by_key(|e| std::cmp::Reverse(e.1));
    println!("{}", entries.len());

    let mut part2 = 0;
    let n = entries.len();
    for i in 0..n {
        for j in i + 1..n {
            if entries[i].0.is_disjoint(&entries[j].0) {
                part2 = part2.max(entries[i].1 + entries[j].1);
            }
        }
    }
    dbg!(part2);

    Ok(())
}
