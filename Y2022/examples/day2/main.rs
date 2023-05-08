use std::cmp::Ordering;
use std::io::{self, BufRead, Cursor};

// A for Rock, B for Paper, and C for Scissors
#[derive(PartialEq, Clone, Copy)]
enum Strategy {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Strategy {
    fn get_less(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn get_greater(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn get_equal(&self) -> Self {
        *self
    }
}

impl PartialOrd for Strategy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord = match self {
            Self::Rock => match other {
                Self::Paper => Ordering::Less,
                Self::Rock => Ordering::Equal,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Scissors => Ordering::Less,
                Self::Paper => Ordering::Equal,
                Self::Rock => Ordering::Greater,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Scissors => Ordering::Equal,
                Self::Paper => Ordering::Greater,
            },
        };
        Some(ord)
    }
}

impl From<u8> for Strategy {
    fn from(value: u8) -> Self {
        match value {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

fn main() -> io::Result<()> {
    let f = include_str!("./input.txt");

    let cursor = Cursor::new(f);
    let mut res = 0;
    for line in cursor.lines().map(|l| l.unwrap()) {
        if !line.is_empty() {
            let bytes = line.as_bytes();
            let first = Strategy::from(bytes[0]);
            let second = Strategy::from(bytes[2]);
            match second.partial_cmp(&first) {
                Some(Ordering::Less) => res += 0 + second as i32,
                Some(Ordering::Equal) => res += 3 + second as i32,
                Some(Ordering::Greater) => res += 6 + second as i32,
                None => unreachable!(),
            }
        }
    }
    dbg!(res);

    let cursor = Cursor::new(f);
    let mut res = 0;
    for line in cursor.lines().map(|l| l.unwrap()) {
        if !line.is_empty() {
            let bytes = line.as_bytes();
            let first = Strategy::from(bytes[0]);
            let second = bytes[2];
            match second {
                b'X' => res += 0 + first.get_less() as i32,
                b'Y' => res += 3 + first.get_equal() as i32,
                b'Z' => res += 6 + first.get_greater() as i32,
                _ => unreachable!(),
            }
        }
    }
    dbg!(res);

    Ok(())
}
