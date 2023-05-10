use anyhow::Result;
use regex::Regex;
use std::{
    collections::HashSet,
    io::{BufRead, Cursor},
};

#[derive(Debug, Clone)]
enum Op {
    Calc(String, i64),
    Square,
}

impl Op {
    fn call(&self, level: i64) -> i64 {
        match self {
            Op::Calc(op, op_num) => {
                if op == "*" {
                    level * op_num
                } else {
                    level + op_num
                }
            }
            Op::Square => level * level,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    div: i64,
    op: Op,
    true_index: usize,
    false_index: usize,
}

fn main() -> Result<()> {
    let f = include_str!("./input.txt");
    let cursor = Cursor::new(f);
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let monkey_re = Regex::new(r"Monkey (\d+):").unwrap();
    let items_re = Regex::new(r"Starting items: (.+)").unwrap();
    let op_re = Regex::new(r"Operation: new = old (.) (\d+)").unwrap();
    let test_re = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let true_re = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let false_re = Regex::new(r"If false: throw to monkey (\d+)").unwrap();

    let mut monkeys = Vec::new();

    while lines.peek().is_some() {
        let _monkey = monkey_re
            .captures(&lines.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()?;
        let line = lines.next().unwrap();
        let items = items_re.captures(&line).unwrap().get(1).unwrap().as_str();
        let items: Vec<i64> = items
            .split(",")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        let line = lines.next().unwrap();
        let op = op_re.captures(&line).map_or_else(
            || Op::Square,
            |cap| {
                Op::Calc(
                    cap.get(1).unwrap().as_str().to_string(),
                    cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                )
            },
        );
        let div = test_re
            .captures(&lines.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i64>()?;
        let true_index = true_re
            .captures(&lines.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()?;
        let false_index = false_re
            .captures(&lines.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()?;
        // dbg!(monkey, items, op, op_num, div, true_monkey, false_monkey);

        monkeys.push(Monkey {
            items,
            div,
            op,
            true_index,
            false_index,
        });

        // skip empty line
        lines.next();
    }

    let part2_monkeys = monkeys.clone();

    let mut items_cnt: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            items_cnt[i] += items.len();
            monkeys[i].items.clear();
            for item in items {
                let mut level = item;
                level = monkeys[i].op.call(level);
                level /= 3;
                let index = if level % monkeys[i].div == 0 {
                    monkeys[i].true_index
                } else {
                    monkeys[i].false_index
                };
                monkeys[index].items.push(level);
            }
        }
        // dbg!(&monkeys);
    }

    dbg!(&items_cnt);
    items_cnt.sort();
    items_cnt.reverse();
    dbg!(items_cnt[0] * items_cnt[1]);

    let mut monkeys = part2_monkeys;
    let modu = monkeys.iter().fold(1, |acc, m| acc * m.div);

    let mut items_cnt: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            items_cnt[i] += items.len();
            monkeys[i].items.clear();
            for item in items {
                let mut level = item;
                level = monkeys[i].op.call(level) % modu;
                let index = if level % monkeys[i].div == 0 {
                    monkeys[i].true_index
                } else {
                    monkeys[i].false_index
                };
                monkeys[index].items.push(level);
            }
        }
        // dbg!(&monkeys);
    }

    dbg!(&items_cnt);
    items_cnt.sort();
    items_cnt.reverse();
    dbg!(items_cnt[0] * items_cnt[1]);

    Ok(())
}
