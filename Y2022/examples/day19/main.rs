use anyhow::Result;
use nom::sequence::tuple;
use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::map,
    sequence::{delimited, preceded},
    IResult,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, Cursor};
use std::ops::{Add, AddAssign};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Blueprint {
    ore: i64,
    clay: i64,
    obsidian: (i64, i64),
    geode: (i64, i64),
}

impl Blueprint {
    fn parse(input: &str) -> IResult<&str, Blueprint> {
        map(
            tuple((
                delimited(tag("Blueprint "), complete::i64, tag(": ")),
                delimited(tag("Each ore robot costs "), complete::i64, tag(" ore. ")),
                delimited(tag("Each clay robot costs "), complete::i64, tag(" ore. ")),
                tuple((
                    preceded(tag("Each obsidian robot costs "), complete::i64),
                    delimited(tag(" ore and "), complete::i64, tag(" clay. ")),
                )),
                tuple((
                    preceded(tag("Each geode robot costs "), complete::i64),
                    delimited(tag(" ore and "), complete::i64, tag(" obsidian.")),
                )),
            )),
            |(_, ore, clay, obsidian, geode)| Blueprint {
                ore,
                clay,
                obsidian,
                geode,
            },
        )(input)
    }

    fn max_ore_robots(&self) -> i64 {
        [self.ore, self.clay, self.obsidian.0, self.geode.0]
            .iter()
            .max()
            .unwrap()
            .to_owned()
    }

    fn max_clay_robots(&self) -> i64 {
        self.obsidian.1
    }

