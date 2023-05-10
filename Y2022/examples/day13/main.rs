use anyhow::Result;
use serde_json::Value;
use std::{
    cmp::Ordering,
    io::{BufRead, Cursor},
    str::FromStr,
};

fn cmp_value(l: &Value, r: &Value) -> Ordering {
    match (l, r) {
        (Value::Array(la), Value::Array(ra)) => {
            let len_l = la.len();
            let len_r = ra.len();
            let len = len_l.min(len_r);
            for i in 0..len {
                let ord = cmp_value(&la[i], &ra[i]);
                if !Ordering::is_eq(ord) {
                    return ord;
                }
            }
            len_l.cmp(&len_r)
        }
        (Value::Number(ln), Value::Number(rn)) => ln.as_i64().unwrap().cmp(&rn.as_i64().unwrap()),
        (Value::Number(l_num), Value::Array(r_arr)) => {
            let l_val = Value::Array(vec![Value::Number(l_num.clone())]);
            let r_val = Value::Array(r_arr.clone());
            cmp_value(&l_val, &r_val)
        }
        (Value::Array(l_arr), Value::Number(r_num)) => {
            let l_val = Value::Array(l_arr.clone());
            let r_val = Value::Array(vec![Value::Number(r_num.clone())]);
            cmp_value(&l_val, &r_val)
        }
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Key(Value);

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(cmp_value(&self.0, &other.0))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_value(&self.0, &other.0)
    }
}

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");
    let cursor = Cursor::new(f);
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();

    let mut keys = Vec::new();

    let mut part1 = 0;
    let mut index = 1;
    while lines.peek().is_some() {
        let l = Value::from_str(&lines.next().unwrap()).unwrap();
        let r = Value::from_str(&lines.next().unwrap()).unwrap();

        if Ordering::is_lt(cmp_value(&l, &r)) {
            part1 += index;
        }

        keys.push(Key(l));
        keys.push(Key(r));

        // skip empty line
        lines.next();

        index += 1;
    }

    dbg!(part1);

    let key1 = Key(Value::from_str("[[2]]").unwrap());
    let key2 = Key(Value::from_str("[[6]]").unwrap());
    keys.push(key1.clone());
    keys.push(key2.clone());

    keys.sort();

    let p1 = keys.iter().position(|x| *x == key1).unwrap() + 1;
    let p2 = keys.iter().position(|x| *x == key2).unwrap() + 1;

    dbg!(p1 * p2);

    Ok(())
}