    fn max_obsidian_robots(&self) -> i64 {
        self.geode.1
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Resource {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl Add<Robot> for Resource {
    type Output = Resource;
    fn add(self, rhs: Robot) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign<Robot> for Resource {
    fn add_assign(&mut self, rhs: Robot) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    robot: Robot,
    resource: Resource,
    time: i64,
}

impl State {
    fn new(robot: Robot, resource: Resource, time: i64) -> Self {
        Self {
            robot,
            resource,
            time,
        }
    }
}

#[derive(Debug, Default)]
struct Solution {
    blueprint: Blueprint,
    state_map: HashMap<State, i64>,
    max_geode: i64,
}

impl Solution {
    fn new(blueprint: Blueprint) -> Self {
        Solution {
            blueprint,
            ..Default::default()
        }
    }

    fn dfs(&mut self, state: State) {
        if self.max_geode < state.resource.geode {
            self.max_geode = state.resource.geode;
        }

        if state.time == 0 {
            return;
        }
        let robot = state.robot;
        let resource = state.resource;
        // make robot
        // Only one robot can be produced per minute, there is no need to build robots after the maximum demand is met
        if robot.ore < self.blueprint.max_ore_robots() && resource.ore >= self.blueprint.ore {
            let mut new_state = state;
            new_state.robot.ore += 1;
            new_state.resource.ore -= self.blueprint.ore;
            new_state.resource += robot;
            new_state.time -= 1;
            if new_state.resource.geode
                + new_state.robot.geode * new_state.time
                + (new_state.time - 1) * new_state.time / 2
                >= self.max_geode
            {
                self.dfs(new_state);
            }
        }
        if robot.clay < self.blueprint.max_clay_robots() && resource.ore >= self.blueprint.clay {
            let mut new_state = state;
            new_state.robot.clay += 1;
            new_state.resource.ore -= self.blueprint.clay;
            new_state.resource += robot;
            new_state.time -= 1;
            if new_state.resource.geode
                + new_state.robot.geode * new_state.time
                + (new_state.time - 1) * new_state.time / 2
                >= self.max_geode
            {
                self.dfs(new_state);
            }
        }
        if robot.obsidian < self.blueprint.max_obsidian_robots()
            && resource.ore >= self.blueprint.obsidian.0
            && resource.clay >= self.blueprint.obsidian.1
        {
            let mut new_state = state;
            new_state.robot.obsidian += 1;
            new_state.resource.ore -= self.blueprint.obsidian.0;
            new_state.resource.clay -= self.blueprint.obsidian.1;
            new_state.resource += robot;
            new_state.time -= 1;
            if new_state.resource.geode
                + new_state.robot.geode * new_state.time
                + (new_state.time - 1) * new_state.time / 2
                >= self.max_geode
            {
                self.dfs(new_state);
            }
        }
        if resource.ore >= self.blueprint.geode.0 && resource.obsidian >= self.blueprint.geode.1 {
            let mut new_state = state;
            new_state.robot.geode += 1;
            new_state.resource.ore -= self.blueprint.geode.0;
            new_state.resource.obsidian -= self.blueprint.geode.1;
            new_state.resource += robot;
            new_state.time -= 1;
            if new_state.resource.geode
                + new_state.robot.geode * new_state.time
                + (new_state.time - 1) * new_state.time / 2
                >= self.max_geode
            {
                self.dfs(new_state);
            }
        }
        // only gen resource
        {
            let mut new_state = state;
            new_state.resource += robot;
            new_state.time -= 1;
            if new_state.resource.geode
                + new_state.robot.geode * new_state.time
                + (new_state.time - 1) * new_state.time / 2
                >= self.max_geode
            {
                self.dfs(new_state);
            }
        }
    }
}

fn part1(blueprints: Vec<Blueprint>) {
    let res: Arc<Mutex<Vec<(usize, i64)>>> = Arc::new(Mutex::new(Vec::new()));
    thread::scope(|s| {
        for (i, blueprint) in blueprints.into_iter().enumerate() {
            let res_ = res.clone();
            s.spawn(move || {
                let mut sol = Solution::new(blueprint);
                sol.dfs(State {
                    time: 24,
                    robot: Robot {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    ..Default::default()
                });
                dbg!(i + 1, sol.max_geode);
                res_.lock().unwrap().push((i + 1, sol.max_geode));
            });
        }
    });

    let ans = res.lock().unwrap().clone();
    let sum = ans.iter().fold(0, |pre, cur| pre + (cur.0 as i64 * cur.1));
    println!("{:?}", ans);
    println!("{}", sum);
}

fn part2(blueprints: Vec<Blueprint>) {
    let res: Arc<Mutex<Vec<(usize, i64)>>> = Arc::new(Mutex::new(Vec::new()));
    thread::scope(|s| {
        for (i, blueprint) in blueprints.into_iter().take(3).enumerate() {
            let res_ = res.clone();
            s.spawn(move || {
                let mut sol = Solution::new(blueprint);
                sol.dfs(State {
                    time: 32,
                    robot: Robot {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    ..Default::default()
                });
                dbg!(i + 1, sol.max_geode);
                res_.lock().unwrap().push((i + 1, sol.max_geode));
            });
        }
    });

    let ans = res.lock().unwrap().clone();
    let sum = ans.iter().fold(1, |pre, cur| pre * cur.1);
    println!("{:?}", ans);
    println!("{}", sum);
}

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut blueprints = Vec::new();
    for line in lines {
        let (_, blueprint) = Blueprint::parse(&line).unwrap();
        blueprints.push(blueprint);
    }

    // part1(blueprints.clone());
    part2(blueprints);
    // [examples/day19/main.rs:267] i + 1 = 3
    // [examples/day19/main.rs:267] sol.max_geode = 23
    // [examples/day19/main.rs:267] i + 1 = 2
    // [examples/day19/main.rs:267] sol.max_geode = 11
    // [examples/day19/main.rs:267] i + 1 = 1
    // [examples/day19/main.rs:267] sol.max_geode = 14
    // [(3, 23), (2, 11), (1, 14)]
    // 3542

    // ________________________________________________________
    // Executed in   20.68 mins    fish           external
    //    usr time   38.53 mins    0.21 millis   38.53 mins
    //    sys time    0.43 mins    1.81 millis    0.43 mins
    Ok(())
}
